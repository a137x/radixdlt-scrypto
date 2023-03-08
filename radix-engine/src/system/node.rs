use crate::blueprints::transaction_runtime::TransactionRuntimeSubstate;
use crate::system::node_modules::access_rules::*;
use crate::system::node_modules::metadata::MetadataSubstate;
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::system::node_substates::*;
use crate::types::*;
use radix_engine_interface::api::component::*;
use radix_engine_interface::api::package::*;
use radix_engine_interface::api::types::{AuthZoneStackOffset, PackageOffset, SubstateOffset};

#[derive(Debug)]
pub enum RENodeModuleInit {
    /* Type info */
    TypeInfo(TypeInfoSubstate),

    /* Metadata */
    Metadata(MetadataSubstate),

    /* Access rules */
    MethodAccessRules(MethodAccessRulesSubstate),
    FunctionAccessRules(FunctionAccessRulesSubstate),

    /* Royalty */
    ComponentRoyalty(
        ComponentRoyaltyConfigSubstate,
        ComponentRoyaltyAccumulatorSubstate,
    ),
    PackageRoyalty(
        PackageRoyaltyConfigSubstate,
        PackageRoyaltyAccumulatorSubstate,
    ),
}

impl RENodeModuleInit {
    pub fn to_substates(self) -> HashMap<SubstateOffset, RuntimeSubstate> {
        let mut substates = HashMap::<SubstateOffset, RuntimeSubstate>::new();
        match self {
            RENodeModuleInit::Metadata(metadata) => {
                substates.insert(
                    SubstateOffset::Metadata(MetadataOffset::Metadata),
                    metadata.into(),
                );
            }
            RENodeModuleInit::MethodAccessRules(access_rules) => {
                substates.insert(
                    SubstateOffset::AccessRules(AccessRulesOffset::AccessRules),
                    access_rules.into(),
                );
            }
            RENodeModuleInit::FunctionAccessRules(access_rules) => {
                substates.insert(SubstateOffset::PackageAccessRules, access_rules.into());
            }
            RENodeModuleInit::TypeInfo(type_info) => {
                substates.insert(
                    SubstateOffset::TypeInfo(TypeInfoOffset::TypeInfo),
                    type_info.into(),
                );
            }
            RENodeModuleInit::ComponentRoyalty(config, accumulator) => {
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyConfig),
                    config.into(),
                );
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyAccumulator),
                    accumulator.into(),
                );
            }
            RENodeModuleInit::PackageRoyalty(config, accumulator) => {
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyConfig),
                    config.into(),
                );
                substates.insert(
                    SubstateOffset::Royalty(RoyaltyOffset::RoyaltyAccumulator),
                    accumulator.into(),
                );
            }
        }

        substates
    }
}

#[derive(Debug)]
pub enum RENodeInit {
    GlobalObject(BTreeMap<SubstateOffset, RuntimeSubstate>),
    GlobalPackage(PackageInfoSubstate, PackageCodeSubstate),
    Object(BTreeMap<SubstateOffset, RuntimeSubstate>),
    AuthZoneStack(AuthZoneStackSubstate),
    KeyValueStore,
    NonFungibleStore,
    TransactionRuntime(TransactionRuntimeSubstate),
}

impl RENodeInit {
    pub fn to_substates(self) -> HashMap<SubstateOffset, RuntimeSubstate> {
        let mut substates = HashMap::<SubstateOffset, RuntimeSubstate>::new();
        match self {
            RENodeInit::AuthZoneStack(auth_zone) => {
                substates.insert(
                    SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
                    RuntimeSubstate::AuthZoneStack(auth_zone),
                );
            }
            RENodeInit::GlobalObject(object_substates) | RENodeInit::Object(object_substates) => {
                substates.extend(object_substates);
            }
            RENodeInit::KeyValueStore | RENodeInit::NonFungibleStore => {}
            RENodeInit::GlobalPackage(package_info, code) => {
                substates.insert(
                    SubstateOffset::Package(PackageOffset::Info),
                    package_info.into(),
                );
                substates.insert(SubstateOffset::Package(PackageOffset::Code), code.into());
            }
            RENodeInit::TransactionRuntime(transaction_hash) => {
                substates.insert(
                    SubstateOffset::TransactionRuntime(
                        TransactionRuntimeOffset::TransactionRuntime,
                    ),
                    transaction_hash.into(),
                );
            }
        };

        substates
    }
}
