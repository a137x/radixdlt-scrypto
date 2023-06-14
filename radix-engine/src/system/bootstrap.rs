use crate::blueprints::access_controller::*;
use crate::blueprints::account::AccountNativePackage;
use crate::blueprints::consensus_manager::ConsensusManagerNativePackage;
use crate::blueprints::identity::IdentityNativePackage;
use crate::blueprints::package::PackageNativePackage;
use crate::blueprints::resource::ResourceManagerNativePackage;
use crate::blueprints::transaction_processor::TransactionProcessorNativePackage;
use crate::system::node_modules::access_rules::AccessRulesNativePackage;
use crate::system::node_modules::metadata::MetadataNativePackage;
use crate::system::node_modules::royalty::RoyaltyNativePackage;
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::track::db_key_mapper::{MappedSubstateDatabase, SpreadPrefixKeyMapper};
use crate::transaction::{
    execute_transaction, ExecutionConfig, FeeReserveConfig, TransactionReceipt,
};
use crate::types::*;
use crate::vm::wasm::WasmEngine;
use crate::vm::ScryptoVm;
use lazy_static::lazy_static;
use radix_engine_common::crypto::EcdsaSecp256k1PublicKey;
use radix_engine_common::types::ComponentAddress;
use radix_engine_interface::api::node_modules::auth::AuthAddresses;
use radix_engine_interface::api::node_modules::metadata::{MetadataValue, Url};
use radix_engine_interface::blueprints::consensus_manager::{
    ConsensusManagerConfig, EpochChangeCondition, CONSENSUS_MANAGER_BLUEPRINT,
    CONSENSUS_MANAGER_CREATE_IDENT,
};
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::rule;
use radix_engine_store_interface::interface::{CommittableSubstateDatabase, SubstateDatabase};
use transaction::model::{
    BlobsV1, InstructionV1, InstructionsV1, SystemTransactionV1, TransactionPayload,
};
use transaction::validation::ManifestIdAllocator;

const XRD_SYMBOL: &str = "XRD";
const XRD_NAME: &str = "Radix";
const XRD_DESCRIPTION: &str = "The Radix Public Network's native token, used to pay the network's required transaction fees and to secure the network through staking to its validator nodes.";
const XRD_URL: &str = "https://tokens.radixdlt.com";
const XRD_ICON_URL: &str = "https://assets.radixdlt.com/icons/icon-xrd-32x32.png";

lazy_static! {
    pub static ref DEFAULT_TESTING_FAUCET_SUPPLY: Decimal = dec!("100000000000000000");
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct GenesisValidator {
    pub key: EcdsaSecp256k1PublicKey,
    pub accept_delegated_stake: bool,
    pub is_registered: bool,
    pub fee_factor: Decimal,
    pub metadata: Vec<(String, MetadataValue)>,
    pub owner: ComponentAddress,
}

impl From<EcdsaSecp256k1PublicKey> for GenesisValidator {
    fn from(key: EcdsaSecp256k1PublicKey) -> Self {
        // Re-using the validator key for its owner
        let default_owner_address = ComponentAddress::virtual_account_from_public_key(&key);
        GenesisValidator {
            key,
            accept_delegated_stake: true,
            is_registered: true,
            fee_factor: Decimal::zero(),
            metadata: vec![(
                "url".to_string(),
                MetadataValue::Url(Url(format!("http://test.local?validator={:?}", key))),
            )],
            owner: default_owner_address,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct GenesisStakeAllocation {
    pub account_index: u32,
    pub xrd_amount: Decimal,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct GenesisResource {
    pub address_bytes_without_entity_id: [u8; NodeId::UUID_LENGTH],
    pub metadata: Vec<(String, MetadataValue)>,
    pub owner: Option<ComponentAddress>,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct GenesisResourceAllocation {
    pub account_index: u32,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub enum GenesisDataChunk {
    Validators(Vec<GenesisValidator>),
    Stakes {
        accounts: Vec<ComponentAddress>,
        allocations: Vec<(EcdsaSecp256k1PublicKey, Vec<GenesisStakeAllocation>)>,
    },
    Resources(Vec<GenesisResource>),
    ResourceBalances {
        accounts: Vec<ComponentAddress>,
        allocations: Vec<(ResourceAddress, Vec<GenesisResourceAllocation>)>,
    },
    XrdBalances(Vec<(ComponentAddress, Decimal)>),
}

#[derive(Debug, Clone, ScryptoSbor)]
pub struct GenesisReceipts {
    pub system_bootstrap_receipt: TransactionReceipt,
    pub data_ingestion_receipts: Vec<TransactionReceipt>,
    pub wrap_up_receipt: TransactionReceipt,
}

pub struct Bootstrapper<'s, 'i, S, W>
where
    S: SubstateDatabase + CommittableSubstateDatabase,
    W: WasmEngine,
{
    substate_db: &'s mut S,
    scrypto_vm: &'i ScryptoVm<W>,
    trace: bool,
}

impl<'s, 'i, S, W> Bootstrapper<'s, 'i, S, W>
where
    S: SubstateDatabase + CommittableSubstateDatabase,
    W: WasmEngine,
{
    pub fn new(
        substate_db: &'s mut S,
        scrypto_vm: &'i ScryptoVm<W>,
        trace: bool,
    ) -> Bootstrapper<'s, 'i, S, W> {
        Bootstrapper {
            substate_db,
            scrypto_vm,
            trace,
        }
    }

    pub fn bootstrap_test_default(&mut self) -> Option<GenesisReceipts> {
        self.bootstrap_with_genesis_data(
            vec![],
            Epoch::of(1),
            ConsensusManagerConfig {
                max_validators: 10,
                epoch_change_condition: EpochChangeCondition {
                    min_round_count: 1,
                    max_round_count: 1,
                    target_duration_millis: 0,
                },
                num_unstake_epochs: 1,
                total_emission_xrd_per_epoch: Decimal::one(),
                min_validator_reliability: Decimal::one(),
                num_owner_stake_units_unlock_epochs: 2,
                num_fee_increase_delay_epochs: 1,
            },
            1,
            *DEFAULT_TESTING_FAUCET_SUPPLY,
        )
    }

    pub fn bootstrap_with_genesis_data(
        &mut self,
        genesis_data_chunks: Vec<GenesisDataChunk>,
        initial_epoch: Epoch,
        initial_config: ConsensusManagerConfig,
        initial_time_ms: i64,
        faucet_supply: Decimal,
    ) -> Option<GenesisReceipts> {
        let xrd_info = self
            .substate_db
            .get_mapped::<SpreadPrefixKeyMapper, TypeInfoSubstate>(
                &RADIX_TOKEN.into(),
                TYPE_INFO_FIELD_PARTITION,
                &TypeInfoField::TypeInfo.into(),
            );

        if xrd_info.is_none() {
            let system_bootstrap_receipt =
                self.execute_system_bootstrap(initial_epoch, initial_config, initial_time_ms);

            let mut data_ingestion_receipts = vec![];
            for (chunk_index, chunk) in genesis_data_chunks.into_iter().enumerate() {
                let receipt = self.ingest_genesis_data_chunk(chunk, chunk_index);
                data_ingestion_receipts.push(receipt);
            }

            let genesis_wrap_up_receipt = self.execute_genesis_wrap_up(faucet_supply);

            Some(GenesisReceipts {
                system_bootstrap_receipt,
                data_ingestion_receipts,
                wrap_up_receipt: genesis_wrap_up_receipt,
            })
        } else {
            None
        }
    }

    fn execute_system_bootstrap(
        &mut self,
        initial_epoch: Epoch,
        initial_config: ConsensusManagerConfig,
        initial_time_ms: i64,
    ) -> TransactionReceipt {
        let transaction =
            create_system_bootstrap_transaction(initial_epoch, initial_config, initial_time_ms);

        let receipt = execute_transaction(
            self.substate_db,
            self.scrypto_vm,
            &FeeReserveConfig::default(),
            &ExecutionConfig::genesis().with_trace(self.trace),
            &transaction
                .prepare()
                .expect("Expected system bootstrap transaction to be preparable")
                .get_executable(btreeset![AuthAddresses::system_role()]),
        );

        let commit_result = receipt.expect_commit(true);

        self.substate_db
            .commit(&commit_result.state_updates.database_updates);

        receipt
    }

    fn ingest_genesis_data_chunk(
        &mut self,
        chunk: GenesisDataChunk,
        chunk_number: usize,
    ) -> TransactionReceipt {
        let transaction =
            create_genesis_data_ingestion_transaction(&GENESIS_HELPER, chunk, chunk_number);
        let receipt = execute_transaction(
            self.substate_db,
            self.scrypto_vm,
            &FeeReserveConfig::default(),
            &ExecutionConfig::genesis().with_trace(self.trace),
            &transaction
                .prepare()
                .expect("Expected genesis data chunk transaction to be preparable")
                .get_executable(btreeset![AuthAddresses::system_role()]),
        );

        let commit_result = receipt.expect_commit(true);
        self.substate_db
            .commit(&commit_result.state_updates.database_updates);

        receipt
    }

    fn execute_genesis_wrap_up(&mut self, faucet_supply: Decimal) -> TransactionReceipt {
        let transaction = create_genesis_wrap_up_transaction(faucet_supply);

        let receipt = execute_transaction(
            self.substate_db,
            self.scrypto_vm,
            &FeeReserveConfig::default(),
            &ExecutionConfig::genesis(),
            &transaction
                .prepare()
                .expect("Expected genesis wrap up transaction to be preparable")
                .get_executable(btreeset![AuthAddresses::system_role()]),
        );

        let commit_result = receipt.expect_commit(true);
        self.substate_db
            .commit(&commit_result.state_updates.database_updates);

        receipt
    }
}

pub fn create_system_bootstrap_transaction(
    initial_epoch: Epoch,
    initial_config: ConsensusManagerConfig,
    initial_time_ms: i64,
) -> SystemTransactionV1 {
    // NOTES
    // * Create resources before packages to avoid circular dependencies.

    let mut instructions = Vec::new();
    let mut pre_allocated_ids = index_set_new();

    // Package Package
    {
        pre_allocated_ids.insert(PACKAGE_PACKAGE.into());
        let package_address = PACKAGE_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                native_package_code_id: PACKAGE_CODE_ID,
                schema: PackageNativePackage::schema(),
                dependent_resources: vec![
                    PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
                    PACKAGE_OWNER_BADGE,
                ],
                dependent_components: vec![],
                metadata: BTreeMap::new(),
                package_access_rules: PackageNativePackage::function_access_rules(),
                default_package_access_rule: AccessRule::DenyAll,
            }),
        });
    }

    // Metadata Package
    {
        pre_allocated_ids.insert(METADATA_MODULE_PACKAGE.into());
        let package_address = METADATA_MODULE_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                native_package_code_id: METADATA_CODE_ID,
                schema: MetadataNativePackage::schema(),
                dependent_resources: vec![],
                dependent_components: vec![],
                metadata: BTreeMap::new(),
                package_access_rules: MetadataNativePackage::function_access_rules(),
                default_package_access_rule: AccessRule::DenyAll,
            }),
        });
    }

    // Royalty Package
    {
        pre_allocated_ids.insert(ROYALTY_MODULE_PACKAGE.into());
        let package_address = ROYALTY_MODULE_PACKAGE.into();

        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                native_package_code_id: ROYALTY_CODE_ID,
                schema: RoyaltyNativePackage::schema(),
                dependent_resources: vec![RADIX_TOKEN],
                dependent_components: vec![],
                metadata: BTreeMap::new(),
                package_access_rules: RoyaltyNativePackage::function_access_rules(),
                default_package_access_rule: AccessRule::DenyAll,
            }),
        });
    }

    // Access Rules Package
    {
        pre_allocated_ids.insert(ACCESS_RULES_MODULE_PACKAGE.into());
        let package_address = ACCESS_RULES_MODULE_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                native_package_code_id: ACCESS_RULES_CODE_ID,
                schema: AccessRulesNativePackage::schema(),
                dependent_resources: vec![],
                dependent_components: vec![],
                metadata: BTreeMap::new(),
                package_access_rules: AccessRulesNativePackage::function_access_rules(),
                default_package_access_rule: AccessRule::DenyAll,
            }),
        });
    }

    // Resource Package
    {
        pre_allocated_ids.insert(RESOURCE_PACKAGE.into());
        let package_address = RESOURCE_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                native_package_code_id: RESOURCE_MANAGER_CODE_ID,
                schema: ResourceManagerNativePackage::schema(),
                dependent_resources: vec![],
                dependent_components: vec![],
                metadata: BTreeMap::new(),
                package_access_rules: BTreeMap::new(),
                default_package_access_rule: AccessRule::AllowAll,
            }),
        });
    }

    // XRD Token
    {
        let mut metadata = BTreeMap::new();
        metadata.insert(
            "symbol".to_owned(),
            MetadataValue::String(XRD_SYMBOL.to_owned()),
        );
        metadata.insert(
            "name".to_owned(),
            MetadataValue::String(XRD_NAME.to_owned()),
        );
        metadata.insert(
            "description".to_owned(),
            MetadataValue::String(XRD_DESCRIPTION.to_owned()),
        );
        metadata.insert("url".to_owned(), MetadataValue::String(XRD_URL.to_owned()));
        metadata.insert(
            "icon_url".to_owned(),
            MetadataValue::String(XRD_ICON_URL.to_owned()),
        );

        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        access_rules.insert(
            Mint,
            (
                rule!(require(global_caller(CONSENSUS_MANAGER))),
                rule!(deny_all),
            ),
        );
        let resource_address = RADIX_TOKEN.into();
        pre_allocated_ids.insert(RADIX_TOKEN.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_AND_ADDRESS_IDENT
                .to_string(),
            args: to_manifest_value(
                &FungibleResourceManagerCreateWithInitialSupplyAndAddressInput {
                    divisibility: 18,
                    metadata,
                    access_rules,
                    initial_supply: Decimal::zero(),
                    resource_address,
                },
            ),
        });
    }

    // Package Token
    {
        let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(deny_all), rule!(deny_all)));
        let resource_address = PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.into();
        pre_allocated_ids.insert(PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::Bytes,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata,
                access_rules,
                resource_address,
            }),
        });
    }

    // Object Token
    {
        let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(deny_all), rule!(deny_all)));
        let resource_address = GLOBAL_CALLER_VIRTUAL_BADGE.into();
        pre_allocated_ids.insert(GLOBAL_CALLER_VIRTUAL_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::Bytes,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata,
                access_rules,
                resource_address,
            }),
        });
    }

    // Package Owner Token
    {
        // TODO: Integrate this into package instantiation to remove circular dependency
        let mut access_rules = BTreeMap::new();
        access_rules.insert(
            Mint,
            (
                rule!(require(package_of_direct_caller(PACKAGE_PACKAGE))),
                rule!(deny_all),
            ),
        );
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = PACKAGE_OWNER_BADGE.into();
        pre_allocated_ids.insert(PACKAGE_OWNER_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::UUID,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata: btreemap!(),
                access_rules,
                resource_address,
            }),
        });
    }

    // Identity Package
    {
        // TODO: Integrate this into package instantiation to remove circular dependency
        let mut access_rules = BTreeMap::new();
        access_rules.insert(
            Mint,
            (
                rule!(require(package_of_direct_caller(IDENTITY_PACKAGE))),
                rule!(deny_all),
            ),
        );
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = IDENTITY_OWNER_BADGE.into();
        pre_allocated_ids.insert(IDENTITY_OWNER_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::UUID,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata: btreemap!(),
                access_rules,
                resource_address,
            }),
        });

        pre_allocated_ids.insert(IDENTITY_PACKAGE.into());
        let package_address = IDENTITY_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                schema: IdentityNativePackage::schema(),
                dependent_resources: vec![
                    ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE,
                    EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE,
                    IDENTITY_OWNER_BADGE,
                    PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
                ],
                dependent_components: vec![],
                native_package_code_id: IDENTITY_CODE_ID,
                metadata: BTreeMap::new(),
                package_access_rules: BTreeMap::new(),
                default_package_access_rule: AccessRule::AllowAll,
            }),
        });
    }

    // ConsensusManager Package
    {
        pre_allocated_ids.insert(CONSENSUS_MANAGER_PACKAGE.into());
        let package_address = CONSENSUS_MANAGER_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                schema: ConsensusManagerNativePackage::schema(),
                native_package_code_id: CONSENSUS_MANAGER_CODE_ID,
                metadata: BTreeMap::new(),
                dependent_resources: vec![
                    RADIX_TOKEN,
                    PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
                    SYSTEM_TRANSACTION_BADGE,
                    VALIDATOR_OWNER_BADGE,
                ],
                dependent_components: vec![],
                package_access_rules: ConsensusManagerNativePackage::package_access_rules(),
                default_package_access_rule: AccessRule::DenyAll,
            }),
        });
    }

    // Account Package
    {
        // TODO: Integrate this into package instantiation to remove circular dependency
        let mut access_rules = BTreeMap::new();
        access_rules.insert(
            Mint,
            (
                rule!(require(package_of_direct_caller(ACCOUNT_PACKAGE))),
                rule!(deny_all),
            ),
        );
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = ACCOUNT_OWNER_BADGE.into();
        pre_allocated_ids.insert(ACCOUNT_OWNER_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::UUID,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata: btreemap!(),
                access_rules,
                resource_address,
            }),
        });

        pre_allocated_ids.insert(ACCOUNT_PACKAGE.into());
        let package_address = ACCOUNT_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                schema: AccountNativePackage::schema(),
                native_package_code_id: ACCOUNT_CODE_ID,
                metadata: BTreeMap::new(),
                dependent_resources: vec![
                    ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE,
                    EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE,
                    ACCOUNT_OWNER_BADGE,
                    PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE,
                ],
                dependent_components: vec![],
                package_access_rules: BTreeMap::new(),
                default_package_access_rule: AccessRule::AllowAll,
            }),
        });
    }

    // AccessController Package
    {
        pre_allocated_ids.insert(ACCESS_CONTROLLER_PACKAGE.into());
        let package_address = ACCESS_CONTROLLER_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                schema: AccessControllerNativePackage::schema(),
                metadata: BTreeMap::new(),
                native_package_code_id: ACCESS_CONTROLLER_CODE_ID,
                dependent_resources: vec![PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE],
                dependent_components: vec![CONSENSUS_MANAGER],
                package_access_rules: BTreeMap::new(),
                default_package_access_rule: AccessRule::AllowAll,
            }),
        });
    }

    // TransactionProcessor Package
    {
        pre_allocated_ids.insert(TRANSACTION_PROCESSOR_PACKAGE.into());
        let package_address = TRANSACTION_PROCESSOR_PACKAGE.into();
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishNativeInput {
                package_address: Some(package_address), // TODO: Clean this up
                schema: TransactionProcessorNativePackage::schema(),
                metadata: BTreeMap::new(),
                native_package_code_id: TRANSACTION_PROCESSOR_CODE_ID,
                dependent_resources: vec![],
                dependent_components: vec![CONSENSUS_MANAGER],
                package_access_rules: BTreeMap::new(),
                default_package_access_rule: AccessRule::AllowAll,
            }),
        });
    }

    // ECDSA
    {
        let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE.into();
        pre_allocated_ids.insert(ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::Bytes,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata,
                access_rules,
                resource_address,
            }),
        });
    }

    // EDDSA ED25519 Token
    {
        let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE.into();
        pre_allocated_ids.insert(EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::Bytes,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata,
                access_rules,
                resource_address,
            }),
        });
    }

    // System Token
    {
        let metadata: BTreeMap<String, MetadataValue> = BTreeMap::new();
        let mut access_rules = BTreeMap::new();
        access_rules.insert(Withdraw, (rule!(allow_all), rule!(deny_all)));
        let resource_address = SYSTEM_TRANSACTION_BADGE.into();
        pre_allocated_ids.insert(SYSTEM_TRANSACTION_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_ADDRESS_IDENT.to_string(),
            args: to_manifest_value(&NonFungibleResourceManagerCreateWithAddressInput {
                id_type: NonFungibleIdType::Bytes,
                non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                metadata,
                access_rules,
                resource_address,
            }),
        });
    }

    // Faucet Package
    {
        let faucet_code = include_bytes!("../../../assets/faucet.wasm").to_vec();
        let faucet_abi = include_bytes!("../../../assets/faucet.schema").to_vec();
        let package_address = FAUCET_PACKAGE.into();
        pre_allocated_ids.insert(FAUCET_PACKAGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_WASM_ADVANCED_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishWasmAdvancedInput {
                package_address: Some(package_address),
                code: faucet_code,
                schema: manifest_decode(&faucet_abi).unwrap(),
                royalty_config: BTreeMap::new(),
                metadata: BTreeMap::new(),
                authority_rules: AuthorityRules::new(),
            }),
        });
    }

    // Genesis helper package
    {
        // TODO: Add authorization rules around preventing anyone else from
        // TODO: calling genesis helper code
        let genesis_helper_code = include_bytes!("../../../assets/genesis_helper.wasm").to_vec();
        let genesis_helper_abi = include_bytes!("../../../assets/genesis_helper.schema").to_vec();
        let package_address = GENESIS_HELPER_PACKAGE.into();
        pre_allocated_ids.insert(GENESIS_HELPER_PACKAGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            function_name: PACKAGE_PUBLISH_WASM_ADVANCED_IDENT.to_string(),
            args: to_manifest_value(&PackagePublishWasmAdvancedInput {
                package_address: Some(package_address),
                code: genesis_helper_code,
                schema: manifest_decode(&genesis_helper_abi).unwrap(),
                royalty_config: BTreeMap::new(),
                metadata: BTreeMap::new(),
                authority_rules: AuthorityRules::new(),
            }),
        });
    }

    // Create ConsensusManager
    {
        let consensus_manager_component_address =
            Into::<[u8; NodeId::LENGTH]>::into(CONSENSUS_MANAGER);
        let validator_owner_token = Into::<[u8; NodeId::LENGTH]>::into(VALIDATOR_OWNER_BADGE);
        pre_allocated_ids.insert(CONSENSUS_MANAGER.into());
        pre_allocated_ids.insert(VALIDATOR_OWNER_BADGE.into());
        instructions.push(InstructionV1::CallFunction {
            package_address: CONSENSUS_MANAGER_PACKAGE,
            blueprint_name: CONSENSUS_MANAGER_BLUEPRINT.to_string(),
            function_name: CONSENSUS_MANAGER_CREATE_IDENT.to_string(),
            args: manifest_args!(
                validator_owner_token,
                consensus_manager_component_address,
                initial_epoch,
                initial_config,
                initial_time_ms
            ),
        });
    }

    // Create GenesisHelper
    {
        pre_allocated_ids.insert(GENESIS_HELPER.into());
        let address_bytes = GENESIS_HELPER.as_node_id().0;
        instructions.push(InstructionV1::CallFunction {
            package_address: GENESIS_HELPER_PACKAGE,
            blueprint_name: GENESIS_HELPER_BLUEPRINT.to_string(),
            function_name: "new".to_string(),
            args: manifest_args!(
                address_bytes,
                CONSENSUS_MANAGER,
                AuthAddresses::system_role()
            ),
        });
    }

    SystemTransactionV1 {
        instructions: InstructionsV1(instructions),
        pre_allocated_ids,
        blobs: BlobsV1 { blobs: vec![] },
        hash_for_execution: hash(format!("Genesis Bootstrap")),
    }
}

pub fn create_genesis_data_ingestion_transaction(
    genesis_helper: &ComponentAddress,
    chunk: GenesisDataChunk,
    chunk_number: usize,
) -> SystemTransactionV1 {
    let mut instructions = Vec::new();
    let mut pre_allocated_ids = index_set_new();

    if let GenesisDataChunk::Resources(resources) = &chunk {
        for resource in resources {
            pre_allocated_ids.insert(NodeId::new(
                EntityType::GlobalFungibleResourceManager as u8,
                &resource.address_bytes_without_entity_id,
            ));
        }
    }

    instructions.push(InstructionV1::CallMethod {
        address: genesis_helper.clone().into(),
        method_name: "ingest_data_chunk".to_string(),
        args: manifest_args!(chunk),
    });

    SystemTransactionV1 {
        instructions: InstructionsV1(instructions),
        pre_allocated_ids,
        blobs: BlobsV1 { blobs: vec![] },
        hash_for_execution: hash(format!("Genesis Data Chunk: {}", chunk_number)),
    }
}

pub fn create_genesis_wrap_up_transaction(faucet_supply: Decimal) -> SystemTransactionV1 {
    let mut id_allocator = ManifestIdAllocator::new();
    let mut instructions = Vec::new();

    instructions.push(InstructionV1::CallMethod {
        address: GENESIS_HELPER.clone().into(),
        method_name: "wrap_up".to_string(),
        args: manifest_args!(),
    });

    instructions.push(
        InstructionV1::CallMethod {
            address: RADIX_TOKEN.clone().into(),
            method_name: FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
            args: manifest_args!(faucet_supply),
        }
        .into(),
    );

    instructions.push(
        InstructionV1::TakeAllFromWorktop {
            resource_address: RADIX_TOKEN,
        }
        .into(),
    );

    let bucket = id_allocator.new_bucket_id().unwrap();
    let address_bytes = FAUCET.as_node_id().0;

    instructions.push(InstructionV1::CallFunction {
        package_address: FAUCET_PACKAGE,
        blueprint_name: FAUCET_BLUEPRINT.to_string(),
        function_name: "new".to_string(),
        args: manifest_args!(address_bytes, bucket),
    });

    SystemTransactionV1 {
        instructions: InstructionsV1(instructions),
        pre_allocated_ids: indexset! { FAUCET.as_node_id().clone() },
        blobs: BlobsV1 { blobs: vec![] },
        hash_for_execution: hash(format!("Genesis Wrap Up")),
    }
}
