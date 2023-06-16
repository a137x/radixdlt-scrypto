use crate::api::ObjectModuleId;
use crate::blueprints::resource::*;
use crate::rule;
use crate::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use sbor::rust::collections::BTreeMap;
use sbor::rust::str;
use sbor::rust::string::String;
use sbor::rust::string::ToString;
use sbor::rust::vec;
use sbor::rust::vec::Vec;
#[cfg(not(feature = "indexmap"))]
use utils::btreemap;

use super::AccessRule;

pub const SELF_ROLE: &'static str = "_self_";
pub const OWNER_ROLE: &'static str = "_owner_";

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub enum ObjectKey {
    SELF,
    InnerBlueprint(String),
}

impl ObjectKey {
    pub fn inner_blueprint(name: &str) -> Self {
        ObjectKey::InnerBlueprint(name.to_string())
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub struct MethodKey {
    // TODO: Remove `ObjectModuleId::AccessRules`?
    pub module_id: ObjectModuleId,
    pub ident: String,
}

impl MethodKey {
    pub fn new<S: ToString>(module_id: ObjectModuleId, method_ident: S) -> Self {
        Self {
            module_id,
            ident: method_ident.to_string(),
        }
    }

    pub fn metadata<S: ToString>(method_ident: S) -> Self {
        Self {
            module_id: ObjectModuleId::Metadata,
            ident: method_ident.to_string(),
        }
    }

    pub fn royalty<S: ToString>(method_ident: S) -> Self {
        Self {
            module_id: ObjectModuleId::Royalty,
            ident: method_ident.to_string(),
        }
    }

    pub fn main<S: ToString>(method_ident: S) -> Self {
        Self {
            module_id: ObjectModuleId::Main,
            ident: method_ident.to_string(),
        }
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub struct MethodEntry {
    pub permission: MethodPermission,
}

impl MethodEntry {
    pub fn new<P: Into<MethodPermission>>(permission: P) -> Self {
        Self {
            permission: permission.into(),
        }
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub enum MethodPermission {
    Public,
    Protected(RoleList),
}

impl MethodPermission {
    pub fn nobody() -> Self {
        MethodPermission::Protected(RoleList::none())
    }
}

impl<const N: usize> From<[&str; N]> for MethodPermission {
    fn from(value: [&str; N]) -> Self {
        MethodPermission::Protected(value.into())
    }
}

impl From<RoleList> for MethodPermission {
    fn from(value: RoleList) -> Self {
        Self::Protected(value)
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub enum AttachedModule {
    Metadata,
    Royalty,
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
#[sbor(transparent)]
pub struct RoleKey {
    pub key: String,
}

impl From<String> for RoleKey {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for RoleKey {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl RoleKey {
    pub fn new<S: Into<String>>(key: S) -> Self {
        RoleKey { key: key.into() }
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
pub struct RoleEntry {
    pub rule: AccessRule,
    pub mutable: RoleList,
    pub mutable_mutable: bool,
}

impl RoleEntry {
    pub fn new<A: Into<AccessRule>, M: Into<RoleList>>(
        rule: A,
        mutable: M,
        mutable_mutable: bool,
    ) -> Self {
        Self {
            rule: rule.into(),
            mutable: mutable.into(),
            mutable_mutable,
        }
    }

    pub fn immutable<A: Into<AccessRule>>(rule: A) -> Self {
        Self {
            rule: rule.into(),
            mutable: RoleList::none(),
            mutable_mutable: false,
        }
    }

    pub fn disabled() -> Self {
        Self::immutable(AccessRule::DenyAll)
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, ScryptoSbor, ManifestSbor)]
#[sbor(transparent)]
pub struct RoleList {
    pub list: Vec<RoleKey>,
}

impl RoleList {
    pub fn none() -> Self {
        Self { list: vec![] }
    }

    pub fn insert<R: Into<RoleKey>>(&mut self, role: R) {
        self.list.push(role.into());
    }

    pub fn to_list(self) -> Vec<String> {
        self.list.into_iter().map(|k| k.key).collect()
    }
}

impl From<Vec<&str>> for RoleList {
    fn from(value: Vec<&str>) -> Self {
        Self {
            list: value.into_iter().map(|s| RoleKey::new(s)).collect(),
        }
    }
}

impl From<Vec<String>> for RoleList {
    fn from(value: Vec<String>) -> Self {
        Self {
            list: value.into_iter().map(|s| RoleKey::new(s)).collect(),
        }
    }
}

impl<const N: usize> From<[&str; N]> for RoleList {
    fn from(value: [&str; N]) -> Self {
        Self {
            list: value.into_iter().map(|s| RoleKey::new(s)).collect(),
        }
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, ScryptoSbor, ManifestSbor)]
pub enum OwnerRole {
    None,
    Fixed(AccessRule),
    Updateable(AccessRule),
}

impl OwnerRole {
    pub fn to_role_entry(self, owner_role_name: &str) -> RoleEntry {
        match self {
            OwnerRole::Fixed(rule) => RoleEntry::immutable(rule),
            OwnerRole::Updateable(rule) => RoleEntry::new(rule, [owner_role_name], false),
            OwnerRole::None => RoleEntry::immutable(AccessRule::DenyAll),
        }
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor, ManifestSbor)]
#[sbor(transparent)]
pub struct Roles {
    #[cfg(feature = "indexmap")]
    pub roles: sbor::prelude::IndexMap<RoleKey, RoleEntry>,
    #[cfg(not(feature = "indexmap"))]
    pub roles: BTreeMap<RoleKey, RoleEntry>,
}

impl Roles {
    #[cfg(feature = "indexmap")]
    pub fn new() -> Self {
        Self {
            roles: sbor::prelude::index_map::new(),
        }
    }

    #[cfg(not(feature = "indexmap"))]
    pub fn new() -> Self {
        Self { roles: btreemap!() }
    }

    pub fn define_role<K: Into<RoleKey>>(&mut self, authority: K, entry: RoleEntry) {
        self.roles.insert(authority.into(), entry);
    }
}

// TODO: Remove?
pub fn resource_access_rules_from_owner_badge(
    owner_badge: &NonFungibleGlobalId,
) -> BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)> {
    let mut access_rules = BTreeMap::new();
    access_rules.insert(
        ResourceMethodAuthKey::Withdraw,
        (AccessRule::AllowAll, rule!(require(owner_badge.clone()))),
    );
    access_rules.insert(
        ResourceMethodAuthKey::Deposit,
        (AccessRule::AllowAll, rule!(require(owner_badge.clone()))),
    );
    access_rules.insert(
        ResourceMethodAuthKey::Recall,
        (AccessRule::DenyAll, rule!(require(owner_badge.clone()))),
    );
    access_rules.insert(
        Mint,
        (AccessRule::DenyAll, rule!(require(owner_badge.clone()))),
    );
    access_rules.insert(
        Burn,
        (AccessRule::DenyAll, rule!(require(owner_badge.clone()))),
    );
    access_rules.insert(
        UpdateNonFungibleData,
        (
            rule!(require(owner_badge.clone())),
            rule!(require(owner_badge.clone())),
        ),
    );
    access_rules.insert(
        UpdateMetadata,
        (
            rule!(require(owner_badge.clone())),
            rule!(require(owner_badge.clone())),
        ),
    );
    access_rules
}
