use radix_common::*;
use radix_common::constants::*;
use radix_common::prelude::*;
use radix_engine::blueprints::account::*;
use radix_engine::errors::*;
use radix_engine::updates::*;
use radix_engine_interface::*;
use radix_engine_interface::api::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::prelude::*;
use radix_transactions::builder::*;
use scrypto_test::ledger_simulator::*;

#[test]
fn before_protocol_update_try_deposit_or_refund_fails_if_claimed_authorized_depositor_is_not_one() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new()
        .without_kernel_trace()
        .with_custom_protocol_updates(ProtocolUpdates::none())
        .build();
    let (user_public_key, _, user_account) = ledger.new_account(false);

    // Act
    let receipt = ledger.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .get_free_xrd_from_faucet()
            .call_method(
                user_account,
                ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                AccountSetResourcePreferenceInput {
                    resource_address: XRD,
                    resource_preference: ResourcePreference::Disallowed,
                },
            )
            .take_all_from_worktop(XRD, "bucket")
            .with_bucket("bucket", |builder, bucket| {
                builder.try_deposit_or_refund(
                    user_account,
                    Some(ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE)),
                    bucket,
                )
            })
            .deposit_batch(user_account)
            .build(),
        [&user_public_key].map(NonFungibleGlobalId::from_public_key),
    );

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::AccountError(
                AccountError::NotAnAuthorizedDepositor { .. }
            ))
        )
    })
}

#[test]
fn after_protocol_update_try_deposit_or_refund_refunds_resources_if_claimed_authorized_depositor_is_not_one() {
    // Arrange
    let mut ledger = LedgerSimulatorBuilder::new()
        .without_kernel_trace()
        .with_custom_protocol_updates(ProtocolUpdates::all())
        .build();
    let (user_public_key, _, user_account) = ledger.new_account(false);

    // Act
    let receipt = ledger.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .get_free_xrd_from_faucet()
            .call_method(
                user_account,
                ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
                AccountSetResourcePreferenceInput {
                    resource_address: XRD,
                    resource_preference: ResourcePreference::Disallowed,
                },
            )
            .take_all_from_worktop(XRD, "bucket")
            .with_bucket("bucket", |builder, bucket| {
                builder.try_deposit_or_refund(
                    user_account,
                    Some(ResourceOrNonFungible::Resource(ACCOUNT_OWNER_BADGE)),
                    bucket,
                )
            })
            .deposit_batch(user_account)
            .build(),
        [&user_public_key].map(NonFungibleGlobalId::from_public_key),
    );

    // Assert
    receipt.expect_commit_success();
    assert_eq!(
        ledger.get_component_balance(user_account, XRD),
        dec!(20_000)
    )
}
