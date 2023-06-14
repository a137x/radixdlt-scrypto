use super::{EpochChangeEvent, RoundChangeEvent, ValidatorCreator};
use crate::errors::ApplicationError;
use crate::errors::RuntimeError;
use crate::kernel::kernel_api::KernelNodeApi;
use crate::types::*;
use native_sdk::modules::access_rules::{AccessRules, AccessRulesObject, AttachedAccessRules};
use native_sdk::modules::metadata::Metadata;
use native_sdk::modules::royalty::ComponentRoyalty;
use native_sdk::resource::{NativeBucket, ResourceManager};
use native_sdk::runtime::Runtime;
use radix_engine_interface::api::field_lock_api::LockFlags;
use radix_engine_interface::api::node_modules::auth::AuthAddresses;
use radix_engine_interface::api::node_modules::metadata::MetadataValue;
use radix_engine_interface::api::object_api::ObjectModuleId;
use radix_engine_interface::api::{ClientApi, CollectionIndex, OBJECT_HANDLE_SELF};
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::rule;

const MILLIS_IN_SECOND: i64 = 1000;
const SECONDS_IN_MINUTE: i64 = 60;
const MILLIS_IN_MINUTE: i64 = MILLIS_IN_SECOND * SECONDS_IN_MINUTE;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct ConsensusManagerConfigSubstate {
    pub config: ConsensusManagerConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct ConsensusManagerSubstate {
    pub epoch: Epoch,
    pub epoch_start_milli: i64,
    pub round: Round,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, ScryptoSbor)]
pub struct Validator {
    pub key: EcdsaSecp256k1PublicKey,
    pub stake: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct CurrentValidatorSetSubstate {
    pub validator_set: ActiveValidatorSet,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
#[sbor(transparent)]
pub struct ActiveValidatorSet {
    /// The validators in the set, ordered by stake descending.
    pub validators_by_stake_desc: IndexMap<ComponentAddress, Validator>,
}

impl ActiveValidatorSet {
    pub fn get_by_index(&self, index: ValidatorIndex) -> Option<(&ComponentAddress, &Validator)> {
        self.validators_by_stake_desc.get_index(index as usize)
    }

    pub fn get_by_address(&self, address: &ComponentAddress) -> Option<&Validator> {
        self.validators_by_stake_desc.get(address)
    }

    /// Note for performance - this is calculated by iterating over the whole validator set.
    pub fn get_by_public_key(
        &self,
        public_key: &EcdsaSecp256k1PublicKey,
    ) -> Option<(&ComponentAddress, &Validator)> {
        self.validators_by_stake_desc
            .iter()
            .find(|(_, validator)| &validator.key == public_key)
    }

    /// Note for performance - this is calculated by iterating over the whole validator set.
    pub fn total_active_stake_xrd(&self) -> Decimal {
        self.validators_by_stake_desc
            .iter()
            .map(|(_, validator)| validator.stake)
            .sum()
    }

    pub fn validator_count(&self) -> usize {
        self.validators_by_stake_desc.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
#[sbor(transparent)]
pub struct ProposerMilliTimestampSubstate {
    /// A number of millis elapsed since epoch (i.e. a classic "epoch millis" timestamp).
    /// A signed number is traditionally used (for reasons like representing instants before A.D.
    /// 1970, which may not even apply in our case).
    pub epoch_milli: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
#[sbor(transparent)]
pub struct ProposerMinuteTimestampSubstate {
    /// A number of full minutes elapsed since epoch.
    /// A signed number is used for the same reasons as in [`ProposerMilliTimestampSubstate`], and
    /// gives us time until A.D. 5772.
    pub epoch_minute: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct CurrentProposalStatisticSubstate {
    /// A proposal statistic of each validator from the current validator set, in the iteration
    /// order of [`CurrentValidatorSetSubstate.validator_set`].
    pub validator_statistics: Vec<ProposalStatistic>,
}

impl CurrentProposalStatisticSubstate {
    /// Gets a mutable reference to a proposal statistic tracker of an individual validator.
    pub fn get_mut_proposal_statistic(
        &mut self,
        validator_index: ValidatorIndex,
    ) -> Result<&mut ProposalStatistic, RuntimeError> {
        let validator_count = self.validator_statistics.len();
        self.validator_statistics
            .get_mut(validator_index as usize)
            .ok_or_else(|| {
                RuntimeError::ApplicationError(ApplicationError::ConsensusManagerError(
                    ConsensusManagerError::InvalidValidatorIndex {
                        index: validator_index,
                        count: validator_count,
                    },
                ))
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ScryptoSbor)]
pub struct ProposalStatistic {
    /// A counter of successful proposals made by a specific validator.
    pub made: u64,
    /// A counter of missed proposals (caused both by gap rounds or fallback rounds).
    pub missed: u64,
}

impl ProposalStatistic {
    /// A ratio of successful to total proposals.
    /// There is a special case of a validator which did not have a chance of leading even a single
    /// round of consensus - currently we assume they should not be punished (i.e. we return `1.0`).
    pub fn success_ratio(&self) -> Decimal {
        let total = self.made + self.missed;
        if total == 0 {
            return Decimal::one();
        }
        Decimal::from(self.made) / Decimal::from(total)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Sbor)]
pub enum ConsensusManagerError {
    InvalidRoundUpdate {
        from: Round,
        to: Round,
    },
    InconsistentGapRounds {
        gap_rounds: usize,
        progressed_rounds: u64,
    },
    InvalidValidatorIndex {
        index: ValidatorIndex,
        count: usize,
    },
}

pub const CONSENSUS_MANAGER_REGISTERED_VALIDATORS_BY_STAKE_INDEX: CollectionIndex = 0u8;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct EpochRegisteredValidatorByStakeEntry {
    pub component_address: ComponentAddress,
    pub validator: Validator,
}

pub struct ConsensusManagerBlueprint;

impl ConsensusManagerBlueprint {
    pub(crate) fn create<Y>(
        validator_token_address: [u8; NodeId::LENGTH], // TODO: Clean this up
        component_address: [u8; NodeId::LENGTH],       // TODO: Clean this up
        initial_epoch: Epoch,
        initial_config: ConsensusManagerConfig,
        initial_time_milli: i64,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let address = ComponentAddress::new_or_panic(component_address);

        {
            let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
            let mut access_rules = BTreeMap::new();

            // TODO: remove mint and premint all tokens
            {
                let global_id =
                    NonFungibleGlobalId::package_of_direct_caller_badge(CONSENSUS_MANAGER_PACKAGE);
                access_rules.insert(Mint, (rule!(require(global_id)), rule!(deny_all)));
            }

            access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));

            ResourceManager::new_non_fungible_with_address::<(), Y, RuntimeError>(
                NonFungibleIdType::UUID,
                metadata,
                access_rules,
                validator_token_address,
                api,
            )?;
        };

        let consensus_manager_id = {
            let config = ConsensusManagerConfigSubstate {
                config: initial_config,
            };
            let consensus_manager = ConsensusManagerSubstate {
                epoch: initial_epoch,
                epoch_start_milli: initial_time_milli,
                round: Round::zero(),
            };
            let current_validator_set = CurrentValidatorSetSubstate {
                validator_set: ActiveValidatorSet {
                    validators_by_stake_desc: index_map_new(),
                },
            };
            let current_proposal_statistic = CurrentProposalStatisticSubstate {
                validator_statistics: Vec::new(),
            };
            let minute_timestamp = ProposerMinuteTimestampSubstate {
                epoch_minute: Self::milli_to_minute(initial_time_milli),
            };
            let milli_timestamp = ProposerMilliTimestampSubstate {
                epoch_milli: initial_time_milli,
            };

            api.new_simple_object(
                CONSENSUS_MANAGER_BLUEPRINT,
                vec![
                    scrypto_encode(&config).unwrap(),
                    scrypto_encode(&consensus_manager).unwrap(),
                    scrypto_encode(&current_validator_set).unwrap(),
                    scrypto_encode(&current_proposal_statistic).unwrap(),
                    scrypto_encode(&minute_timestamp).unwrap(),
                    scrypto_encode(&milli_timestamp).unwrap(),
                ],
            )?
        };

        let mut authority_rules = AuthorityRules::new();
        authority_rules.set_main_authority_rule(
            CONSENSUS_MANAGER_START_IDENT,
            rule!(require(package_of_direct_caller(CONSENSUS_MANAGER_PACKAGE))),
            rule!(require(package_of_direct_caller(CONSENSUS_MANAGER_PACKAGE))),
        );
        authority_rules.set_fixed_main_authority_rule(
            CONSENSUS_MANAGER_NEXT_ROUND_IDENT,
            rule!(require(AuthAddresses::validator_role())),
        );

        let access_rules = AccessRules::create(
            authority_rules,
            btreemap!(
                VALIDATOR_BLUEPRINT.to_string() => AuthorityRules::new()
            ),
            api,
        )?
        .0;
        let metadata = Metadata::create(api)?;
        let royalty = ComponentRoyalty::create(RoyaltyConfig::default(), api)?;

        api.globalize_with_address(
            btreemap!(
                ObjectModuleId::Main => consensus_manager_id,
                ObjectModuleId::AccessRules => access_rules.0,
                ObjectModuleId::Metadata => metadata.0,
                ObjectModuleId::Royalty => royalty.0,
            ),
            address.into(),
        )?;

        Ok(())
    }

    pub(crate) fn get_current_epoch<Y>(api: &mut Y) -> Result<Epoch, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::ConsensusManager.into(),
            LockFlags::read_only(),
        )?;

        let consensus_manager: ConsensusManagerSubstate = api.field_lock_read_typed(handle)?;

        Ok(consensus_manager.epoch)
    }

    pub(crate) fn start<Y>(receiver: &NodeId, api: &mut Y) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let config_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::Config.into(),
            LockFlags::read_only(),
        )?;
        let config_substate: ConsensusManagerConfigSubstate =
            api.field_lock_read_typed(config_handle)?;
        api.field_lock_release(config_handle)?;

        let manager_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::ConsensusManager.into(),
            LockFlags::read_only(),
        )?;
        let manager_substate: ConsensusManagerSubstate =
            api.field_lock_read_typed(manager_handle)?;
        api.field_lock_release(manager_handle)?;

        Self::epoch_change(manager_substate.epoch, &config_substate.config, api)?;

        let access_rules = AttachedAccessRules(*receiver);
        access_rules.set_authority_rule_and_mutability(
            AuthorityKey::main("start"),
            AccessRule::DenyAll,
            AccessRule::DenyAll,
            api,
        )?;

        Ok(())
    }

    pub(crate) fn get_current_time<Y>(
        precision: TimePrecision,
        api: &mut Y,
    ) -> Result<Instant, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        match precision {
            TimePrecision::Minute => {
                let handle = api.actor_lock_field(
                    OBJECT_HANDLE_SELF,
                    ConsensusManagerField::CurrentTimeRoundedToMinutes.into(),
                    LockFlags::read_only(),
                )?;
                let proposer_minute_timestamp: ProposerMinuteTimestampSubstate =
                    api.field_lock_read_typed(handle)?;
                api.field_lock_release(handle)?;

                Ok(Self::epoch_minute_to_instant(
                    proposer_minute_timestamp.epoch_minute,
                ))
            }
        }
    }

    pub(crate) fn compare_current_time<Y>(
        other_arbitrary_precision_instant: Instant,
        precision: TimePrecision,
        operator: TimeComparisonOperator,
        api: &mut Y,
    ) -> Result<bool, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        match precision {
            TimePrecision::Minute => {
                let other_epoch_minute = Self::milli_to_minute(
                    other_arbitrary_precision_instant.seconds_since_unix_epoch * MILLIS_IN_SECOND,
                );

                let handle = api.actor_lock_field(
                    OBJECT_HANDLE_SELF,
                    ConsensusManagerField::CurrentTimeRoundedToMinutes.into(),
                    LockFlags::read_only(),
                )?;
                let proposer_minute_timestamp: ProposerMinuteTimestampSubstate =
                    api.field_lock_read_typed(handle)?;
                api.field_lock_release(handle)?;

                // convert back to Instant only for comparison operation
                let proposer_instant =
                    Self::epoch_minute_to_instant(proposer_minute_timestamp.epoch_minute);
                let other_instant = Self::epoch_minute_to_instant(other_epoch_minute);
                let result = proposer_instant.compare(other_instant, operator);
                Ok(result)
            }
        }
    }

    fn epoch_minute_to_instant(epoch_minute: i32) -> Instant {
        Instant::new(epoch_minute as i64 * SECONDS_IN_MINUTE)
    }

    fn milli_to_minute(epoch_milli: i64) -> i32 {
        i32::try_from(epoch_milli / MILLIS_IN_MINUTE).unwrap() // safe until A.D. 5700
    }

    pub(crate) fn next_round<Y>(
        round: Round,
        proposer_timestamp_milli: i64,
        proposal_history: LeaderProposalHistory,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let config_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::Config.into(),
            LockFlags::read_only(),
        )?;
        let config_substate: ConsensusManagerConfigSubstate =
            api.field_lock_read_typed(config_handle)?;
        api.field_lock_release(config_handle)?;

        let manager_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::ConsensusManager.into(),
            LockFlags::MUTABLE,
        )?;
        let mut manager_substate: ConsensusManagerSubstate =
            api.field_lock_read_typed(manager_handle)?;

        let progressed_rounds = Round::calculate_progress(manager_substate.round, round)
            .ok_or_else(|| {
                RuntimeError::ApplicationError(ApplicationError::ConsensusManagerError(
                    ConsensusManagerError::InvalidRoundUpdate {
                        from: manager_substate.round,
                        to: round,
                    },
                ))
            })?;

        Self::update_proposal_statistics(progressed_rounds, proposal_history, api)?;

        let config = &config_substate.config;
        let epoch_duration_millis = proposer_timestamp_milli - manager_substate.epoch_start_milli;
        if config
            .epoch_change_condition
            .is_met(epoch_duration_millis, round)
        {
            let next_epoch = manager_substate.epoch.next();
            Self::epoch_change(next_epoch, config, api)?;
            manager_substate.epoch = next_epoch;
            manager_substate.epoch_start_milli = proposer_timestamp_milli;
            manager_substate.round = Round::zero();
        } else {
            Runtime::emit_event(api, RoundChangeEvent { round })?;
            manager_substate.round = round;
        }

        api.field_lock_write_typed(manager_handle, &manager_substate)?;
        api.field_lock_release(manager_handle)?;

        Self::update_timestamps(proposer_timestamp_milli, api)?;

        Ok(())
    }

    pub(crate) fn create_validator<Y>(
        key: EcdsaSecp256k1PublicKey,
        fee_factor: Decimal,
        api: &mut Y,
    ) -> Result<(ComponentAddress, Bucket), RuntimeError>
    where
        Y: KernelNodeApi + ClientApi<RuntimeError>,
    {
        let (validator_address, owner_token_bucket) =
            ValidatorCreator::create(key, false, fee_factor, api)?;

        Ok((validator_address, owner_token_bucket))
    }

    fn update_timestamps<Y>(current_time_ms: i64, api: &mut Y) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let proposer_milli_timestamp = ProposerMilliTimestampSubstate {
            epoch_milli: current_time_ms,
        };
        let handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::CurrentTime.into(),
            LockFlags::MUTABLE,
        )?;
        api.field_lock_write_typed(handle, &proposer_milli_timestamp)?;
        api.field_lock_release(handle)?;

        let proposer_minute_timestamp = ProposerMinuteTimestampSubstate {
            epoch_minute: Self::milli_to_minute(current_time_ms),
        };
        let handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::CurrentTimeRoundedToMinutes.into(),
            LockFlags::MUTABLE,
        )?;
        api.field_lock_write_typed(handle, &proposer_minute_timestamp)?;
        api.field_lock_release(handle)?;

        Ok(())
    }

    fn update_proposal_statistics<Y>(
        progressed_rounds: u64,
        proposal_history: LeaderProposalHistory,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        if proposal_history.gap_round_leaders.len() as u64 != progressed_rounds - 1 {
            return Err(RuntimeError::ApplicationError(
                ApplicationError::ConsensusManagerError(
                    ConsensusManagerError::InconsistentGapRounds {
                        gap_rounds: proposal_history.gap_round_leaders.len(),
                        progressed_rounds,
                    },
                ),
            ));
        }

        let statistic_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::CurrentProposalStatistic.into(),
            LockFlags::MUTABLE,
        )?;
        let mut statistic: CurrentProposalStatisticSubstate =
            api.field_lock_read_typed(statistic_handle)?;
        for gap_round_leader in proposal_history.gap_round_leaders {
            let mut gap_round_statistic = statistic.get_mut_proposal_statistic(gap_round_leader)?;
            gap_round_statistic.missed += 1;
        }
        let mut current_round_statistic =
            statistic.get_mut_proposal_statistic(proposal_history.current_leader)?;
        if proposal_history.is_fallback {
            current_round_statistic.missed += 1;
        } else {
            current_round_statistic.made += 1;
        }
        api.field_lock_write_typed(statistic_handle, statistic)?;
        api.field_lock_release(statistic_handle)?;

        Ok(())
    }

    fn epoch_change<Y>(
        next_epoch: Epoch,
        config: &ConsensusManagerConfig,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Read previous validator set
        let validator_set_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::CurrentValidatorSet.into(),
            LockFlags::MUTABLE,
        )?;
        let mut validator_set_substate: CurrentValidatorSetSubstate =
            api.field_lock_read_typed(validator_set_handle)?;
        let previous_validator_set = validator_set_substate.validator_set;

        // Read previous validator statistics
        let statistic_handle = api.actor_lock_field(
            OBJECT_HANDLE_SELF,
            ConsensusManagerField::CurrentProposalStatistic.into(),
            LockFlags::MUTABLE,
        )?;
        let mut statistic_substate: CurrentProposalStatisticSubstate =
            api.field_lock_read_typed(statistic_handle)?;
        let previous_statistics = statistic_substate.validator_statistics;

        // Apply emissions
        Self::apply_validator_emissions(
            previous_validator_set,
            previous_statistics,
            config,
            next_epoch.previous(),
            api,
        )?;

        // Select next validator set
        // NOTE - because the stake index is by u16 buckets, it's possible that there are multiple validators at the cut off point
        // that fall into the same bucket.
        // To reduce the risk of that causing issues, we take a decent chunk more than we need from the index.
        // It's still possible that the bucket is _very_ large and we miss some validators in the bucket, and fail to read validators
        // with a higher stake, but lower DbSortKey.
        // The risk is very low though in practice, and only affects validators near the bottom of the list who would likely get very
        // few proposals, so we feel it's an okay trade-off.
        let num_validators_to_read_from_store =
            config.max_validators + (config.max_validators / 10) + 10;

        let mut top_registered_validators: Vec<EpochRegisteredValidatorByStakeEntry> = api
            .actor_sorted_index_scan_typed(
                OBJECT_HANDLE_SELF,
                CONSENSUS_MANAGER_REGISTERED_VALIDATORS_BY_STAKE_INDEX,
                num_validators_to_read_from_store,
            )?;

        // The index scan should already pull the validators out in stake DESC, but if multiple validators are on the same u16 stake,
        // then let's be even more accurate here. This sort is stable, so if two validators tie, then the resultant order will be
        // decided on sort key DESC.
        top_registered_validators.sort_by(|validator_1, validator_2| {
            validator_1
                .validator
                .stake
                .cmp(&validator_2.validator.stake)
                .reverse()
        });

        let next_active_validator_set = ActiveValidatorSet {
            validators_by_stake_desc: top_registered_validators
                .into_iter()
                .take(config.max_validators as usize)
                .map(|entry| (entry.component_address, entry.validator))
                .collect(),
        };

        // Emit epoch change event
        Runtime::emit_event(
            api,
            EpochChangeEvent {
                epoch: next_epoch,
                validator_set: next_active_validator_set.clone(),
            },
        )?;

        // Write zeroed statistics of next validators
        statistic_substate.validator_statistics = (0..next_active_validator_set.validator_count())
            .map(|_index| ProposalStatistic::default())
            .collect();
        api.field_lock_write_typed(statistic_handle, statistic_substate)?;
        api.field_lock_release(statistic_handle)?;

        // Write next validator set
        validator_set_substate.validator_set = next_active_validator_set;
        api.field_lock_write_typed(validator_set_handle, validator_set_substate)?;
        api.field_lock_release(validator_set_handle)?;

        Ok(())
    }

    /// Emits a configured XRD amount ([`ConsensusManagerConfigSubstate.total_emission_xrd_per_epoch`])
    /// and distributes it across the given validator set, according to their stake.
    fn apply_validator_emissions<Y>(
        validator_set: ActiveValidatorSet,
        validator_statistics: Vec<ProposalStatistic>,
        config: &ConsensusManagerConfig,
        epoch: Epoch, // the concluded epoch, for event creation
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let validator_emissions = validator_set
            .validators_by_stake_desc
            .into_iter()
            .zip(validator_statistics)
            .filter_map(|((address, validator), statistic)| {
                ValidatorEmission::create_if_applicable(
                    address,
                    validator.stake,
                    statistic,
                    config.min_validator_reliability,
                )
            })
            .collect::<Vec<_>>();

        if validator_emissions.is_empty() {
            return Ok(());
        }

        let stake_sum_xrd = validator_emissions
            .iter()
            .map(|validator_emission| validator_emission.stake_xrd)
            .sum::<Decimal>();
        // calculate "how much XRD is emitted by 1 XRD staked", and later apply it evenly among validators
        // (the gains are slightly rounded down, but more fairly distributed - not affected by different rounding errors for different validators)
        let emission_per_staked_xrd = config.total_emission_xrd_per_epoch / stake_sum_xrd;
        let effective_total_emission_xrd = validator_emissions
            .iter()
            .map(|validator_emission| {
                validator_emission.effective_stake_xrd * emission_per_staked_xrd
            })
            .sum::<Decimal>();

        let total_emission_xrd_bucket =
            ResourceManager(RADIX_TOKEN).mint_fungible(effective_total_emission_xrd, api)?;

        for validator_emission in validator_emissions {
            let emission_xrd_bucket = total_emission_xrd_bucket.take(
                validator_emission.effective_stake_xrd * emission_per_staked_xrd,
                api,
            )?;
            api.call_method(
                validator_emission.address.as_node_id(),
                VALIDATOR_APPLY_EMISSION_IDENT,
                scrypto_encode(&ValidatorApplyEmissionInput {
                    xrd_bucket: emission_xrd_bucket,
                    epoch,
                    proposals_made: validator_emission.proposal_statistic.made,
                    proposals_missed: validator_emission.proposal_statistic.missed,
                })
                .unwrap(),
            )?;
        }
        total_emission_xrd_bucket.drop_empty(api)?;

        Ok(())
    }
}

#[derive(Debug)]
struct ValidatorEmission {
    pub address: ComponentAddress,
    pub stake_xrd: Decimal,
    pub effective_stake_xrd: Decimal,
    pub proposal_statistic: ProposalStatistic, // needed only for passing the information to event
}

impl ValidatorEmission {
    fn create_if_applicable(
        address: ComponentAddress,
        stake_xrd: Decimal,
        proposal_statistic: ProposalStatistic,
        min_required_reliability: Decimal,
    ) -> Option<Self> {
        if stake_xrd.is_positive() {
            let reliability_factor = Self::to_reliability_factor(
                proposal_statistic.success_ratio(),
                min_required_reliability,
            );
            let effective_stake_xrd = stake_xrd * reliability_factor;
            Some(Self {
                address,
                stake_xrd,
                proposal_statistic,
                effective_stake_xrd,
            })
        } else {
            None
        }
    }

    /// Converts the absolute reliability measure (e.g. "0.97 uptime") into a reliability factor
    /// which directly drives the fraction of received emission (e.g. "0.25 of base emission"), by
    /// rescaling it into the allowed reliability range (e.g. "required >0.96 uptime").
    fn to_reliability_factor(reliability: Decimal, min_required_reliability: Decimal) -> Decimal {
        let reliability_reserve = reliability - min_required_reliability;
        if reliability_reserve.is_negative() {
            return Decimal::zero();
        }
        let max_allowed_unreliability = Decimal::one() - min_required_reliability;
        if max_allowed_unreliability.is_zero() {
            // special-casing the dirac delta behavior
            if reliability == Decimal::one() {
                return Decimal::one();
            } else {
                return Decimal::zero();
            }
        }
        reliability_reserve / max_allowed_unreliability
    }
}
