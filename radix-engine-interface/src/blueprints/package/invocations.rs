use crate::api::node_modules::metadata::{
    METADATA_GET_IDENT, METADATA_REMOVE_IDENT, METADATA_SET_IDENT,
};
use crate::blueprints::resource::*;
use crate::types::*;
use crate::*;
use radix_engine_common::data::manifest::model::ManifestAddressReservation;
use radix_engine_common::data::manifest::model::ManifestBlobRef;
use radix_engine_interface::api::node_modules::metadata::MetadataValue;
#[cfg(not(feature = "indexmap"))]
use sbor::rust::collections::BTreeMap;
use sbor::rust::collections::BTreeSet;
use sbor::rust::string::String;
use sbor::rust::vec::Vec;
use scrypto_schema::BlueprintSchemaInit;

pub const PACKAGE_BLUEPRINT: &str = "Package";

pub const PACKAGE_PUBLISH_WASM_IDENT: &str = "publish_wasm";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct PackagePublishWasmInput {
    pub code: Vec<u8>,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, ManifestSbor)]
pub struct PackagePublishWasmManifestInput {
    pub code: ManifestBlobRef,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
}

pub type PackagePublishWasmOutput = (PackageAddress, Bucket);

pub const PACKAGE_PUBLISH_WASM_ADVANCED_IDENT: &str = "publish_wasm_advanced";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct PackagePublishWasmAdvancedInput {
    pub package_address: Option<GlobalAddressReservation>,
    pub code: Vec<u8>,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
    pub owner_rule: OwnerRole,
}

#[derive(Debug, Clone, Eq, PartialEq, ManifestSbor)]
pub struct PackagePublishWasmAdvancedManifestInput {
    pub package_address: Option<ManifestAddressReservation>,
    pub code: ManifestBlobRef,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
    pub owner_rule: OwnerRole,
}

pub type PackagePublishWasmAdvancedOutput = PackageAddress;

pub const PACKAGE_PUBLISH_NATIVE_IDENT: &str = "publish_native";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct PackagePublishNativeInput {
    pub package_address: Option<GlobalAddressReservation>,
    pub native_package_code_id: u8,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, ManifestSbor)]
pub struct PackagePublishNativeManifestInput {
    pub package_address: Option<ManifestAddressReservation>,
    pub native_package_code_id: u8,
    pub setup: PackageDefinition,
    #[cfg(feature = "indexmap")]
    pub metadata: sbor::prelude::IndexMap<String, MetadataValue>,
    #[cfg(not(feature = "indexmap"))]
    pub metadata: BTreeMap<String, MetadataValue>,
}

pub type PackagePublishNativeOutput = PackageAddress;

pub const PACKAGE_CLAIM_ROYALTIES_IDENT: &str = "PackageRoyalty_claim_royalties";

#[derive(
    Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestCategorize, ManifestEncode, ManifestDecode,
)]
pub struct PackageClaimRoyaltiesInput {}

pub type PackageClaimRoyaltiesOutput = Bucket;

#[derive(Debug, Clone, Eq, PartialEq, Default, ScryptoSbor, ManifestSbor)]
pub struct PackageDefinition {
    #[cfg(feature = "indexmap")]
    pub blueprints: sbor::prelude::IndexMap<String, BlueprintDefinitionInit>,
    #[cfg(not(feature = "indexmap"))]
    pub blueprints: BTreeMap<String, BlueprintDefinitionInit>,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct BlueprintDefinitionInit {
    pub outer_blueprint: Option<String>,
    pub feature_set: BTreeSet<String>,
    pub dependencies: BTreeSet<GlobalAddress>,

    pub schema: BlueprintSchemaInit,

    pub royalty_config: RoyaltyConfig,
    pub auth_config: AuthConfig,
}

impl Default for BlueprintDefinitionInit {
    fn default() -> Self {
        Self {
            outer_blueprint: None,
            dependencies: BTreeSet::default(),
            feature_set: BTreeSet::default(),
            schema: BlueprintSchemaInit::default(),
            royalty_config: RoyaltyConfig::default(),
            auth_config: AuthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ScryptoSbor, ManifestSbor)]
pub struct AuthConfig {
    #[cfg(feature = "indexmap")]
    pub function_auth: sbor::prelude::IndexMap<String, AccessRule>,
    #[cfg(not(feature = "indexmap"))]
    pub function_auth: BTreeMap<String, AccessRule>,
    pub method_auth: MethodAuthTemplate,
}

#[cfg(not(feature = "indexmap"))]
type Auth = BTreeMap<MethodKey, MethodPermission>;
#[cfg(feature = "indexmap")]
type Auth = sbor::prelude::IndexMap<MethodKey, MethodPermission>;

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub enum MethodAuthTemplate {
    Static {
        #[cfg(feature = "indexmap")]
        auth: Auth,
        #[cfg(not(feature = "indexmap"))]
        auth: Auth,
        #[cfg(feature = "indexmap")]
        outer_auth: Auth,
        #[cfg(not(feature = "indexmap"))]
        outer_auth: Auth,
    },
}

impl MethodAuthTemplate {
    pub fn add_metadata_default_if_not_specified(&mut self) {
        match self {
            MethodAuthTemplate::Static { auth, .. } => {
                if !auth.contains_key(&MethodKey::metadata(METADATA_GET_IDENT)) {
                    auth.insert(
                        MethodKey::metadata(METADATA_GET_IDENT),
                        MethodPermission::Public,
                    );
                    auth.insert(MethodKey::metadata(METADATA_SET_IDENT), [OWNER_ROLE].into());
                    auth.insert(
                        MethodKey::metadata(METADATA_REMOVE_IDENT),
                        [OWNER_ROLE].into(),
                    );
                }
            }
        }
    }

    pub fn auth(self) -> Auth {
        match self {
            MethodAuthTemplate::Static { auth, .. } => auth,
        }
    }

    pub fn outer_auth(self) -> Auth {
        match self {
            MethodAuthTemplate::Static { outer_auth, .. } => outer_auth,
        }
    }
}

impl Default for MethodAuthTemplate {
    #[cfg(feature = "indexmap")]
    fn default() -> Self {
        MethodAuthTemplate::Static {
            auth: sbor::prelude::index_map::new(),
            outer_auth: sbor::prelude::index_map::new(),
        }
    }

    #[cfg(not(feature = "indexmap"))]
    fn default() -> Self {
        MethodAuthTemplate::Static {
            auth: Auth::default(),
            outer_auth: Auth::default(),
        }
    }
}
