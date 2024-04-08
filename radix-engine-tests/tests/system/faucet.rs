use core::ops::*;

use radix_common::*;
use radix_common::math::*;
use radix_common::prelude::*;
use radix_engine_interface::*;
use radix_transactions::builder::*;
use scrypto_test::ledger_simulator::*;

#[test]
fn lock_fee_on_empty_faucet_should_give_nice_error() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new()
        .with_custom_genesis(CustomGenesis::with_faucet_supply(Decimal::ZERO))
        .build();

    // Act
    let manifest = ManifestBuilder::new().lock_fee_from_faucet().build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    let rejection = receipt.expect_rejection();
    assert!(rejection.to_string().contains("The faucet doesn't have funds on this environment. Consider locking fee from an account instead."));
}

#[test]
fn fee_xrd_on_empty_faucet_should_give_nice_error() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new()
        .with_custom_genesis(CustomGenesis::with_faucet_supply(Decimal::ZERO))
        .build();

    // Act
    let manifest = ManifestBuilder::new().get_free_xrd_from_faucet().build();
    let receipt = ledger.execute_manifest(manifest, vec![]);

    // Assert
    let rejection = receipt.expect_rejection();
    assert!(rejection.to_string().contains("The faucet doesn't have funds on this environment. You will need to source XRD another way."));
}
