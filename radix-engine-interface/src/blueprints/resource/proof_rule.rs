use crate::blueprints::resource::RuleNode::{AllOf, AnyOf};
use crate::internal_prelude::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
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
#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
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

impl From<ResourceAddress> for RuleNode {
    fn from(resource_address: ResourceAddress) -> Self {
        RuleNode::ProofRule(ProofRule::Require(resource_address.into()))
    }
}

impl From<NonFungibleGlobalId> for RuleNode {
    fn from(id: NonFungibleGlobalId) -> Self {
        RuleNode::ProofRule(ProofRule::Require(id.into()))
    }
}

impl From<ResourceOrNonFungible> for RuleNode {
    fn from(resource_or_non_fungible: ResourceOrNonFungible) -> Self {
        RuleNode::ProofRule(ProofRule::Require(resource_or_non_fungible))
    }
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
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
pub enum RuleNode {
    ProofRule(ProofRule),
    AnyOf(Vec<RuleNode>),
    AllOf(Vec<RuleNode>),
}

/// Notes: This is to be deprecated, please use `RuleNode` instead
pub type AccessRuleNode = RuleNode;

impl Describe<ScryptoCustomTypeKind> for RuleNode {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::ACCESS_RULE_NODE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::access_rule_node_type_data()
    }
}

impl RuleNode {
    pub fn or(self, other: RuleNode) -> Self {
        match self {
            RuleNode::AnyOf(mut rules) => {
                rules.push(other);
                AnyOf(rules)
            }
            _ => AnyOf(vec![self, other]),
        }
    }

    pub fn and(self, other: RuleNode) -> Self {
        match self {
            RuleNode::AllOf(mut rules) => {
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

pub fn require<T>(required: T) -> RuleNode
where
    T: Into<RuleNode>,
{
    required.into()
}

pub fn require_any_of<T>(resources: T) -> RuleNode
where
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    RuleNode::ProofRule(ProofRule::AnyOf(list.list))
}

pub fn require_all_of<T>(resources: T) -> RuleNode
where
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    RuleNode::ProofRule(ProofRule::AllOf(list.list))
}

pub fn require_n_of<C, T>(count: C, resources: T) -> RuleNode
where
    C: Into<u8>,
    T: Into<ResourceOrNonFungibleList>,
{
    let list: ResourceOrNonFungibleList = resources.into();
    RuleNode::ProofRule(ProofRule::CountOf(count.into(), list.list))
}

pub fn require_amount<D, T>(amount: D, resource: T) -> RuleNode
where
    D: Into<Decimal>,
    T: Into<ResourceAddress>,
{
    RuleNode::ProofRule(ProofRule::AmountOf(amount.into(), resource.into()))
}

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
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
pub enum Rule {
    AllowAll,
    DenyAll,
    Protected(RuleNode),
}

/// Notes: This is to be deprecated, please use `Rule` instead
pub type AccessRule = Rule;

impl Describe<ScryptoCustomTypeKind> for Rule {
    const TYPE_ID: RustTypeId =
        RustTypeId::WellKnown(well_known_scrypto_custom_types::ACCESS_RULE_TYPE);

    fn type_data() -> ScryptoTypeData<RustTypeId> {
        well_known_scrypto_custom_types::access_rule_type_data()
    }
}

impl From<RuleNode> for Rule {
    fn from(value: RuleNode) -> Self {
        Rule::Protected(value)
    }
}

pub trait RuleVisitor {
    type Error;
    fn visit(&mut self, node: &RuleNode, depth: usize) -> Result<(), Self::Error>;
}

impl Rule {
    pub fn dfs_traverse_nodes<V: RuleVisitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Rule::Protected(node) => node.dfs_traverse_recursive(visitor, 0),
            _ => Ok(()),
        }
    }
}

impl RuleNode {
    fn dfs_traverse_recursive<V: RuleVisitor>(
        &self,
        visitor: &mut V,
        depth: usize,
    ) -> Result<(), V::Error> {
        visitor.visit(self, depth)?;

        match self {
            RuleNode::ProofRule(..) => {}
            RuleNode::AnyOf(nodes) | RuleNode::AllOf(nodes) => {
                for node in nodes {
                    node.dfs_traverse_recursive(visitor, depth + 1)?;
                }
            }
        }

        Ok(())
    }
}
