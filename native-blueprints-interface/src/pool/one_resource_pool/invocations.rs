use crate::macros::*;
use crate::resource::*;
use radix_engine_common::data::manifest::model::*;
use radix_engine_common::define_type_info_marker;
use radix_engine_common::math::*;
use radix_engine_common::prelude::*;
use radix_engine_common::markers::*;
use radix_engine_common::*;

define_type_info_marker!(Some(POOL_PACKAGE), OneResourcePool);

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: instantiate,
    input: struct {
        owner_role: OwnerRole,
        pool_manager_rule: AccessRule,
        resource_address: ResourceAddress,
        address_reservation: Option<GlobalAddressReservation>
    },
    output: type Global<OneResourcePoolObjectTypeInfo>,
    manifest_input: struct {
        owner_role: OwnerRole,
        pool_manager_rule: AccessRule,
        resource_address: ResourceAddress,
        address_reservation: Option<ManifestAddressReservation>
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: contribute,
    input: struct {
        bucket: Bucket
    },
    output: type Bucket,
    manifest_input: struct {
        bucket: ManifestBucket
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: redeem,
    input: struct {
        bucket: Bucket
    },
    output: type Bucket,
    manifest_input: struct {
        bucket: ManifestBucket
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: protected_deposit,
    input: struct {
        bucket: Bucket
    },
    output: type (),
    manifest_input: struct {
        bucket: ManifestBucket
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: protected_withdraw,
    input: struct {
        amount: Decimal,
        withdraw_strategy: WithdrawStrategy
    },
    output: type Bucket,
    manifest_input: struct {
        amount: Decimal,
        withdraw_strategy: WithdrawStrategy
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: get_redemption_value,
    input: struct {
        amount_of_pool_units: Decimal
    },
    output: type Decimal,
    manifest_input: struct {
        amount_of_pool_units: Decimal
    }
}

define_invocation! {
    blueprint_name: OneResourcePool,
    function_name: get_vault_amount,
    input: struct {},
    output: type Decimal,
    manifest_input: struct {}
}
