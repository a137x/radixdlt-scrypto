use crate::blueprints::consensus_manager::{
    ConsensusManagerField, VersionedConsensusManagerConfiguration,
};
use crate::blueprints::models::KeyValueEntryContentSource;
use crate::blueprints::package::*;
use crate::blueprints::pool::v1::package::*;
use crate::internal_prelude::*;
use crate::system::system_db_reader::{ObjectCollectionKey, SystemDatabaseReader};
use crate::track::{NodeStateUpdates, PartitionStateUpdates, StateUpdates};
use crate::vm::VmApi;
use crate::vm::*;
use radix_engine_common::constants::*;
use radix_engine_common::crypto::hash;
use radix_engine_common::math::Decimal;
use radix_engine_common::prelude::ScopedTypeId;
use radix_engine_common::prelude::{scrypto_encode, ScryptoCustomTypeKind};
use radix_engine_common::types::SubstateKey;
use radix_engine_interface::api::ObjectModuleId;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::prelude::*;
use radix_engine_interface::types::CollectionDescriptor;
use radix_engine_store_interface::interface::*;
use sbor::HasLatestVersion;
use sbor::{generate_full_schema, TypeAggregator};
use utils::indexmap;

pub fn generate_vm_boot_scrypto_minor_version_state_updates() -> StateUpdates {
    let substate = scrypto_encode(&VmBoot::V1 {
        scrypto_v1_minor_version: 1u64,
    })
    .unwrap();

    StateUpdates {
        by_node: indexmap!(
            TRANSACTION_TRACKER.into_node_id() => NodeStateUpdates::Delta {
                by_partition: indexmap! {
                    BOOT_LOADER_PARTITION => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Field(BOOT_LOADER_VM_SUBSTATE_FIELD_KEY) => DatabaseUpdate::Set(substate)
                        }
                    },
                }
            }
        ),
    }
}

/// Generates the state updates required for updating the Consensus Manager blueprint
/// to use seconds precision
pub fn generate_seconds_precision_state_updates<S: SubstateDatabase>(db: &S) -> StateUpdates {
    let reader = SystemDatabaseReader::new(db);
    let consensus_mgr_pkg_node_id = CONSENSUS_MANAGER_PACKAGE.into_node_id();
    let bp_version_key = BlueprintVersionKey {
        blueprint: CONSENSUS_MANAGER_BLUEPRINT.to_string(),
        version: BlueprintVersion::default(),
    };

    // Generate the new code substates
    let (new_code_substate, new_vm_type_substate, code_hash) = {
        let original_code = CONSENSUS_MANAGER_SECONDS_PRECISION_CODE_ID
            .to_be_bytes()
            .to_vec();

        let code_hash = CodeHash::from_hash(hash(&original_code));
        let versioned_code = VersionedPackageCodeOriginalCode::V1(PackageCodeOriginalCodeV1 {
            code: original_code,
        });
        let code_payload = versioned_code.into_payload();
        let code_substate = code_payload.into_locked_substate();
        let vm_type_substate = PackageCodeVmTypeV1 {
            vm_type: VmType::Native,
        }
        .into_versioned()
        .into_locked_substate();
        (
            scrypto_encode(&code_substate).unwrap(),
            scrypto_encode(&vm_type_substate).unwrap(),
            code_hash,
        )
    };

    // Generate the new schema substate
    let (
        new_schema_substate,
        get_current_time_input_v2_type_id,
        compare_current_time_input_v2_type_id,
        new_schema_hash,
    ) = {
        let mut aggregator = TypeAggregator::<ScryptoCustomTypeKind>::new();
        let get_current_time_input_v2 =
            aggregator.add_child_type_and_descendents::<ConsensusManagerGetCurrentTimeInputV2>();
        let compare_current_time_input_v2 = aggregator
            .add_child_type_and_descendents::<ConsensusManagerCompareCurrentTimeInputV2>();
        let schema = generate_full_schema(aggregator);
        let schema_hash = schema.generate_schema_hash();
        let schema_substate = schema.into_locked_substate();
        (
            scrypto_encode(&schema_substate).unwrap(),
            get_current_time_input_v2,
            compare_current_time_input_v2,
            schema_hash,
        )
    };

    // Generate the blueprint definition substate updates
    let updated_bp_definition_substate = {
        let versioned_definition: VersionedPackageBlueprintVersionDefinition = reader
            .read_object_collection_entry(
                &consensus_mgr_pkg_node_id,
                ObjectModuleId::Main,
                ObjectCollectionKey::KeyValue(
                    PackageCollection::BlueprintVersionDefinitionKeyValue.collection_index(),
                    &bp_version_key,
                ),
            )
            .unwrap()
            .unwrap();

        let mut definition = versioned_definition.into_latest();

        let export = definition
            .function_exports
            .get_mut(CONSENSUS_MANAGER_GET_CURRENT_TIME_IDENT)
            .unwrap();
        export.code_hash = code_hash;
        let function_schema = definition
            .interface
            .functions
            .get_mut(CONSENSUS_MANAGER_GET_CURRENT_TIME_IDENT)
            .unwrap();
        function_schema.input = BlueprintPayloadDef::Static(ScopedTypeId(
            new_schema_hash,
            get_current_time_input_v2_type_id,
        ));

        let export = definition
            .function_exports
            .get_mut(CONSENSUS_MANAGER_COMPARE_CURRENT_TIME_IDENT)
            .unwrap();
        export.code_hash = code_hash;
        let function_schema = definition
            .interface
            .functions
            .get_mut(CONSENSUS_MANAGER_COMPARE_CURRENT_TIME_IDENT)
            .unwrap();
        function_schema.input = BlueprintPayloadDef::Static(ScopedTypeId(
            new_schema_hash,
            compare_current_time_input_v2_type_id,
        ));

        scrypto_encode(
            &VersionedPackageBlueprintVersionDefinition::V1(definition).into_locked_substate(),
        )
        .unwrap()
    };

    let bp_definition_partition_num = reader
        .get_partition_of_collection(
            &consensus_mgr_pkg_node_id,
            ObjectModuleId::Main,
            PackageCollection::BlueprintVersionDefinitionKeyValue.collection_index(),
        )
        .unwrap();

    let code_vm_type_partition_num = reader
        .get_partition_of_collection(
            &consensus_mgr_pkg_node_id,
            ObjectModuleId::Main,
            PackageCollection::CodeVmTypeKeyValue.collection_index(),
        )
        .unwrap();

    let code_partition_num = reader
        .get_partition_of_collection(
            &consensus_mgr_pkg_node_id,
            ObjectModuleId::Main,
            PackageCollection::CodeOriginalCodeKeyValue.collection_index(),
        )
        .unwrap();

    let schema_partition_num = reader
        .get_partition_of_collection(
            &consensus_mgr_pkg_node_id,
            ObjectModuleId::Main,
            PackageCollection::SchemaKeyValue.collection_index(),
        )
        .unwrap();

    StateUpdates {
        by_node: indexmap!(
            consensus_mgr_pkg_node_id => NodeStateUpdates::Delta {
                by_partition: indexmap! {
                    bp_definition_partition_num => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Map(scrypto_encode(&bp_version_key).unwrap()) => DatabaseUpdate::Set(
                                updated_bp_definition_substate
                            )
                        }
                    },
                    code_vm_type_partition_num => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Map(scrypto_encode(&code_hash).unwrap()) => DatabaseUpdate::Set(new_vm_type_substate)
                        }
                    },
                    code_partition_num => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Map(scrypto_encode(&code_hash).unwrap()) => DatabaseUpdate::Set(new_code_substate)
                        }
                    },
                    schema_partition_num => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Map(scrypto_encode(&new_schema_hash).unwrap()) => DatabaseUpdate::Set(new_schema_substate)
                        }
                    }
                }
            }
        ),
    }
}

pub mod pools_package_v1_1 {
    use crate::track::NodeSubstates;

    use super::*;

    pub fn generate_state_updates<S: SubstateDatabase>(db: &S) -> StateUpdates {
        let mut state_updates = StateUpdates::default();

        let node_id = POOL_PACKAGE.into_node_id();
        let node_substates_v0 = compute_pool_package_substates(db, PoolV1MinorVersion::Zero);
        let node_substates_v1 = compute_pool_package_substates(db, PoolV1MinorVersion::One);

        let mut a = 0u32;
        let mut b = 0u32;
        for (partition_number, entries) in &node_substates_v0 {
            for (key, _) in entries {
                if node_substates_v1
                    .get(partition_number)
                    .and_then(|entries| entries.get(key))
                    .is_none()
                {
                    a += 1;
                    state_updates
                        .by_node
                        .entry(node_id)
                        .or_default()
                        .of_partition(*partition_number)
                        .update_substates(vec![(key.clone(), DatabaseUpdate::Delete)])
                }
            }
        }

        for (partition_number, entries) in &node_substates_v1 {
            for (key, value) in entries {
                if node_substates_v0
                    .get(partition_number)
                    .and_then(|entries| entries.get(key))
                    != Some(value)
                {
                    b += 1;
                    state_updates
                        .by_node
                        .entry(node_id)
                        .or_default()
                        .of_partition(*partition_number)
                        .update_substates(vec![(
                            key.clone(),
                            DatabaseUpdate::Set(value.clone().into()),
                        )])
                }
            }
        }

        assert_eq!(a, 2);
        assert_eq!(b, 2 + 3);

        state_updates
    }

    fn compute_pool_package_substates<S: SubstateDatabase>(
        db: &S,
        version: PoolV1MinorVersion,
    ) -> NodeSubstates {
        let reader = SystemDatabaseReader::new(db);
        let node_id = POOL_PACKAGE.into_node_id();

        let original_code = match version {
            PoolV1MinorVersion::Zero => POOL_V1_0_CODE_ID,
            PoolV1MinorVersion::One => POOL_V1_1_CODE_ID,
        }
        .to_be_bytes()
        .to_vec();

        let package_structure = PackageNativePackage::validate_and_build_package_structure(
            PoolNativePackage::definition(version),
            VmType::Native,
            original_code,
            Default::default(),
            &MockVmApi,
        )
        .unwrap();

        let royalty_vault = reader
            .read_object_field(
                &node_id,
                ModuleId::Main,
                PackageField::RoyaltyAccumulator.field_index(),
            )
            .unwrap()
            .as_typed::<PackageRoyaltyAccumulatorFieldPayload>()
            .unwrap()
            .into_latest()
            .royalty_vault;

        create_package_partition_substates(
            package_structure,
            metadata_init! {
                "name" => "Pool Package".to_owned(), locked;
                "description" => "A native package that defines the logic for a selection of pool components.".to_owned(), locked;
            },
            Some(royalty_vault),
        )
    }

    struct MockVmApi;

    impl VmApi for MockVmApi {
        fn get_scrypto_minor_version(&self) -> u64 {
            0
        }
    }
}

pub fn generate_validator_fee_fix_state_updates<S: SubstateDatabase>(db: &S) -> StateUpdates {
    let reader = SystemDatabaseReader::new(db);
    let consensus_mgr_node_id = CONSENSUS_MANAGER.into_node_id();

    let versioned_config: VersionedConsensusManagerConfiguration = reader
        .read_typed_object_field(
            &consensus_mgr_node_id,
            ModuleId::Main,
            ConsensusManagerField::Configuration.field_index(),
        )
        .unwrap();

    let mut config = versioned_config.into_latest();
    config.config.validator_creation_usd_cost = Decimal::from(100);

    let updated_substate = config.into_locked_substate();

    StateUpdates {
        by_node: indexmap!(
            consensus_mgr_node_id => NodeStateUpdates::Delta {
                by_partition: indexmap! {
                    MAIN_BASE_PARTITION => PartitionStateUpdates::Delta {
                        by_substate: indexmap! {
                            SubstateKey::Field(ConsensusManagerField::Configuration.field_index()) => DatabaseUpdate::Set(
                                scrypto_encode(&updated_substate).unwrap()
                            )
                        }
                    },
                }
            }
        ),
    }
}
