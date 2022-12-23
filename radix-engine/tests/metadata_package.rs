use radix_engine::engine::{AuthError, ModuleError, RuntimeError};
use radix_engine::ledger::TypedInMemorySubstateStore;
use radix_engine::types::*;
use radix_engine_interface::core::NetworkDefinition;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn cannot_set_package_metadata_with_no_owner() {
    // Arrange
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);
    let code = wat2wasm(include_str!("wasm/basic_package.wat"));
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .publish_package(
            code,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            AccessRules::new(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let package_address = receipt.expect_commit().entity_changes.new_package_addresses[0];

    // Act
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_native_method(
            RENodeId::Global(GlobalAddress::Package(package_address)),
            "set",
            scrypto_encode(&MetadataSetInvocation {
                receiver: RENodeId::Global(GlobalAddress::Package(package_address)),
                key: "name".to_string(),
                value: "best package ever!".to_string(),
            })
            .unwrap(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ModuleError(ModuleError::AuthError(AuthError::Unauthorized { .. }))
        )
    });
    let metadata = test_runner.get_metadata(GlobalAddress::Package(package_address));
    assert!(metadata.get("name").is_none());
}

#[test]
fn can_set_package_metadata_with_owner() {
    // Arrange
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);
    let code = wat2wasm(include_str!("wasm/basic_package.wat"));
    let (public_key, _, account) = test_runner.new_account(false);
    let owner_badge_resource = test_runner.create_non_fungible_resource(account);
    let owner_badge_addr = NonFungibleAddress::new(owner_badge_resource, NonFungibleId::U32(1));
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .publish_package_with_owner(code, HashMap::new(), owner_badge_addr)
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let package_address = receipt.expect_commit().entity_changes.new_package_addresses[0];

    // Act
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .create_proof_from_account(account, owner_badge_resource)
        .call_native_method(
            RENodeId::Global(GlobalAddress::Package(package_address)),
            "set",
            scrypto_encode(&MetadataSetInvocation {
                receiver: RENodeId::Global(GlobalAddress::Package(package_address)),
                key: "name".to_string(),
                value: "best package ever!".to_string(),
            })
            .unwrap(),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleAddress::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_commit_success();
    let metadata = test_runner.get_metadata(GlobalAddress::Package(package_address));
    assert_eq!(metadata.get("name").unwrap(), "best package ever!");
}

#[test]
fn can_lock_package_metadata_with_owner() {
    // Arrange
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);
    let code = wat2wasm(include_str!("wasm/basic_package.wat"));
    let (public_key, _, account) = test_runner.new_account(false);
    let owner_badge_resource = test_runner.create_non_fungible_resource(account);
    let owner_badge_addr = NonFungibleAddress::new(owner_badge_resource, NonFungibleId::U32(1));
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .publish_package_with_owner(code, HashMap::new(), owner_badge_addr)
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let package_address = receipt.expect_commit().entity_changes.new_package_addresses[0];

    // Act
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .create_proof_from_account(account, owner_badge_resource)
        .call_native_method(
            RENodeId::Global(GlobalAddress::Package(package_address)),
            &AccessRulesChainMethod::SetMethodAccessRule.to_string(),
            scrypto_encode(&AccessRulesSetMethodAccessRuleInvocation {
                receiver: RENodeId::Global(GlobalAddress::Package(package_address)),
                index: 0,
                key: AccessRuleKey::Native(NativeFn::Method(NativeMethod::Metadata(
                    MetadataMethod::Set,
                ))),
                rule: AccessRule::DenyAll,
            })
            .unwrap(),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleAddress::from_public_key(&public_key)],
    );
    receipt.expect_commit_success();

    // Act
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .create_proof_from_account(account, owner_badge_resource)
        .call_native_method(
            RENodeId::Global(GlobalAddress::Package(package_address)),
            "set",
            scrypto_encode(&MetadataSetInvocation {
                receiver: RENodeId::Global(GlobalAddress::Package(package_address)),
                key: "name".to_string(),
                value: "best package ever!".to_string(),
            })
            .unwrap(),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleAddress::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ModuleError(ModuleError::AuthError(AuthError::Unauthorized { .. }))
        )
    });
    let metadata = test_runner.get_metadata(GlobalAddress::Package(package_address));
    assert!(metadata.get("name").is_none());
}
