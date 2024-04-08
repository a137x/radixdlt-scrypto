use core::ops::*;

use radix_common::*;
use radix_common::constants::*;
use radix_common::data::manifest::*;
use radix_common::data::manifest::model::*;
use radix_common::math::*;
use radix_common::prelude::*;
use radix_engine_interface::{metadata, metadata_init};
use radix_engine_interface::*;
use radix_engine_interface::api::*;
use radix_engine_interface::blueprints::account::ACCOUNT_DEPOSIT_BATCH_IDENT;
use radix_engine_interface::object_modules::ModuleConfig;
use radix_engine_interface::prelude::*;
use radix_engine_tests::common::*;
use radix_transactions::builder::*;
use radix_transactions::model::InstructionV1;
use scrypto_test::ledger_simulator::*;

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

const PACKAGE_ADDRESS_PLACE_HOLDER: [u8; NodeId::LENGTH] = [
    13, 144, 99, 24, 198, 49, 140, 100, 247, 152, 202, 204, 99, 24, 198, 49, 140, 247, 189, 241,
    172, 105, 67, 234, 38, 49, 140, 99, 24, 198,
];

#[test]
fn test_static_package_address() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let package_address1 = ledger.publish_package_simple(PackageLoader::get("static_dependencies"));

    let (mut code, mut definition) = PackageLoader::get("static_dependencies");
    let place_holder: GlobalAddress =
        PackageAddress::new_or_panic(PACKAGE_ADDRESS_PLACE_HOLDER).into();
    for (_, blueprint) in &mut definition.blueprints {
        if blueprint.dependencies.contains(&place_holder) {
            blueprint.dependencies.swap_remove(&place_holder);
            blueprint.dependencies.insert(package_address1.into());
        }
    }

    let start = find_subsequence(&code, &PACKAGE_ADDRESS_PLACE_HOLDER).unwrap();
    code[start..start + PACKAGE_ADDRESS_PLACE_HOLDER.len()]
        .copy_from_slice(package_address1.as_ref());
    let package_address2 =
        ledger.publish_package((code, definition), BTreeMap::new(), OwnerRole::None);

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address2,
            "Sample",
            "call_external_package",
            manifest_args!(),
        )
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn test_static_component_address() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let package_address = ledger.publish_package_simple(PackageLoader::get("static_dependencies"));
    let (key, _priv, account) = ledger.new_account(false);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_standard_test_fee(account)
        .call_function(
            package_address,
            "FaucetCall",
            "call_faucet_lock_fee",
            manifest_args!(),
        )
        .build();
    let receipt =
        ledger.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&key)]);

    // Assert
    receipt.expect_commit_success();
}

const PRE_ALLOCATED: [u8; NodeId::LENGTH] = [
    192, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
];

const PRE_ALLOCATED_PACKAGE: [u8; NodeId::LENGTH] = [
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
];

#[test]
fn static_component_should_be_callable() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let package_address = PackageAddress::new_or_panic(PRE_ALLOCATED_PACKAGE);
    ledger.publish_package_at_address(PackageLoader::get("static_dependencies"), package_address);
    let receipt = ledger.execute_system_transaction(
        vec![InstructionV1::CallFunction {
            package_address: package_address.into(),
            blueprint_name: "Preallocated".to_string(),
            function_name: "new".to_string(),
            args: manifest_args!(ManifestAddressReservation(0), "my_secret".to_string()).into(),
        }],
        btreeset!(),
        vec![(
            BlueprintId::new(&package_address, "Preallocated"),
            GlobalAddress::new_or_panic(PRE_ALLOCATED),
        )
            .into()],
    );
    receipt.expect_commit_success();

    // Act
    let package_address2 = ledger.publish_retain_blueprints(
        PackageLoader::get("static_dependencies2"),
        |blueprint, _| blueprint.eq("PreallocatedCall"),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address2,
            "PreallocatedCall",
            "call_preallocated",
            manifest_args!(),
        )
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    let result = receipt.expect_commit_success();
    let output = result.outcome.expect_success();
    output[1].expect_return_value(&"my_secret".to_string());
}

const PRE_ALLOCATED_RESOURCE: [u8; NodeId::LENGTH] = [
    93, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
];

#[test]
fn static_resource_should_be_callable() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    let (key, _priv, account) = ledger.new_account(false);
    let receipt = ledger.execute_system_transaction(
        vec![
            InstructionV1::CallFunction {
                package_address: RESOURCE_PACKAGE.into(),
                blueprint_name: "FungibleResourceManager".to_string(),
                function_name: "create_with_initial_supply".to_string(),
                args: manifest_decode(
                    &manifest_encode(
                        &FungibleResourceManagerCreateWithInitialSupplyManifestInput {
                            owner_role: OwnerRole::None,
                            track_total_supply: true,
                            divisibility: 0u8,
                            resource_roles: FungibleResourceRoles::default(),
                            metadata: metadata!(),
                            initial_supply: Decimal::from(10),
                            address_reservation: Some(ManifestAddressReservation(0)),
                        },
                    )
                        .unwrap(),
                )
                    .unwrap(),
            },
            InstructionV1::CallMethod {
                address: account.into(),
                method_name: ACCOUNT_DEPOSIT_BATCH_IDENT.to_string(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            },
        ],
        btreeset!(NonFungibleGlobalId::from_public_key(&key)),
        vec![(
            BlueprintId::new(&RESOURCE_PACKAGE, FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT),
            GlobalAddress::new_or_panic(PRE_ALLOCATED_RESOURCE),
        )
            .into()],
    );
    receipt.expect_commit_success();

    // Act
    let package_address2 = ledger.publish_retain_blueprints(
        PackageLoader::get("static_dependencies2"),
        |blueprint, _| blueprint.eq("SomeResource"),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address2,
            "SomeResource",
            "call_some_resource_total_supply",
            manifest_args!(),
        )
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    let result = receipt.expect_commit_success();
    let output = result.outcome.expect_success();
    output[1].expect_return_value(&Decimal::from(10));
}

#[test]
fn static_package_should_be_callable() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new().build();
    ledger.publish_package_at_address(
        PackageLoader::get("static_dependencies"),
        PackageAddress::new_or_panic(PRE_ALLOCATED_PACKAGE),
    );

    // Act
    let package_address2 = ledger.publish_retain_blueprints(
        PackageLoader::get("static_dependencies2"),
        |blueprint, _| blueprint.eq("SomePackage"),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address2,
            "SomePackage",
            "set_package_metadata",
            manifest_args!(),
        )
        .build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}
