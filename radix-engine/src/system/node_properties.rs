use super::node::{RENodeInit, RENodeModuleInit};
use crate::errors::{KernelError, RuntimeError};
use crate::kernel::{ExecutionMode, LockFlags, ResolvedActor, ResolvedReceiver};
use crate::system::global::GlobalAddressSubstate;
use radix_engine_interface::api::types::*;
use sbor::rust::collections::BTreeMap;

pub struct VisibilityProperties;

impl VisibilityProperties {
    pub fn check_drop_node_visibility(
        mode: ExecutionMode,
        actor: &ResolvedActor,
        node_id: RENodeId,
    ) -> bool {
        match mode {
            ExecutionMode::LoggerModule => match node_id {
                RENodeId::Logger => return true,
                _ => return false,
            },
            ExecutionMode::Client => match node_id {
                // TODO: Cleanup and reduce to least privilege
                RENodeId::Worktop => match &actor.identifier {
                    FnIdentifier::Native(NativeFn::TransactionProcessor(..)) => true,
                    _ => false,
                },
                RENodeId::AuthZoneStack => match &actor.identifier {
                    FnIdentifier::Native(NativeFn::TransactionProcessor(..)) => true,
                    _ => false,
                },
                RENodeId::TransactionRuntime => match &actor.identifier {
                    FnIdentifier::Native(NativeFn::TransactionProcessor(..)) => true,
                    _ => false,
                },
                RENodeId::Bucket(..) => match &actor.identifier {
                    FnIdentifier::Native(NativeFn::Bucket(..))
                    | FnIdentifier::Native(NativeFn::Worktop(..))
                    | FnIdentifier::Native(NativeFn::ResourceManager(..))
                    | FnIdentifier::Native(NativeFn::Vault(..)) => true,
                    _ => false,
                },
                RENodeId::Proof(..) => match &actor.identifier {
                    FnIdentifier::Native(NativeFn::AuthZoneStack(..)) => true,
                    FnIdentifier::Native(NativeFn::Proof(..)) => true,
                    FnIdentifier::Native(NativeFn::TransactionProcessor(
                        TransactionProcessorFn::Run,
                    )) => true,
                    FnIdentifier::Scrypto(..) => true,
                    _ => false,
                },
                _ => false,
            },
            _ => return false,
        }
    }

    pub fn check_create_node_visibility(
        mode: ExecutionMode,
        actor: &ResolvedActor,
        node: &RENodeInit,
        module_init: &BTreeMap<NodeModuleId, RENodeModuleInit>,
    ) -> bool {
        // TODO: Cleanup and reduce to least privilege
        match (mode, &actor.identifier) {
            (
                ExecutionMode::Client,
                FnIdentifier::Scrypto(ScryptoFnIdentifier {
                    package_address,
                    blueprint_name,
                    ..
                }),
            ) => match node {
                RENodeInit::Component(..) => {
                    if let Some(RENodeModuleInit::ComponentTypeInfo(type_info)) =
                        module_init.get(&NodeModuleId::ComponentTypeInfo)
                    {
                        blueprint_name.eq(&type_info.blueprint_name)
                            && package_address.eq(&type_info.package_address)
                    } else {
                        false
                    }
                }
                RENodeInit::KeyValueStore => true,
                RENodeInit::Global(GlobalAddressSubstate::Component(..)) => true,
                _ => false,
            },
            _ => true,
        }
    }

    pub fn check_substate_visibility(
        mode: ExecutionMode,
        actor: &ResolvedActor,
        node_id: RENodeId,
        offset: SubstateOffset,
        flags: LockFlags,
    ) -> bool {
        let read_only = flags == LockFlags::read_only();

        // TODO: Cleanup and reduce to least privilege
        match (mode, offset) {
            (ExecutionMode::Kernel, ..) => false, // Protect ourselves!
            (ExecutionMode::KernelDeref, offset) => match offset {
                SubstateOffset::Global(GlobalOffset::Global) => read_only,
                _ => false,
            },
            (ExecutionMode::LoggerModule, ..) => false,
            (ExecutionMode::NodeMoveModule, offset) => match offset {
                SubstateOffset::Bucket(BucketOffset::Bucket) => read_only,
                SubstateOffset::Proof(ProofOffset::Proof) => true,
                _ => false,
            },
            (ExecutionMode::TransactionModule, _offset) => false,
            (ExecutionMode::KernelDropNode, offset) => match offset {
                SubstateOffset::Bucket(BucketOffset::Bucket) => true,
                SubstateOffset::Proof(ProofOffset::Proof) => true,
                SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack) => true,
                SubstateOffset::Worktop(WorktopOffset::Worktop) => true,
                _ => false,
            },
            (ExecutionMode::EntityModule, _offset) => false,
            (ExecutionMode::AuthModule, offset) => match offset {
                SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack) => true,
                // TODO: Remove these and use AuthRulesSubstate
                SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager) => {
                    read_only
                }
                SubstateOffset::Bucket(BucketOffset::Bucket) => true, // TODO: Remove to read_only!
                SubstateOffset::Vault(VaultOffset::Vault) => read_only,
                SubstateOffset::Package(PackageOffset::Info) => read_only,
                SubstateOffset::Component(ComponentOffset::State0) => read_only,
                SubstateOffset::ComponentTypeInfo(ComponentTypeInfoOffset::TypeInfo) => read_only,
                SubstateOffset::AccessRulesChain(AccessRulesChainOffset::AccessRulesChain) => {
                    read_only
                }
                _ => false,
            },
            (ExecutionMode::Client, offset) => {
                if !flags.contains(LockFlags::MUTABLE) {
                    match &actor.identifier {
                        // Native
                        FnIdentifier::Native(..) => true,
                        // Scrypto
                        FnIdentifier::Scrypto(..) => match &actor.receiver {
                            None => match (node_id, offset) {
                                // READ package code & abi
                                (
                                    RENodeId::Package(_),
                                    SubstateOffset::Package(PackageOffset::Info), // TODO: Remove
                                ) => read_only,
                                // READ global substates
                                (RENodeId::Global(_), SubstateOffset::Global(_)) => read_only,
                                (
                                    RENodeId::Component(_),
                                    SubstateOffset::ComponentTypeInfo(
                                        ComponentTypeInfoOffset::TypeInfo,
                                    ),
                                ) => read_only,
                                // READ/WRITE KVStore entry
                                (
                                    RENodeId::KeyValueStore(_),
                                    SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..)),
                                ) => true,
                                // Otherwise, false
                                _ => false,
                            },
                            Some(ResolvedReceiver {
                                receiver: RENodeId::Component(component_address),
                                ..
                            }) => match (node_id, offset) {
                                // READ package code & abi
                                (
                                    RENodeId::Package(_),
                                    SubstateOffset::Package(PackageOffset::Info), // TODO: Remove
                                ) => read_only,
                                // READ global substates
                                (RENodeId::Global(_), SubstateOffset::Global(_)) => read_only,
                                (
                                    RENodeId::Component(_),
                                    SubstateOffset::ComponentTypeInfo(
                                        ComponentTypeInfoOffset::TypeInfo,
                                    ),
                                ) => read_only,
                                // READ/WRITE KVStore entry
                                (
                                    RENodeId::KeyValueStore(_),
                                    SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..)),
                                ) => true,
                                // READ/WRITE component application state
                                (
                                    RENodeId::Component(addr),
                                    SubstateOffset::Component(ComponentOffset::State0),
                                ) => addr.eq(component_address),
                                // Otherwise, false
                                _ => false,
                            },
                            _ => false,
                        },
                    }
                } else {
                    match &actor.identifier {
                        // Native
                        FnIdentifier::Native(..) => true,

                        // Scrypto
                        FnIdentifier::Scrypto(..) => match &actor.receiver {
                            None => match (node_id, offset) {
                                (
                                    RENodeId::KeyValueStore(_),
                                    SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..)),
                                ) => true,
                                _ => false,
                            },

                            Some(ResolvedReceiver {
                                receiver: RENodeId::Component(component_address),
                                ..
                            }) => match (node_id, offset) {
                                (
                                    RENodeId::KeyValueStore(_),
                                    SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..)),
                                ) => true,
                                (
                                    RENodeId::Component(addr),
                                    SubstateOffset::Component(ComponentOffset::State0),
                                ) => addr.eq(component_address),
                                _ => false,
                            },
                            _ => false,
                        },
                    }
                }
            }
        }
    }
}

pub struct SubstateProperties;

impl SubstateProperties {
    pub fn is_persisted(offset: &SubstateOffset) -> bool {
        match offset {
            SubstateOffset::Global(..) => true,
            SubstateOffset::AuthZoneStack(..) => false,
            SubstateOffset::FeeReserve(..) => false,
            SubstateOffset::Component(..) => true,
            SubstateOffset::Royalty(..) => true,
            SubstateOffset::AccessRulesChain(..) => true,
            SubstateOffset::Metadata(..) => true,
            SubstateOffset::Package(..) => true,
            SubstateOffset::ResourceManager(..) => true,
            SubstateOffset::KeyValueStore(..) => true,
            SubstateOffset::NonFungibleStore(..) => true,
            SubstateOffset::Vault(..) => true,
            SubstateOffset::EpochManager(..) => true,
            SubstateOffset::Validator(..) => true,
            SubstateOffset::Bucket(..) => false,
            SubstateOffset::Proof(..) => false,
            SubstateOffset::Worktop(..) => false,
            SubstateOffset::Logger(..) => false,
            SubstateOffset::Clock(..) => true,
            SubstateOffset::TransactionRuntime(..) => false,
            SubstateOffset::Account(..) => true,
            SubstateOffset::AccessController(..) => true,
            SubstateOffset::ComponentTypeInfo(..) => true,
        }
    }

    pub fn verify_can_own(offset: &SubstateOffset, node_id: RENodeId) -> Result<(), RuntimeError> {
        match offset {
            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..))
            | SubstateOffset::Component(ComponentOffset::State0) => match node_id {
                RENodeId::KeyValueStore(..) | RENodeId::Component { .. } | RENodeId::Vault(..) => {
                    Ok(())
                }
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager) => {
                match node_id {
                    RENodeId::NonFungibleStore(..) => Ok(()),
                    _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                        offset.clone(),
                        node_id,
                    ))),
                }
            }
            SubstateOffset::Worktop(WorktopOffset::Worktop) => match node_id {
                RENodeId::Bucket(..) => Ok(()),
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            SubstateOffset::Royalty(RoyaltyOffset::RoyaltyAccumulator) => match node_id {
                RENodeId::Vault(..) => Ok(()),
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            SubstateOffset::AccessController(AccessControllerOffset::AccessController) => {
                match node_id {
                    RENodeId::Vault(..) => Ok(()),
                    _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                        offset.clone(),
                        node_id,
                    ))),
                }
            }
            SubstateOffset::Validator(ValidatorOffset::Validator) => match node_id {
                RENodeId::Vault(..) => Ok(()),
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            SubstateOffset::Account(AccountOffset::Account) => match node_id {
                RENodeId::KeyValueStore(..) => Ok(()),
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            SubstateOffset::Global(GlobalOffset::Global) => match node_id {
                RENodeId::Component(..)
                | RENodeId::Package(..)
                | RENodeId::ResourceManager(..)
                | RENodeId::EpochManager(..)
                | RENodeId::Validator(..)
                | RENodeId::Clock(..)
                | RENodeId::Identity(..)
                | RENodeId::Account(..)
                | RENodeId::AccessController(..) => Ok(()),
                _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                    offset.clone(),
                    node_id,
                ))),
            },
            _ => Err(RuntimeError::KernelError(KernelError::InvalidOwnership(
                offset.clone(),
                node_id,
            ))),
        }
    }
}
