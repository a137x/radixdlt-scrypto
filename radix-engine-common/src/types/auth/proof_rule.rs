use self::AccessRuleNode::{AllOf, AnyOf};
use crate::internal_prelude::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;

#[cfg_attr(
    feature = "radix_engine_fuzzing",
    derive(Arbitrary, serde::Serialize, serde::Deserialize)
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    ManifestSbor,
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
)]
pub enum ResourceOrNonFungible {
    NonFungible(NonFungibleGlobalId),
    Resource(ResourceAddress),
}

impl Describe<ScryptoCustomTypeKind> for ResourceOrNonFungible {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::RESOURCE_OR_NON_FUNGIBLE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::resource_or_non_fungible_type_data()
    }
}

impl From<NonFungibleGlobalId> for ResourceOrNonFungible {
    fn from(non_fungible_global_id: NonFungibleGlobalId) -> Self {
        ResourceOrNonFungible::NonFungible(non_fungible_global_id)
    }
}

impl From<ResourceAddress> for ResourceOrNonFungible {
    fn from(resource_address: ResourceAddress) -> Self {
        ResourceOrNonFungible::Resource(resource_address)
    }
}

pub struct ResourceOrNonFungibleList {
    list: Vec<ResourceOrNonFungible>,
}

impl<T> From<Vec<T>> for ResourceOrNonFungibleList
where
    T: Into<ResourceOrNonFungible>,
{
    fn from(addresses: Vec<T>) -> Self {
        ResourceOrNonFungibleList {
            list: addresses.into_iter().map(|a| a.into()).collect(),
        }
    }
}

/// Resource Proof Rules
#[cfg_attr(
    feature = "radix_engine_fuzzing",
    derive(Arbitrary, serde::Serialize, serde::Deserialize)
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    ManifestSbor,
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
)]
pub enum ProofRule {
    Require(ResourceOrNonFungible),
    AmountOf(Decimal, ResourceAddress),
    CountOf(u8, Vec<ResourceOrNonFungible>),
    AllOf(Vec<ResourceOrNonFungible>),
    AnyOf(Vec<ResourceOrNonFungible>),
}

impl Describe<ScryptoCustomTypeKind> for ProofRule {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::PROOF_RULE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::proof_rule_type_data()
    }
}

impl From<ResourceAddress> for AccessRuleNode {
    fn from(resource_address: ResourceAddress) -> Self {
        AccessRuleNode::ProofRule(ProofRule::Require(resource_address.into()))
    }
}

impl From<NonFungibleGlobalId> for AccessRuleNode {
    fn from(id: NonFungibleGlobalId) -> Self {
        AccessRuleNode::ProofRule(ProofRule::Require(id.into()))
    }
}

impl From<ResourceOrNonFungible> for AccessRuleNode {
    fn from(resource_or_non_fungible: ResourceOrNonFungible) -> Self {
        AccessRuleNode::ProofRule(ProofRule::Require(resource_or_non_fungible))
    }
}

#[cfg_attr(
    feature = "radix_engine_fuzzing",
    derive(Arbitrary, serde::Serialize, serde::Deserialize)
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    ManifestSbor,
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
)]
pub enum AccessRuleNode {
    ProofRule(ProofRule),
    AnyOf(Vec<AccessRuleNode>),
    AllOf(Vec<AccessRuleNode>),
}

impl Describe<ScryptoCustomTypeKind> for AccessRuleNode {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::ACCESS_RULE_NODE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::access_rule_node_type_data()
    }
}

impl AccessRuleNode {
    pub fn or(self, other: AccessRuleNode) -> Self {
        match self {
            AccessRuleNode::AnyOf(mut rules) => {
                rules.push(other);
                AnyOf(rules)
            }
            _ => AnyOf(vec![self, other]),
        }
    }

    pub fn and(self, other: AccessRuleNode) -> Self {
        match self {
            AccessRuleNode::AllOf(mut rules) => {
                rules.push(other);
                AllOf(rules)
            }
            _ => AllOf(vec![self, other]),
        }
    }
}

/// A requirement for the immediate caller's package to equal the given package.
pub fn package_of_direct_caller(package: PackageAddress) -> ResourceOrNonFungible {
    ResourceOrNonFungible::NonFungible(NonFungibleGlobalId::package_of_direct_caller_badge(package))
}

/// A requirement for the global ancestor of the actor who made the latest global call to either be:
/// * The main module of the given global component (pass a `ComponentAddress` or `GlobalAddress`)
/// * A package function on the given blueprint (pass `(PackageAddress, String)` or `Blueprint`)
pub fn global_caller(global_caller: impl Into<GlobalCaller>) -> ResourceOrNonFungible {
    ResourceOrNonFungible::NonFungible(NonFungibleGlobalId::global_caller_badge(global_caller))
}

pub fn require<T>(required: T) -> AccessRuleNode
where
    T: Into<AccessRuleNode>,
{
    required.into()
}

pub fn require_any_of<T>(resources: T) -> AccessRuleNode
where
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    AccessRuleNode::ProofRule(ProofRule::AnyOf(list.list))
}

pub fn require_all_of<T>(resources: T) -> AccessRuleNode
where
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    AccessRuleNode::ProofRule(ProofRule::AllOf(list.list))
}

pub fn require_n_of<C, T>(count: C, resources: T) -> AccessRuleNode
where
    C: Into<u8>,
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    AccessRuleNode::ProofRule(ProofRule::CountOf(count.into(), list.list))
}

pub fn require_amount<D, T>(amount: D, resource: T) -> AccessRuleNode
where
    D: Into<Decimal>,
    T: Into<ResourceAddress>,
{
    AccessRuleNode::ProofRule(ProofRule::AmountOf(amount.into(), resource.into()))
}

#[cfg_attr(
    feature = "radix_engine_fuzzing",
    derive(Arbitrary, serde::Serialize, serde::Deserialize)
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    ManifestSbor,
    ScryptoCategorize,
    ScryptoEncode,
    ScryptoDecode,
)]
pub enum AccessRule {
    AllowAll,
    DenyAll,
    Protected(AccessRuleNode),
}

impl Describe<ScryptoCustomTypeKind> for AccessRule {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::ACCESS_RULE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::access_rule_type_data()
    }
}

impl From<AccessRuleNode> for AccessRule {
    fn from(value: AccessRuleNode) -> Self {
        AccessRule::Protected(value)
    }
}

pub trait AccessRuleVisitor {
    type Error;
    fn visit(&mut self, node: &AccessRuleNode, depth: usize) -> Result<(), Self::Error>;
}

impl AccessRule {
    pub fn dfs_traverse_nodes<V: AccessRuleVisitor>(
        &self,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        match self {
            AccessRule::Protected(node) => node.dfs_traverse_recursive(visitor, 0),
            _ => Ok(()),
        }
    }
}

impl AccessRuleNode {
    fn dfs_traverse_recursive<V: AccessRuleVisitor>(
        &self,
        visitor: &mut V,
        depth: usize,
    ) -> Result<(), V::Error> {
        visitor.visit(self, depth)?;

        match self {
            AccessRuleNode::ProofRule(..) => {}
            AccessRuleNode::AnyOf(nodes) | AccessRuleNode::AllOf(nodes) => {
                for node in nodes {
                    node.dfs_traverse_recursive(visitor, depth + 1)?;
                }
            }
        }

        Ok(())
    }
}
