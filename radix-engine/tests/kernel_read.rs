use radix_engine::engine::{KernelError, REActor, RuntimeError};
use radix_engine::ledger::TypedInMemorySubstateStore;
use radix_engine::types::*;
use radix_engine_interface::api::types::RENodeId;
use radix_engine_interface::core::NetworkDefinition;
use radix_engine_interface::data::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn should_not_be_able_to_read_global_substate() {
    // Arrange
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(true, &mut store);
    let (_, _, account) = test_runner.new_allocated_account();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/kernel");

    // Act
    let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "Read",
            "read_global_substate",
            args!(account),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::KernelError(KernelError::InvalidSubstateVisibility {
                actor: REActor::Function(..),
                node_id: RENodeId::Global(GlobalAddress::Component(..)),
                offset: SubstateOffset::Global(GlobalOffset::Global),
                ..
            })
        )
    });
}
