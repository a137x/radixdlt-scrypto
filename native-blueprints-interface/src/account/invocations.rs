use super::*;
use crate::resource::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use radix_engine_common::define_type_info_marker;
use radix_engine_common::prelude::*;
use sbor::rust::collections::IndexSet;
use sbor::rust::fmt::Debug;

pub const ACCOUNT_BLUEPRINT: &str = "Account";

define_type_info_marker!(Some(ACCOUNT_PACKAGE), Account);

//=============
// Account Create Advanced
//=============

pub const ACCOUNT_CREATE_ADVANCED_IDENT: &str = "create_advanced";

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct AccountCreateAdvancedInput {
    pub owner_role: OwnerRole,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Eq, PartialEq, ManifestSbor)]
pub struct AccountCreateAdvancedManifestInput {
    pub owner_role: OwnerRole,
    pub address_reservation: Option<ManifestAddressReservation>,
}

pub type AccountCreateAdvancedOutput = Global<AccountObjectTypeInfo>;

//================
// Account Create
//================

pub const ACCOUNT_CREATE_IDENT: &str = "create";

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountCreateInput {}

pub type AccountCreateOutput = (Global<AccountObjectTypeInfo>, Bucket);

//==================
// Account Securify
//==================

pub const ACCOUNT_SECURIFY_IDENT: &str = "securify";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountSecurifyInput {}

pub type AccountSecurifyOutput = Bucket;

//==================
// Account Lock Fee
//==================

pub const ACCOUNT_LOCK_FEE_IDENT: &str = "lock_fee";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountLockFeeInput {
    pub amount: Decimal,
}

pub type AccountLockFeeOutput = ();

//=============================
// Account Lock Contingent Fee
//=============================

pub const ACCOUNT_LOCK_CONTINGENT_FEE_IDENT: &str = "lock_contingent_fee";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountLockContingentFeeInput {
    pub amount: Decimal,
}

pub type AccountLockContingentFeeOutput = ();

//=================
// Account Deposit
//=================

pub const ACCOUNT_DEPOSIT_IDENT: &str = "deposit";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountDepositInput {
    pub bucket: Bucket,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountDepositManifestInput {
    pub bucket: ManifestBucket,
}

pub type AccountDepositOutput = ();

//=======================
// Account Deposit Batch
//=======================

pub const ACCOUNT_DEPOSIT_BATCH_IDENT: &str = "deposit_batch";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountDepositBatchInput {
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountDepositBatchManifestInput {
    pub buckets: Vec<ManifestBucket>,
}

pub type AccountDepositBatchOutput = ();

//============================
// Account Withdraw
//============================

pub const ACCOUNT_WITHDRAW_IDENT: &str = "withdraw";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountWithdrawInput {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
}

pub type AccountWithdrawOutput = Bucket;

//=========================
// Account Withdraw By Ids
//=========================

pub const ACCOUNT_WITHDRAW_NON_FUNGIBLES_IDENT: &str = "withdraw_non_fungibles";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountWithdrawNonFungiblesInput {
    pub resource_address: ResourceAddress,
    pub ids: IndexSet<NonFungibleLocalId>,
}

pub type AccountWithdrawNonFungiblesOutput = Bucket;

//=====================================
// Account Withdraw
//=====================================

pub const ACCOUNT_LOCK_FEE_AND_WITHDRAW_IDENT: &str = "lock_fee_and_withdraw";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountLockFeeAndWithdrawInput {
    pub amount_to_lock: Decimal,
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
}

pub type AccountLockFeeAndWithdrawOutput = Bucket;

//==================================
// Account Withdraw By Ids And Lock
//==================================

pub const ACCOUNT_LOCK_FEE_AND_WITHDRAW_NON_FUNGIBLES_IDENT: &str =
    "lock_fee_and_withdraw_non_fungibles";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountLockFeeAndWithdrawNonFungiblesInput {
    pub amount_to_lock: Decimal,
    pub resource_address: ResourceAddress,
    pub ids: IndexSet<NonFungibleLocalId>,
}

pub type AccountLockFeeAndWithdrawNonFungiblesOutput = Bucket;

//================================
// Account Create Proof By Amount
//================================

pub const ACCOUNT_CREATE_PROOF_OF_AMOUNT_IDENT: &str = "create_proof_of_amount";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountCreateProofOfAmountInput {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
}

pub type AccountCreateProofOfAmountOutput = Proof;

//=============================
// Account Create Proof By Ids
//=============================

pub const ACCOUNT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT: &str = "create_proof_of_non_fungibles";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountCreateProofOfNonFungiblesInput {
    pub resource_address: ResourceAddress,
    pub ids: IndexSet<NonFungibleLocalId>,
}

pub type AccountCreateProofOfNonFungiblesOutput = Proof;

//=================================
// Account Transition Deposit Mode
//=================================

pub const ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT: &str = "set_default_deposit_rule";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountSetDefaultDepositRuleInput {
    pub default: DefaultDepositRule,
}

pub type AccountSetDefaultDepositRuleOutput = ();

//=========================
// Set Resource Preference
//=========================

pub const ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT: &str = "set_resource_preference";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountSetResourcePreferenceInput {
    pub resource_address: ResourceAddress,
    pub resource_preference: ResourcePreference,
}

pub type AccountSetResourcePreferenceOutput = ();

//============================
// Remove Resource Preference
//============================

pub const ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT: &str = "remove_resource_preference";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountRemoveResourcePreferenceInput {
    pub resource_address: ResourceAddress,
}

pub type AccountRemoveResourcePreferenceOutput = ();

//===============================
// Account Try Deposit Or Refund
//===============================

pub const ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT: &str = "try_deposit_or_refund";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountTryDepositOrRefundInput {
    pub bucket: Bucket,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountTryDepositOrRefundManifestInput {
    pub bucket: ManifestBucket,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

pub type AccountTryDepositOrRefundOutput = Option<Bucket>;

//=====================================
// Account Try Deposit Batch Or Refund
//=====================================

pub const ACCOUNT_TRY_DEPOSIT_BATCH_OR_REFUND_IDENT: &str = "try_deposit_batch_or_refund";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountTryDepositBatchOrRefundInput {
    pub buckets: Vec<Bucket>,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountTryDepositBatchOrRefundManifestInput {
    pub buckets: Vec<ManifestBucket>,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

pub type AccountTryDepositBatchOrRefundOutput = Option<Vec<Bucket>>;

//==============================
// Account Try Deposit Or Abort
//==============================

pub const ACCOUNT_TRY_DEPOSIT_OR_ABORT_IDENT: &str = "try_deposit_or_abort";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountTryDepositOrAbortInput {
    pub bucket: Bucket,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountTryDepositOrAbortManifestInput {
    pub bucket: ManifestBucket,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

pub type AccountTryDepositOrAbortOutput = ();

//====================================
// Account Try Deposit Batch Or Abort
//====================================

pub const ACCOUNT_TRY_DEPOSIT_BATCH_OR_ABORT_IDENT: &str = "try_deposit_batch_or_abort";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccountTryDepositBatchOrAbortInput {
    pub buckets: Vec<Bucket>,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccountTryDepositBatchOrAbortManifestInput {
    pub buckets: Vec<ManifestBucket>,
    pub authorized_depositor_badge: Option<ResourceOrNonFungible>,
}

pub type AccountTryDepositBatchOrAbortOutput = ();

//==============
// Account Burn
//==============

pub const ACCOUNT_BURN_IDENT: &str = "burn";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountBurnInput {
    pub resource_address: ResourceAddress,
    pub amount: Decimal,
}

pub type AccountBurnOutput = ();

//=====================
// Account Burn By Ids
//=====================

pub const ACCOUNT_BURN_NON_FUNGIBLES_IDENT: &str = "burn_non_fungibles";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountBurnNonFungiblesInput {
    pub resource_address: ResourceAddress,
    pub ids: IndexSet<NonFungibleLocalId>,
}

pub type AccountBurnNonFungiblesOutput = ();

//==================================
// Account Add Authorized Depositor
//==================================

pub const ACCOUNT_ADD_AUTHORIZED_DEPOSITOR: &str = "add_authorized_depositor";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountAddAuthorizedDepositorInput {
    pub badge: ResourceOrNonFungible,
}

pub type AccountAddAuthorizedDepositorOutput = ();

//=====================================
// Account Remove Authorized Depositor
//=====================================

pub const ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR: &str = "remove_authorized_depositor";

#[derive(Debug, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct AccountRemoveAuthorizedDepositorInput {
    pub badge: ResourceOrNonFungible,
}

pub type AccountRemoveAuthorizedDepositorOutput = ();
