mod package_loader;

use package_loader::*;
use radix_engine::errors::*;
use radix_engine::system::system_type_checker::*;
use radix_engine::track::*;
use radix_engine::types::blueprints::package::VmType::*;
use radix_engine::types::*;
use radix_engine_queries::typed_substate_layout::*;
use radix_engine_store_interface::db_key_mapper::*;
use radix_engine_store_interface::interface::*;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn test_add_direct_access_ref_to_stored_substate_external_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();

    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);
    let vault_id = test_runner
        .get_component_vaults(account, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "ReferenceTest", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    // Call method
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_stored_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string().contains("RefCantBeAddedToSubstate")
            && e.to_string().contains(&hex::encode(vault_id.as_bytes()))
    });
}

#[test]
fn test_add_direct_access_ref_to_heap_substate_external_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();

    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);
    let vault_id = test_runner
        .get_component_vaults(account, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "ReferenceTest", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    // Call method
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_heap_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string().contains("RefCantBeAddedToSubstate")
            && e.to_string().contains(&hex::encode(vault_id.as_bytes()))
    });
}

#[test]
fn test_add_direct_access_ref_to_kv_store_substate_external_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();

    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);
    let vault_id = test_runner
        .get_component_vaults(account, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "ReferenceTest", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    // Call method
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_kv_store_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string()
            .contains("Non Global Reference is not allowed")
    });
}

#[test]
fn test_add_direct_access_ref_to_stored_substate_internal_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(account, resource, dec!(1))
            .take_all_from_worktop(resource, "bucket")
            .call_function_with_name_lookup(
                package_address,
                "ReferenceTest",
                "new_with_bucket",
                |lookup| manifest_args!(lookup.bucket("bucket")),
            )
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    let vault_id = test_runner
        .get_component_vaults(component_address, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Call function
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_stored_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string().contains("NonGlobalRefNotAllowed")
            && e.to_string().contains(&hex::encode(vault_id.as_bytes()))
    });
}

#[test]
fn test_add_direct_access_ref_to_heap_substate_internal_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(account, resource, dec!(1))
            .take_all_from_worktop(resource, "bucket")
            .call_function_with_name_lookup(
                package_address,
                "ReferenceTest",
                "new_with_bucket",
                |lookup| manifest_args!(lookup.bucket("bucket")),
            )
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    let vault_id = test_runner
        .get_component_vaults(component_address, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Call function
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_heap_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string().contains("RefCantBeAddedToSubstate")
            && e.to_string().contains(&hex::encode(vault_id.as_bytes()))
    });
}

#[test]
fn test_add_direct_access_ref_to_kv_store_substate_internal_vault() {
    // Basic setup
    let (mut test_runner, package_address) = flash_package();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let resource = test_runner.create_recallable_token(account);

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(account, resource, dec!(1))
            .take_all_from_worktop(resource, "bucket")
            .call_function_with_name_lookup(
                package_address,
                "ReferenceTest",
                "new_with_bucket",
                |lookup| manifest_args!(lookup.bucket("bucket")),
            )
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    let vault_id = test_runner
        .get_component_vaults(component_address, resource)
        .pop()
        .unwrap();
    println!("Recallable vault id: {:?}", vault_id);

    // Call function
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_direct_access_ref_to_kv_store_substate",
                manifest_args!(InternalAddress::try_from(vault_id).unwrap()),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("{:?}", receipt);

    // Assert
    receipt.expect_specific_failure(|e| {
        e.to_string()
            .contains("Non Global Reference is not allowed")
    });
}

#[test]
fn test_create_global_node_with_local_ref() {
    // Basic setup
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key, _, account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.publish_package_simple(PackageLoader::get("reference"));

    // Call function
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_function(
                package_address,
                "ReferenceTest",
                "create_global_node_with_local_ref",
                manifest_args!(),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| match e {
        RuntimeError::SystemError(SystemError::TypeCheckError(
            TypeCheckError::BlueprintPayloadValidationError(.., error),
        )) => error.contains("Non Global Reference"),
        _ => false,
    });
}

#[test]
fn test_add_local_ref_to_stored_substate() {
    // Basic setup
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key, _, account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.publish_package_simple(PackageLoader::get("reference"));

    // Instantiate component
    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "ReferenceTest", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    // Call method
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_standard_test_fee(account)
            .call_method(
                component_address,
                "add_local_ref_to_stored_substate",
                manifest_args!(),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| match e {
        RuntimeError::SystemError(SystemError::TypeCheckError(
            TypeCheckError::BlueprintPayloadValidationError(.., error),
        )) => error.contains("Non Global Reference"),
        _ => false,
    });
}

fn flash_package() -> (DefaultTestRunner, PackageAddress) {
    let mut test_runner = TestRunnerBuilder::new().build();
    let package_address = {
        let mut bytes = ACCOUNT_PACKAGE.as_node_id().0;
        *bytes.last_mut().unwrap() = 0xFF;
        PackageAddress::new_or_panic(bytes)
    };

    let (code, definition) = PackageLoader::get("reference");
    let mut package_structure = PackageNativePackage::validate_and_build_package_structure(
        definition,
        ScryptoV1,
        code,
        Default::default(),
    )
    .unwrap();
    package_structure
        .definitions
        .values_mut()
        .for_each(|def| match def.as_mut() {
            VersionedPackageBlueprintVersionDefinition::V1(def) => {
                def.interface.is_transient = true
            }
        });
    let partitions = create_bootstrap_package_partitions(package_structure, Default::default());
    let to_flash = partitions
        .into_iter()
        .map(|(partition_number, substates)| {
            (
                (package_address.into_node_id(), partition_number),
                substates,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let database_updates = construct_database_updates(to_flash);
    test_runner.substate_db_mut().commit(&database_updates);

    (test_runner, package_address)
}

fn construct_database_updates(
    substate_flash: BTreeMap<(NodeId, PartitionNumber), BTreeMap<SubstateKey, IndexedScryptoValue>>,
) -> DatabaseUpdates {
    let mut system_updates = index_map_new();
    for ((node_id, partition_num), substates) in substate_flash {
        let mut substate_updates = index_map_new();
        for (substate_key, value) in substates {
            substate_updates.insert(substate_key, DatabaseUpdate::Set(value.as_slice().to_vec()));
        }
        system_updates.insert((node_id, partition_num), substate_updates);
    }
    let state_updates = StateUpdates::from(LegacyStateUpdates {
        partition_deletions: index_set_new(),
        system_updates,
    });
    state_updates.create_database_updates::<SpreadPrefixKeyMapper>()
}
