use crate::internal_prelude::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::component::*;
use radix_engine_interface::prelude::*;

declare_native_blueprint_state! {
    blueprint_ident: AccountLocker,
    blueprint_snake_case: account_locker,
    features: {},
    fields: {},
    collections: {
        claim_vaults: KeyValue {
            entry_ident: AccountClaims,
            key_type: {
                kind: Static,
                content_type: Global<AccountObjectTypeInfo>,
            },
            value_type: {
                kind: StaticSingleVersioned,
            },
            allow_ownership: true,
        },
    }
}

/// An [`Own`] which is a KeyValueStore<ResourceAddress, Vault>.
pub type AccountLockerAccountClaimsV1 = Own;
