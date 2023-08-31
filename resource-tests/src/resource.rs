use native_sdk::modules::metadata::Metadata;
use native_sdk::modules::role_assignment::RoleAssignment;
use native_sdk::resource::NativeVault;
use radix_engine::errors::RuntimeError;
use radix_engine::kernel::kernel_api::{KernelNodeApi, KernelSubstateApi};
use radix_engine::system::system_callback::SystemLockData;
use crate::TestFuzzer;
use radix_engine::types::FromRepr;
use radix_engine::vm::VmInvoke;
use radix_engine_common::constants::XRD;
use radix_engine_common::manifest_args;
use radix_engine_common::prelude::{ComponentAddress, NonFungibleLocalId, scrypto_decode, scrypto_encode, ScryptoValue, VALIDATOR_OWNER_BADGE};
use radix_engine_common::types::{InternalAddress, ResourceAddress};
use radix_engine_interface::api::{ACTOR_STATE_SELF, ClientApi, LockFlags, ModuleId};
use radix_engine_interface::blueprints::pool::{
    OneResourcePoolContributeManifestInput, OneResourcePoolGetRedemptionValueManifestInput,
    OneResourcePoolProtectedDepositManifestInput, OneResourcePoolProtectedWithdrawManifestInput,
    OneResourcePoolRedeemManifestInput, ONE_RESOURCE_POOL_CONTRIBUTE_IDENT,
    ONE_RESOURCE_POOL_GET_REDEMPTION_VALUE_IDENT, ONE_RESOURCE_POOL_PROTECTED_DEPOSIT_IDENT,
    ONE_RESOURCE_POOL_PROTECTED_WITHDRAW_IDENT, ONE_RESOURCE_POOL_REDEEM_IDENT,
};
use radix_engine_interface::data::manifest::ManifestArgs;
use radix_engine_interface::prelude::{FieldValue, FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT, FungibleResourceManagerMintInput, OwnerRole, Vault, VAULT_PUT_IDENT, VAULT_TAKE_ADVANCED_IDENT, VAULT_TAKE_IDENT};
use radix_engine_interface::types::IndexedScryptoValue;
use transaction::builder::ManifestBuilder;
use utils::{btreemap, btreeset, indexmap};


pub const BLUEPRINT_NAME: &str = "MyBlueprint";
pub const CUSTOM_PACKAGE_CODE_ID: u64 = 1024;

#[derive(Clone)]
pub struct VaultTestInvoke;
impl VmInvoke for VaultTestInvoke {
    fn invoke<Y>(
        &mut self,
        export_name: &str,
        input: &IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: ClientApi<RuntimeError> + KernelNodeApi + KernelSubstateApi<SystemLockData>,
    {
        match export_name {
            "call_vault" => {
                let handle = api
                    .actor_open_field(ACTOR_STATE_SELF, 0u8, LockFlags::read_only())
                    .unwrap();
                let vault: Vault = api.field_read_typed(handle).unwrap();

                let input: (String, ScryptoValue) = scrypto_decode(input.as_slice()).unwrap();

                let rtn = api.call_method(
                    vault.0.as_node_id(),
                    input.0.as_str(),
                    scrypto_encode(&input.1).unwrap(),
                )?;
                return Ok(IndexedScryptoValue::from_vec(rtn).unwrap());
            }
            "new" => {
                let resource_address: (ResourceAddress,) =
                    scrypto_decode(input.as_slice()).unwrap();
                let vault = Vault::create(resource_address.0, api).unwrap();

                let metadata = Metadata::create(api)?;
                let access_rules = RoleAssignment::create(OwnerRole::None, indexmap!(), api)?;
                let node_id = api
                    .new_simple_object(BLUEPRINT_NAME, indexmap!(0u8 => FieldValue::new(&vault)))?;

                api.globalize(
                    node_id,
                    indexmap!(
                        ModuleId::Metadata => metadata.0,
                        ModuleId::RoleAssignment => access_rules.0.0,
                    ),
                    None,
                )?;
            }
            _ => {}
        }

        Ok(IndexedScryptoValue::from_typed(&()))
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromRepr, Ord, PartialOrd, Eq, PartialEq)]
pub enum FungibleResourceFuzzGetBucketAction {
    Mint,
    VaultTake,
    VaultTakeAdvanced,
    VaultRecall,
}

impl FungibleResourceFuzzGetBucketAction {
    pub fn add_to_manifest(
        &self,
        builder: ManifestBuilder,
        fuzzer: &mut TestFuzzer,
        component_address: ComponentAddress,
        resource_address: ResourceAddress,
        vault_id: InternalAddress,
    ) -> (ManifestBuilder, bool) {
        match self {
            FungibleResourceFuzzGetBucketAction::Mint => {
                let amount = fuzzer.next_amount();
                let builder = builder.call_method(
                    resource_address,
                    FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT,
                    FungibleResourceManagerMintInput { amount },
                );
                (builder, amount.is_zero())
            }
            FungibleResourceFuzzGetBucketAction::VaultTake => {
                let amount = fuzzer.next_amount();
                let builder = builder.call_method(
                    component_address,
                    "call_vault",
                    manifest_args!(VAULT_TAKE_IDENT, (amount,)),
                );
                (builder, amount.is_zero())
            }
            FungibleResourceFuzzGetBucketAction::VaultTakeAdvanced => {
                let amount = fuzzer.next_amount();
                let withdraw_strategy = fuzzer.next_withdraw_strategy();
                let builder = builder.call_method(
                    component_address,
                    "call_vault",
                    manifest_args!(VAULT_TAKE_ADVANCED_IDENT, (amount, withdraw_strategy)),
                );
                (builder, amount.is_zero())
            }
            FungibleResourceFuzzGetBucketAction::VaultRecall => {
                let amount = fuzzer.next_amount();
                let builder = builder.recall(vault_id, amount);
                (builder, amount.is_zero())
            }
        }
    }
}


#[repr(u8)]
#[derive(Copy, Clone, Debug, FromRepr, Ord, PartialOrd, Eq, PartialEq)]
pub enum FungibleResourceFuzzUseBucketAction {
    Burn,
    VaultPut,
}

impl FungibleResourceFuzzUseBucketAction {
    pub fn add_to_manifest(
        &self,
        builder: ManifestBuilder,
        fuzzer: &mut TestFuzzer,
        resource_address: ResourceAddress,
        component_address: ComponentAddress,
    ) -> (ManifestBuilder, bool) {
        match self {
            FungibleResourceFuzzUseBucketAction::Burn => {
                let amount = fuzzer.next_amount();
                let builder = builder
                    .take_from_worktop(resource_address, amount, "bucket")
                    .burn_resource("bucket");

                (builder, amount.is_zero())
            }
            FungibleResourceFuzzUseBucketAction::VaultPut => {
                let amount = fuzzer.next_amount();
                let builder = builder
                    .take_from_worktop(resource_address, amount, "bucket")
                    .with_bucket("bucket", |builder, bucket| {
                        builder.call_method(
                            component_address,
                            "call_vault",
                            manifest_args!(VAULT_PUT_IDENT, (bucket,)),
                        )
                    });
                (builder, amount.is_zero())
            }
        }
    }
}