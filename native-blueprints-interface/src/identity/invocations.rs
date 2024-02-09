use crate::resource::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use radix_engine_common::define_type_info_marker;
use radix_engine_common::prelude::*;
use sbor::rust::fmt::Debug;

pub const IDENTITY_BLUEPRINT: &str = "Identity";

define_type_info_marker!(Some(IDENTITY_PACKAGE), Identity);

pub const IDENTITY_CREATE_ADVANCED_IDENT: &str = "create_advanced";

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct IdentityCreateAdvancedInput {
    pub owner_role: OwnerRole,
}

pub type IdentityCreateAdvancedOutput = Global<IdentityObjectTypeInfo>;

pub const IDENTITY_CREATE_IDENT: &str = "create";

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct IdentityCreateInput {}

pub type IdentityCreateOutput = (Global<IdentityObjectTypeInfo>, Bucket);

pub const IDENTITY_SECURIFY_IDENT: &str = "securify";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct IdentitySecurifyToSingleBadgeInput {}

pub type IdentitySecurifyToSingleBadgeOutput = Bucket;
