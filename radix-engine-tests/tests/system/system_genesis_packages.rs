use radix_common::*;
use radix_common::constants::*;
use radix_common::data::manifest::*;
use radix_common::data::scrypto::*;
use radix_common::prelude::*;
use radix_engine::errors::{RuntimeError, SystemModuleError};
use radix_engine::kernel::kernel_api::{KernelNodeApi, KernelSubstateApi};
use radix_engine::system::system_callback::SystemLockData;
use radix_engine::system::system_modules::auth::AuthError;
use radix_engine::vm::{OverridePackageCode, VmApi, VmInvoke};
use radix_engine_interface::*;
use radix_engine_interface::api::*;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::package::{
    PACKAGE_CLAIM_ROYALTIES_IDENT, PackageClaimRoyaltiesInput, PackageDefinition,
};
use radix_engine_interface::prelude::*;
use radix_transactions::builder::ManifestBuilder;
use scrypto_test::ledger_simulator::*;

#[test]
fn claiming_royalties_on_native_packages_should_be_unauthorized() {
    const BLUEPRINT_NAME: &str = "MyBlueprint";
    const CUSTOM_PACKAGE_CODE_ID: u64 = 1024;

    // Arrange
    #[derive(Clone)]
    struct TestInvoke;
    impl VmInvoke for TestInvoke {
        fn invoke<Y, V>(
            &mut self,
            export_name: &str,
            _input: &IndexedScryptoValue,
            api: &mut Y,
            _vm_api: &V,
        ) -> Result<IndexedScryptoValue, RuntimeError>
            where
                Y: ClientApi<RuntimeError> + KernelNodeApi + KernelSubstateApi<SystemLockData>,
                V: VmApi,
        {
            match export_name {
                "test" => {
                    api.call_method(
                        PACKAGE_PACKAGE.as_node_id(),
                        PACKAGE_CLAIM_ROYALTIES_IDENT,
                        scrypto_encode(&PackageClaimRoyaltiesInput {}).unwrap(),
                    )?;
                    Ok(IndexedScryptoValue::from_typed(&()))
                }
                _ => Ok(IndexedScryptoValue::from_typed(&())),
            }
        }
    }
    let mut ledger = LedgerSimulatorBuilder::new()
        .with_custom_extension(OverridePackageCode::new(CUSTOM_PACKAGE_CODE_ID, TestInvoke))
        .build();
    let package_address = ledger.publish_native_package(
        CUSTOM_PACKAGE_CODE_ID,
        PackageDefinition::new_functions_only_test_definition(
            BLUEPRINT_NAME,
            vec![("test", "test", false)],
        ),
    );

    // Act
    let receipt = ledger.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(ledger.faucet_component(), 500u32)
            .call_function(package_address, BLUEPRINT_NAME, "test", manifest_args!())
            .build(),
        vec![],
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(AuthError::Unauthorized(
                ..
            )))
        )
    });
}
