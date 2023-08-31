use crate::TestFuzzer;
use radix_engine::types::FromRepr;
use radix_engine_common::constants::XRD;
use radix_engine_common::manifest_args;
use radix_engine_common::prelude::{ComponentAddress, NonFungibleLocalId, VALIDATOR_OWNER_BADGE};
use radix_engine_common::types::ResourceAddress;
use radix_engine_interface::blueprints::pool::{
    OneResourcePoolContributeManifestInput, OneResourcePoolGetRedemptionValueManifestInput,
    OneResourcePoolProtectedDepositManifestInput, OneResourcePoolProtectedWithdrawManifestInput,
    OneResourcePoolRedeemManifestInput, ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
    ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT, ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
    ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT, ONE_RESOURCE_POOL_REDEEM_IDENT,
};
use radix_engine_interface::data::manifest::ManifestArgs;
use transaction::builder::ManifestBuilder;
use utils::btreeset;

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromRepr, Ord, PartialOrd, Eq, PartialEq)]
pub enum OnePoolFuzzAction {
    Contribute,
    ProtectedDeposit,
    ProtectedWithdraw,
    Redeem,
    GetRedemptionValue,
}

impl OnePoolFuzzAction {
    pub fn add_to_manifest(
        &self,
        builder: ManifestBuilder,
        fuzzer: &mut TestFuzzer,
        account_address: ComponentAddress,
        pool_address: ComponentAddress,
        pool_unit_resource_address: ResourceAddress,
        resource_address: ResourceAddress,
    ) -> (ManifestBuilder, bool) {
        match self {
            OnePoolFuzzAction::Contribute => {
                let amount = fuzzer.next_amount();

                let builder = builder
                    .mint_fungible(resource_address, amount)
                    .take_all_from_worktop(resource_address, "contribution")
                    .with_name_lookup(|builder, lookup| {
                        builder.call_method(
                            pool_address,
                            ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
                            OneResourcePoolContributeManifestInput {
                                bucket: lookup.bucket("contribution"),
                            },
                        )
                    });

                (builder, amount.is_zero())
            }
            OnePoolFuzzAction::ProtectedDeposit => {
                let amount = fuzzer.next_amount();

                let builder = builder
                    .mint_fungible(resource_address, amount)
                    .take_all_from_worktop(resource_address, "to_deposit")
                    .with_name_lookup(|builder, lookup| {
                        builder.call_method(
                            pool_address,
                            ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
                            OneResourcePoolProtectedDepositManifestInput {
                                bucket: lookup.bucket("to_deposit"),
                            },
                        )
                    });

                (builder, amount.is_zero())
            }
            OnePoolFuzzAction::ProtectedWithdraw => {
                let amount = fuzzer.next_amount();
                let withdraw_strategy = fuzzer.next_withdraw_strategy();

                let builder = builder.call_method(
                    pool_address,
                    ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT,
                    OneResourcePoolProtectedWithdrawManifestInput {
                        amount: amount.into(),
                        withdraw_strategy,
                    },
                );

                (builder, amount.is_zero())
            }
            OnePoolFuzzAction::Redeem => {
                let amount = fuzzer.next_amount();

                let builder = builder
                    .withdraw_from_account(account_address, pool_unit_resource_address, amount)
                    .take_all_from_worktop(pool_unit_resource_address, "pool_unit")
                    .with_name_lookup(|builder, lookup| {
                        builder.call_method(
                            pool_address,
                            ONE_RESOURCE_POOL_REDEEM_IDENT,
                            OneResourcePoolRedeemManifestInput {
                                bucket: lookup.bucket("pool_unit"),
                            },
                        )
                    });

                (builder, amount.is_zero())
            }
            OnePoolFuzzAction::GetRedemptionValue => {
                let amount = fuzzer.next_amount();

                let builder = builder.call_method(
                    pool_address,
                    ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT,
                    OneResourcePoolGetRedemptionValueManifestInput {
                        amount_of_pool_units: amount,
                    },
                );

                (builder, amount.is_zero())
            }
        }
    }
}
