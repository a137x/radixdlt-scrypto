use crate::blueprints::consensus_manager::{ConsensusManagerSubstate, ValidatorRewardsSubstate};
use crate::blueprints::package::{
    PackageBlueprintVersionDefinitionEntrySubstate, PackagePartition,
};
use crate::blueprints::resource::BurnFungibleResourceEvent;
use crate::blueprints::transaction_processor::TransactionProcessorError;
use crate::blueprints::transaction_tracker::{TransactionStatus, TransactionTrackerSubstate};
use crate::errors::*;
use crate::kernel::id_allocator::IdAllocator;
use crate::kernel::kernel::KernelBoot;
use crate::system::system::{FieldSubstate, KeyValueEntrySubstate, SubstateMutability};
use crate::system::system_callback::SystemConfig;
use crate::system::system_callback_api::SystemCallbackObject;
use crate::system::system_modules::costing::*;
use crate::system::system_modules::execution_trace::ExecutionTraceModule;
use crate::system::system_modules::transaction_runtime::TransactionRuntimeModule;
use crate::system::system_modules::{EnabledModules, SystemModuleMixer};
use crate::track::interface::SubstateStore;
use crate::track::{to_state_updates, Track};
use crate::transaction::*;
use crate::types::*;
use radix_engine_common::constants::*;
use radix_engine_interface::api::ObjectModuleId;
use radix_engine_interface::blueprints::package::BlueprintVersionKey;
use radix_engine_interface::blueprints::resource::LiquidFungibleResource;
use radix_engine_interface::blueprints::transaction_processor::InstructionOutput;
use radix_engine_store_interface::{db_key_mapper::SpreadPrefixKeyMapper, interface::*};
use transaction::model::*;

#[derive(Debug, Clone)]
pub struct FeeReserveConfig {
    pub cost_unit_price: Decimal,
    pub usd_price: Decimal,
    pub state_expansion_price: Decimal,
    pub system_loan: u32,
}

impl Default for FeeReserveConfig {
    fn default() -> Self {
        Self {
            cost_unit_price: COST_UNIT_PRICE_IN_XRD.try_into().unwrap(),
            usd_price: USD_PRICE_IN_XRD.try_into().unwrap(),
            state_expansion_price: STATE_EXPANSION_PRICE_IN_XRD.try_into().unwrap(),
            system_loan: SYSTEM_LOAN_AMOUNT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub enabled_modules: EnabledModules,
    pub max_execution_trace_depth: usize,
    pub max_call_depth: usize,
    pub cost_unit_limit: u32,
    pub abort_when_loan_repaid: bool,
    pub max_number_of_substates_in_track: usize,
    pub max_number_of_substates_in_heap: usize,
    pub max_substate_key_size: usize,
    pub max_substate_size: usize,
    pub max_invoke_input_size: usize,
    pub enable_cost_breakdown: bool,
    pub max_event_size: usize,
    pub max_log_size: usize,
    pub max_panic_message_size: usize,
    pub max_number_of_logs: usize,
    pub max_number_of_events: usize,
    pub max_per_function_royalty_in_xrd: Decimal,
}

impl ExecutionConfig {
    /// Creates an `ExecutionConfig` using default configurations.
    /// This is internal. Clients should use `for_xxx` constructors instead.
    fn default() -> Self {
        Self {
            enabled_modules: EnabledModules::for_notarized_transaction(),
            max_execution_trace_depth: MAX_EXECUTION_TRACE_DEPTH,
            max_call_depth: MAX_CALL_DEPTH,
            cost_unit_limit: COST_UNIT_LIMIT,
            abort_when_loan_repaid: false,
            max_number_of_substates_in_track: MAX_NUMBER_OF_SUBSTATES_IN_TRACK,
            max_number_of_substates_in_heap: MAX_NUMBER_OF_SUBSTATES_IN_HEAP,
            max_substate_key_size: MAX_SUBSTATE_KEY_SIZE,
            max_substate_size: MAX_SUBSTATE_SIZE,
            max_invoke_input_size: MAX_INVOKE_PAYLOAD_SIZE,
            enable_cost_breakdown: false,
            max_event_size: MAX_EVENT_SIZE,
            max_log_size: MAX_LOG_SIZE,
            max_panic_message_size: MAX_PANIC_MESSAGE_SIZE,
            max_number_of_logs: MAX_NUMBER_OF_LOGS,
            max_number_of_events: MAX_NUMBER_OF_EVENTS,
            max_per_function_royalty_in_xrd: Decimal::try_from(MAX_PER_FUNCTION_ROYALTY_IN_XRD)
                .unwrap(),
        }
    }

    pub fn for_genesis_transaction() -> Self {
        Self {
            enabled_modules: EnabledModules::for_genesis_transaction(),
            max_number_of_substates_in_track: 50_000,
            max_number_of_substates_in_heap: 50_000,
            max_number_of_events: 1_000_000,
            ..Self::default()
        }
    }

    pub fn for_system_transaction() -> Self {
        Self {
            enabled_modules: EnabledModules::for_system_transaction(),
            ..Self::default()
        }
    }

    pub fn for_notarized_transaction() -> Self {
        Self {
            enabled_modules: EnabledModules::for_notarized_transaction(),
            ..Self::default()
        }
    }

    pub fn for_test_transaction() -> Self {
        Self {
            enabled_modules: EnabledModules::for_test_transaction(),
            enable_cost_breakdown: true,
            ..Self::default()
        }
    }

    pub fn for_preview() -> Self {
        Self {
            enabled_modules: EnabledModules::for_preview(),
            enable_cost_breakdown: true,
            ..Self::default()
        }
    }

    pub fn with_kernel_trace(mut self, enabled: bool) -> Self {
        if enabled {
            self.enabled_modules.insert(EnabledModules::KERNEL_TRACE);
        } else {
            self.enabled_modules.remove(EnabledModules::KERNEL_TRACE);
        }
        self
    }

    pub fn with_cost_breakdown(mut self, enabled: bool) -> Self {
        self.enable_cost_breakdown = enabled;
        self
    }

    pub fn with_cost_unit_limit(mut self, cost_unit_limit: u32) -> Self {
        self.cost_unit_limit = cost_unit_limit;
        self
    }

    pub fn up_to_loan_repayment(mut self, enabled: bool) -> Self {
        self.abort_when_loan_repaid = enabled;
        self
    }
}

/// An executor that runs transactions.
/// This is no longer public -- it can be removed / merged into the exposed functions in a future small PR
/// But I'm not doing it in this PR to avoid merge conflicts in the body of execute_with_fee_reserve
struct TransactionExecutor<'s, S, V: SystemCallbackObject + Clone>
where
    S: SubstateDatabase,
{
    substate_db: &'s S,
    vm: V,
}

impl<'s, S, V> TransactionExecutor<'s, S, V>
where
    S: SubstateDatabase,
    V: SystemCallbackObject + Clone,
{
    pub fn new(substate_db: &'s S, vm: V) -> Self {
        Self { substate_db, vm }
    }

    pub fn execute(
        &mut self,
        executable: &Executable,
        fee_reserve_config: &FeeReserveConfig,
        execution_config: &ExecutionConfig,
    ) -> TransactionReceipt {
        let free_credit = executable.fee_payment().free_credit_in_xrd;
        let tip_percentage = executable.fee_payment().tip_percentage;
        let fee_reserve = SystemLoanFeeReserve::new(
            fee_reserve_config.cost_unit_price,
            fee_reserve_config.usd_price,
            fee_reserve_config.state_expansion_price,
            tip_percentage,
            execution_config.cost_unit_limit,
            fee_reserve_config.system_loan,
            execution_config.abort_when_loan_repaid,
        )
        .with_free_credit(free_credit);
        let fee_table = FeeTable::new();

        // Dump executable
        #[cfg(not(feature = "alloc"))]
        if execution_config
            .enabled_modules
            .contains(EnabledModules::KERNEL_TRACE)
        {
            Self::print_executable(&executable);
        }

        // Start hardware resource usage tracker
        #[cfg(all(target_os = "linux", feature = "std", feature = "cpu_ram_metrics"))]
        let mut resources_tracker =
            crate::kernel::resources_tracker::ResourcesTracker::start_measurement();

        // Create a track
        let mut track = Track::<_, SpreadPrefixKeyMapper>::new(self.substate_db);

        // Perform runtime validation.
        // TODO: the following assumptions can be removed with better interface.
        // We are assuming that intent hash store is ready when epoch manager is ready.
        let current_epoch = Self::read_epoch(&mut track);
        let validation_result = if let Some(current_epoch) = current_epoch {
            if let Some(range) = executable.epoch_range() {
                Self::validate_epoch_range(
                    current_epoch,
                    range.start_epoch_inclusive,
                    range.end_epoch_exclusive,
                )
                .and_then(|_| {
                    Self::validate_intent_hash(
                        &mut track,
                        executable.intent_hash().to_hash(),
                        range.end_epoch_exclusive,
                    )
                })
            } else {
                Ok(())
            }
        } else {
            Ok(())
        };

        // Run manifest
        let result = match validation_result {
            Ok(()) => {
                let (
                    interpretation_result,
                    (mut costing_module, runtime_module, execution_trace_module),
                ) = self.interpret_manifest(
                    &mut track,
                    executable,
                    execution_config,
                    fee_reserve,
                    fee_table,
                );

                #[cfg(not(feature = "alloc"))]
                if execution_config
                    .enabled_modules
                    .contains(EnabledModules::KERNEL_TRACE)
                {
                    println!("{:-^100}", "Interpretation Results");
                    println!("{:?}", interpretation_result);
                }

                let result_type = Self::determine_result_type(
                    interpretation_result,
                    &mut costing_module.fee_reserve,
                );
                match result_type {
                    TransactionResultType::Commit(outcome) => {
                        let is_success = outcome.is_ok();

                        // Commit/revert
                        if !is_success {
                            costing_module.fee_reserve.revert_royalty();
                            track.revert_non_force_write_changes();
                        }

                        // Distribute fees
                        let (mut fee_summary, fee_payments) = Self::finalize_fees(
                            &mut track,
                            costing_module.fee_reserve,
                            is_success,
                            free_credit,
                        );
                        fee_summary.execution_cost_breakdown = costing_module
                            .costing_traces
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v))
                            .collect();
                        fee_summary.fee_payments = fee_payments.clone();

                        // Update intent hash status
                        if let Some(next_epoch) = Self::read_epoch(&mut track) {
                            Self::update_transaction_tracker(
                                &mut track,
                                next_epoch,
                                executable.intent_hash(),
                                is_success,
                            );
                        }

                        // Finalize events and logs
                        let (mut application_events, application_logs) =
                            runtime_module.finalize(is_success);
                        /*
                            Emit XRD burn event, ignoring costing and limits.
                            Otherwise, we won't be able to commit failed transactions.
                            May also cache the information for better performance.
                        */
                        let substate = track
                            .read_substate(
                                RESOURCE_PACKAGE.as_node_id(),
                                PackagePartition::BlueprintVersionDefinitionKeyValue
                                    .as_main_partition(),
                                &SubstateKey::Map(
                                    scrypto_encode(&BlueprintVersionKey::new_default(
                                        FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                                    ))
                                    .unwrap(),
                                ),
                            )
                            .unwrap();
                        let substate: PackageBlueprintVersionDefinitionEntrySubstate =
                            substate.as_typed().unwrap();
                        let type_pointer = substate
                            .value
                            .unwrap()
                            .0
                            .into_latest()
                            .interface
                            .get_event_type_pointer("BurnFungibleResourceEvent")
                            .unwrap();
                        application_events.push((
                            EventTypeIdentifier(
                                Emitter::Method(XRD.into_node_id(), ObjectModuleId::Main),
                                type_pointer,
                            ),
                            scrypto_encode(&BurnFungibleResourceEvent {
                                amount: fee_summary.to_burn_amount(),
                            })
                            .unwrap(),
                        ));

                        // Finalize execution trace
                        let execution_trace =
                            execution_trace_module.finalize(&fee_payments, is_success);

                        // Finalize track
                        let (tracked_nodes, deleted_partitions) = track.finalize();

                        let system_structure = SystemStructure::resolve(
                            self.substate_db,
                            &tracked_nodes,
                            &application_events,
                        );

                        let state_update_summary =
                            StateUpdateSummary::new(self.substate_db, &tracked_nodes);
                        let state_updates = to_state_updates::<SpreadPrefixKeyMapper>(
                            tracked_nodes,
                            deleted_partitions,
                        );

                        TransactionResult::Commit(CommitResult {
                            state_updates,
                            state_update_summary,
                            outcome: match outcome {
                                Ok(o) => TransactionOutcome::Success(o),
                                Err(e) => TransactionOutcome::Failure(e),
                            },
                            fee_summary,
                            application_events,
                            application_logs,
                            system_structure,
                            execution_trace,
                        })
                    }
                    TransactionResultType::Reject(error) => {
                        TransactionResult::Reject(RejectResult { error })
                    }
                    TransactionResultType::Abort(error) => {
                        TransactionResult::Abort(AbortResult { reason: error })
                    }
                }
            }
            Err(error) => TransactionResult::Reject(RejectResult { error }),
        };

        // Stop hardware resource usage tracker
        let resources_usage = match () {
            #[cfg(not(all(target_os = "linux", feature = "std", feature = "cpu_ram_metrics")))]
            () => ResourcesUsage::default(),
            #[cfg(all(target_os = "linux", feature = "std", feature = "cpu_ram_metrics"))]
            () => resources_tracker.end_measurement(),
        };

        // Produce final receipt
        let receipt = TransactionReceipt {
            transaction_result: result,
            resources_usage,
        };

        // Dump summary
        #[cfg(not(feature = "alloc"))]
        if execution_config
            .enabled_modules
            .contains(EnabledModules::KERNEL_TRACE)
        {
            Self::print_execution_summary(&receipt);
        }

        receipt
    }

    fn read_epoch(track: &mut Track<S, SpreadPrefixKeyMapper>) -> Option<Epoch> {
        // TODO - Instead of doing a check of the exact epoch, we could do a check in range [X, Y]
        //        Which could allow for better caching of transaction validity over epoch boundaries
        match track.read_substate(
            CONSENSUS_MANAGER.as_node_id(),
            MAIN_BASE_PARTITION,
            &ConsensusManagerField::ConsensusManager.into(),
        ) {
            Some(x) => {
                let substate: FieldSubstate<ConsensusManagerSubstate> = x.as_typed().unwrap();
                Some(substate.value.0.epoch)
            }
            None => None,
        }
    }

    fn validate_epoch_range(
        current_epoch: Epoch,
        start_epoch_inclusive: Epoch,
        end_epoch_exclusive: Epoch,
    ) -> Result<(), RejectionError> {
        if current_epoch < start_epoch_inclusive {
            return Err(RejectionError::TransactionEpochNotYetValid {
                valid_from: start_epoch_inclusive,
                current_epoch,
            });
        }
        if current_epoch >= end_epoch_exclusive {
            return Err(RejectionError::TransactionEpochNoLongerValid {
                valid_until: end_epoch_exclusive.previous(),
                current_epoch,
            });
        }

        Ok(())
    }

    fn validate_intent_hash(
        track: &mut Track<S, SpreadPrefixKeyMapper>,
        intent_hash: Hash,
        expiry_epoch: Epoch,
    ) -> Result<(), RejectionError> {
        let substate: FieldSubstate<TransactionTrackerSubstate> = track
            .read_substate(
                TRANSACTION_TRACKER.as_node_id(),
                MAIN_BASE_PARTITION,
                &TransactionTrackerField::TransactionTracker.into(),
            )
            .unwrap()
            .as_typed()
            .unwrap();

        let partition_number = substate
            .value
            .0
            .partition_for_expiry_epoch(expiry_epoch)
            .expect("Transaction tracker should cover all valid epoch ranges");

        let substate = track.read_substate(
            TRANSACTION_TRACKER.as_node_id(),
            PartitionNumber(partition_number),
            &SubstateKey::Map(intent_hash.to_vec()),
        );

        match substate {
            Some(value) => {
                let substate: KeyValueEntrySubstate<TransactionStatus> = value.as_typed().unwrap();
                match substate.value {
                    Some(status) => match status {
                        TransactionStatus::CommittedSuccess
                        | TransactionStatus::CommittedFailure => {
                            return Err(RejectionError::IntentHashPreviouslyCommitted);
                        }
                        TransactionStatus::Cancelled => {
                            return Err(RejectionError::IntentHashPreviouslyCancelled);
                        }
                    },
                    None => {}
                }
            }
            None => {}
        }

        Ok(())
    }

    fn interpret_manifest(
        &self,
        track: &mut Track<S, SpreadPrefixKeyMapper>,
        executable: &Executable,
        execution_config: &ExecutionConfig,
        fee_reserve: SystemLoanFeeReserve,
        fee_table: FeeTable,
    ) -> (
        Result<Vec<InstructionOutput>, RuntimeError>,
        (
            CostingModule,
            TransactionRuntimeModule,
            ExecutionTraceModule,
        ),
    ) {
        let mut id_allocator = IdAllocator::new(executable.intent_hash().to_hash());
        let mut system = SystemConfig {
            blueprint_cache: NonIterMap::new(),
            auth_cache: NonIterMap::new(),
            schema_cache: NonIterMap::new(),
            callback_obj: self.vm.clone(),
            modules: SystemModuleMixer::new(
                execution_config.enabled_modules,
                executable.intent_hash().to_hash(),
                executable.auth_zone_params().clone(),
                fee_reserve,
                fee_table,
                executable.payload_size(),
                executable.auth_zone_params().initial_proofs.len(),
                execution_config,
            ),
        };

        let kernel_boot = KernelBoot {
            id_allocator: &mut id_allocator,
            callback: &mut system,
            store: track,
        };

        let interpretation_result = kernel_boot
            .call_transaction_processor(
                executable.encoded_instructions(),
                executable.pre_allocated_addresses(),
                executable.references(),
                executable.blobs(),
            )
            .and_then(|x| {
                let info = track.get_commit_info();
                for commit in &info {
                    system.modules.apply_execution_cost(CostingEntry::Commit {
                        store_commit: commit,
                    })?;
                }
                for commit in &info {
                    system.modules.apply_state_expansion_cost(commit)?;
                }

                Ok(x)
            })
            .map(|rtn| {
                let output: Vec<InstructionOutput> = scrypto_decode(&rtn).unwrap();
                output
            });

        (interpretation_result, system.modules.unpack())
    }

    fn determine_result_type(
        mut interpretation_result: Result<Vec<InstructionOutput>, RuntimeError>,
        fee_reserve: &mut SystemLoanFeeReserve,
    ) -> TransactionResultType {
        // A `SuccessButFeeLoanNotRepaid` error is issued if a transaction finishes before
        // the SYSTEM_LOAN_AMOUNT is reached (which trigger a repay event) and even though
        // enough fee has been locked.
        //
        // Do another `repay` try during finalization to remedy it.
        if let Err(err) = fee_reserve.repay_all() {
            if interpretation_result.is_ok() {
                interpretation_result = Err(RuntimeError::SystemModuleError(
                    SystemModuleError::CostingError(CostingError::FeeReserveError(err)),
                ));
            }
        }

        // First - check for required rejections from explicit invoke result errors
        match &interpretation_result {
            Err(RuntimeError::ApplicationError(ApplicationError::TransactionProcessorError(
                err,
            ))) => match err {
                TransactionProcessorError::TransactionEpochNotYetValid {
                    valid_from,
                    current_epoch,
                } => {
                    return TransactionResultType::Reject(
                        RejectionError::TransactionEpochNotYetValid {
                            valid_from: *valid_from,
                            current_epoch: *current_epoch,
                        },
                    )
                }
                TransactionProcessorError::TransactionEpochNoLongerValid {
                    valid_until,
                    current_epoch,
                } => {
                    return TransactionResultType::Reject(
                        RejectionError::TransactionEpochNoLongerValid {
                            valid_until: *valid_until,
                            current_epoch: *current_epoch,
                        },
                    )
                }
                _ => {}
            },
            Err(err) => {
                if let Some(abort_reason) = err.abortion() {
                    return TransactionResultType::Abort(abort_reason.clone());
                }
            }
            _ => {}
        }

        // Check for errors before loan is repaid - in which case, we also reject
        if !fee_reserve.fully_repaid() {
            return match interpretation_result {
                Ok(..) => TransactionResultType::Reject(RejectionError::SuccessButFeeLoanNotRepaid),
                Err(error) => {
                    TransactionResultType::Reject(RejectionError::ErrorBeforeFeeLoanRepaid(error))
                }
            };
        }

        TransactionResultType::Commit(interpretation_result)
    }

    fn finalize_fees(
        track: &mut Track<S, SpreadPrefixKeyMapper>,
        fee_reserve: SystemLoanFeeReserve,
        is_success: bool,
        free_credit: Decimal,
    ) -> (FeeSummary, IndexMap<NodeId, Decimal>) {
        // Distribute royalty
        for (_, (recipient_vault_id, amount)) in fee_reserve.royalty_cost() {
            let node_id = recipient_vault_id;
            let substate_key = FungibleVaultField::LiquidFungible.into();
            let mut substate: FieldSubstate<LiquidFungibleResource> = track
                .read_substate(&node_id, MAIN_BASE_PARTITION, &substate_key)
                .unwrap()
                .as_typed()
                .unwrap();
            substate.value.0.put(LiquidFungibleResource::new(amount));
            track
                .set_substate(
                    node_id,
                    MAIN_BASE_PARTITION,
                    substate_key,
                    IndexedScryptoValue::from_typed(&substate),
                    &mut |_| -> Result<(), ()> { Ok(()) },
                )
                .unwrap();
        }

        // Take fee payments
        let fee_summary = fee_reserve.finalize();
        let mut fee_payments: IndexMap<NodeId, Decimal> = index_map_new();
        let mut required = fee_summary.total_execution_cost_xrd
            + fee_summary.total_tipping_cost_xrd
            + fee_summary.total_state_expansion_cost_xrd
            + fee_summary.total_royalty_cost_xrd;
        let mut collected_fees = LiquidFungibleResource::new(Decimal::ZERO);
        for (vault_id, mut locked, contingent) in fee_summary.locked_fees.iter().cloned().rev() {
            let amount = if contingent {
                if is_success {
                    Decimal::min(locked.amount(), required)
                } else {
                    Decimal::zero()
                }
            } else {
                Decimal::min(locked.amount(), required)
            };

            // Take fees
            collected_fees.put(locked.take_by_amount(amount).unwrap());
            required -= amount;

            // Refund overpayment
            let mut substate: FieldSubstate<LiquidFungibleResource> = track
                .read_substate(
                    &vault_id,
                    MAIN_BASE_PARTITION,
                    &FungibleVaultField::LiquidFungible.into(),
                )
                .unwrap()
                .as_typed()
                .unwrap();
            substate.value.0.put(locked);
            track
                .set_substate(
                    vault_id,
                    MAIN_BASE_PARTITION,
                    FungibleVaultField::LiquidFungible.into(),
                    IndexedScryptoValue::from_typed(&substate),
                    &mut |_| -> Result<(), ()> { Ok(()) },
                )
                .unwrap();

            // Record final payments
            *fee_payments.entry(vault_id).or_default() += amount;
        }
        // Free credit is locked first and thus used last
        if free_credit.is_positive() {
            let amount = Decimal::min(free_credit, required);
            collected_fees.put(LiquidFungibleResource::new(amount));
            required -= amount;
        }

        let to_proposer = fee_summary.to_proposer_amount();
        let to_validator_set = fee_summary.to_validator_set_amount();
        let to_burn = fee_summary.to_burn_amount();

        // Sanity checks
        assert!(
            fee_summary.total_bad_debt_xrd == Decimal::ZERO,
            "Bad debt is non-zero: {}",
            fee_summary.total_bad_debt_xrd
        );
        assert!(
            required == Decimal::ZERO,
            "Locked fee does not cover transaction cost: {} required",
            required
        );
        let remaining_collected_fees = collected_fees.amount() - fee_summary.total_royalty_cost_xrd /* royalty already distributed */;
        assert!(
            remaining_collected_fees  == to_proposer + to_validator_set + to_burn,
            "Remaining collected fee isn't equal to amount to distribute (proposer/validator set/burn): {} != {}",
            remaining_collected_fees,
            to_proposer + to_validator_set + to_burn,
        );

        if !to_proposer.is_zero() || !to_validator_set.is_zero() {
            // Fetch current leader
            // TODO: maybe we should move current leader into validator rewards?
            let substate: FieldSubstate<ConsensusManagerSubstate> = track
                .read_substate(
                    CONSENSUS_MANAGER.as_node_id(),
                    MAIN_BASE_PARTITION,
                    &ConsensusManagerField::ConsensusManager.into(),
                )
                .unwrap()
                .as_typed()
                .unwrap();
            let current_leader = substate.value.0.current_leader;

            // Update validator rewards
            let mut substate: FieldSubstate<ValidatorRewardsSubstate> = track
                .read_substate(
                    CONSENSUS_MANAGER.as_node_id(),
                    MAIN_BASE_PARTITION,
                    &ConsensusManagerField::ValidatorRewards.into(),
                )
                .unwrap()
                .as_typed()
                .unwrap();
            if let Some(current_leader) = current_leader {
                substate
                    .value
                    .0
                    .proposer_rewards
                    .entry(current_leader)
                    .or_default()
                    .add_assign(to_proposer);
            } else {
                // If there is no current leader, the rewards go to the pool
            };
            let vault_node_id = substate.value.0.rewards_vault.0 .0;

            track
                .set_substate(
                    CONSENSUS_MANAGER.into_node_id(),
                    MAIN_BASE_PARTITION,
                    ConsensusManagerField::ValidatorRewards.into(),
                    IndexedScryptoValue::from_typed(&substate),
                    &mut |_| -> Result<(), ()> { Ok(()) },
                )
                .unwrap();

            // Put validator rewards into the vault
            let mut substate: FieldSubstate<LiquidFungibleResource> = track
                .read_substate(
                    &vault_node_id,
                    MAIN_BASE_PARTITION,
                    &FungibleVaultField::LiquidFungible.into(),
                )
                .unwrap()
                .as_typed()
                .unwrap();
            substate.value.0.put(
                collected_fees
                    .take_by_amount(to_proposer + to_validator_set)
                    .unwrap(),
            );
            track
                .set_substate(
                    vault_node_id,
                    MAIN_BASE_PARTITION,
                    FungibleVaultField::LiquidFungible.into(),
                    IndexedScryptoValue::from_typed(&substate),
                    &mut |_| -> Result<(), ()> { Ok(()) },
                )
                .unwrap();
        }

        (fee_summary, fee_payments)
    }

    fn update_transaction_tracker(
        track: &mut Track<S, SpreadPrefixKeyMapper>,
        next_epoch: Epoch,
        intent_hash: &TransactionIntentHash,
        is_success: bool,
    ) {
        // Read the intent hash store
        let mut transaction_tracker: FieldSubstate<TransactionTrackerSubstate> = track
            .read_substate(
                TRANSACTION_TRACKER.as_node_id(),
                MAIN_BASE_PARTITION,
                &TransactionTrackerField::TransactionTracker.into(),
            )
            .unwrap()
            .as_typed()
            .unwrap();

        // Update the status of the intent hash
        if let TransactionIntentHash::ToCheck {
            expiry_epoch,
            intent_hash,
        } = intent_hash
        {
            if let Some(partition_number) = transaction_tracker
                .value
                .0
                .partition_for_expiry_epoch(*expiry_epoch)
            {
                track
                    .set_substate(
                        TRANSACTION_TRACKER.into_node_id(),
                        PartitionNumber(partition_number),
                        SubstateKey::Map(intent_hash.to_vec()),
                        IndexedScryptoValue::from_typed(&KeyValueEntrySubstate {
                            value: Some(if is_success {
                                TransactionStatus::CommittedSuccess
                            } else {
                                TransactionStatus::CommittedFailure
                            }),
                            // TODO: maybe make it immutable, but how does this affect partition deletion?
                            mutability: SubstateMutability::Mutable,
                        }),
                        &mut |_| -> Result<(), ()> { Ok(()) },
                    )
                    .unwrap();
            } else {
                panic!("No partition for an expiry epoch")
            }
        }

        // Check if all intent hashes in the first epoch have expired, based on the `next_epoch`.
        //
        // In this particular implementation, because the transaction tracker coverage is greater than
        // the max epoch range in transaction header, we must check epoch range first to
        // ensure we don't store intent hash too far into the future.
        //
        // Also, we need to make sure epoch doesn't jump by a large distance.
        if next_epoch.number()
            >= transaction_tracker.value.0.start_epoch
                + transaction_tracker.value.0.epochs_per_partition
        {
            let discarded_partition = transaction_tracker.value.0.advance();
            track.delete_partition(
                TRANSACTION_TRACKER.as_node_id(),
                PartitionNumber(discarded_partition),
            );
        }
        track
            .set_substate(
                TRANSACTION_TRACKER.into_node_id(),
                MAIN_BASE_PARTITION,
                TransactionTrackerField::TransactionTracker.into(),
                IndexedScryptoValue::from_typed(&FieldSubstate::new_field(
                    transaction_tracker.value.0,
                )),
                &mut |_| -> Result<(), ()> { Ok(()) },
            )
            .unwrap();
    }

    #[cfg(not(feature = "alloc"))]
    fn print_executable(executable: &Executable) {
        println!("{:-^100}", "Executable");
        println!("Intent hash: {}", executable.intent_hash().as_hash());
        println!("Payload size: {}", executable.payload_size());
        println!("Fee payment: {:?}", executable.fee_payment());
        println!(
            "Pre-allocated addresses: {:?}",
            executable.pre_allocated_addresses()
        );
        println!("Blobs: {:?}", executable.blobs().keys());
        println!("References: {:?}", executable.references());
    }

    #[cfg(not(feature = "alloc"))]
    fn print_execution_summary(receipt: &TransactionReceipt) {
        match &receipt.transaction_result {
            TransactionResult::Commit(commit) => {
                // NB - we use "to_string" to ensure they align correctly

                println!("{:-^100}", "Cost Breakdown");
                for (k, v) in &commit.fee_summary.execution_cost_breakdown {
                    println!("{:<75}: {:>15}", k, v.to_string());
                }

                println!("{:-^100}", "Cost Totals");
                println!(
                    "{:<30}: {:>15}",
                    "Cost Unit Limit",
                    commit.fee_summary.cost_unit_limit.to_string()
                );
                println!(
                    "{:<30}: {:>15}",
                    "Cost Units Consumed",
                    commit.fee_summary.execution_cost_sum.to_string()
                );
                println!(
                    "{:<30}: {:>15}",
                    "Execution Costs in XRD",
                    commit.fee_summary.total_execution_cost_xrd.to_string()
                );
                println!(
                    "{:<30}: {:>15}",
                    "Tipping Costs in XRD",
                    commit.fee_summary.total_tipping_cost_xrd.to_string()
                );
                println!(
                    "{:<30}: {:>15}",
                    "State Expansion Costs in XRD",
                    commit
                        .fee_summary
                        .total_state_expansion_cost_xrd
                        .to_string()
                );
                println!(
                    "{:<30}: {:>15}",
                    "Royalty Costs in XRD",
                    commit.fee_summary.total_royalty_cost_xrd.to_string()
                );

                println!("{:-^100}", "Application Logs");
                for (level, message) in &commit.application_logs {
                    println!("[{}] {}", level, message);
                }

                println!("{:-^100}", "Outcome");
                println!(
                    "{}",
                    match &commit.outcome {
                        TransactionOutcome::Success(_) => "Success".to_string(),
                        TransactionOutcome::Failure(error) => format!("Failure: {:?}", error),
                    }
                );
            }
            TransactionResult::Reject(e) => {
                println!("{:-^100}", "Transaction Rejected");
                println!("{:?}", e.error);
            }
            TransactionResult::Abort(e) => {
                println!("{:-^100}", "Transaction Aborted");
                println!("{:?}", e);
            }
        }
        println!("{:-^100}", "Finish");
    }
}

pub fn execute_and_commit_transaction<
    S: SubstateDatabase + CommittableSubstateDatabase,
    V: SystemCallbackObject + Clone,
>(
    substate_db: &mut S,
    vm: V,
    fee_reserve_config: &FeeReserveConfig,
    execution_config: &ExecutionConfig,
    transaction: &Executable,
) -> TransactionReceipt {
    let receipt = execute_transaction(
        substate_db,
        vm,
        fee_reserve_config,
        execution_config,
        transaction,
    );
    if let TransactionResult::Commit(commit) = &receipt.transaction_result {
        substate_db.commit(&commit.state_updates.database_updates);
    }
    receipt
}

pub fn execute_transaction<S: SubstateDatabase, V: SystemCallbackObject + Clone>(
    substate_db: &S,
    vm: V,
    fee_reserve_config: &FeeReserveConfig,
    execution_config: &ExecutionConfig,
    transaction: &Executable,
) -> TransactionReceipt {
    TransactionExecutor::new(substate_db, vm).execute(
        transaction,
        fee_reserve_config,
        execution_config,
    )
}

enum TransactionResultType {
    Commit(Result<Vec<InstructionOutput>, RuntimeError>),
    Reject(RejectionError),
    Abort(AbortReason),
}
