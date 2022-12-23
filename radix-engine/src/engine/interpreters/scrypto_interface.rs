use crate::engine::{
    Kernel, KernelError, LockFlags, REActor, RENode, ResolvedFunction, ResolvedMethod,
    ResolvedReceiver, RuntimeError, SystemApi,
};
use crate::fee::FeeReserve;
use crate::model::{
    AccessRulesChainSubstate, ComponentInfoSubstate, ComponentRoyaltyAccumulatorSubstate,
    ComponentRoyaltyConfigSubstate, ComponentStateSubstate, KeyValueStore, RuntimeSubstate,
};
use crate::model::{MetadataSubstate, Resource};
use crate::types::HashMap;
use crate::wasm::WasmEngine;
use radix_engine_interface::api::api::EngineApi;
use radix_engine_interface::api::types::{
    ComponentMethod, LockHandle, NativeFn, NativeMethod, RENodeId, RENodeType, ScryptoActor,
    ScryptoRENode, SubstateOffset,
};
use radix_engine_interface::constants::RADIX_TOKEN;
use radix_engine_interface::crypto::Hash;
use radix_engine_interface::model::{
    AccessRule, AccessRuleKey, AccessRules, ResourceType, RoyaltyConfig,
};
use sbor::rust::string::ToString;
use sbor::rust::vec;
use sbor::rust::vec::Vec;

impl<'g, 's, W, R> EngineApi<RuntimeError> for Kernel<'g, 's, W, R>
where
    W: WasmEngine,
    R: FeeReserve,
{
    fn sys_create_node(&mut self, node: ScryptoRENode) -> Result<RENodeId, RuntimeError> {
        let (node_id, node) = match node {
            ScryptoRENode::Component(package_address, blueprint_name, state) => {
                let node_id = self.allocate_node_id(RENodeType::Component)?;

                // Royalty initialization done here
                let royalty_config = ComponentRoyaltyConfigSubstate {
                    royalty_config: RoyaltyConfig::default(),
                };
                let royalty_accumulator = ComponentRoyaltyAccumulatorSubstate {
                    royalty: Resource::new_empty(
                        RADIX_TOKEN,
                        ResourceType::Fungible { divisibility: 18 },
                    ),
                };

                // TODO: Remove Royalties from Node's access rule chain, possibly implement this
                // TODO: via associated nodes rather than inheritance?
                let mut access_rules =
                    AccessRules::new().default(AccessRule::AllowAll, AccessRule::AllowAll);
                access_rules.set_group_and_mutability(
                    AccessRuleKey::Native(NativeFn::Method(NativeMethod::Component(
                        ComponentMethod::ClaimRoyalty,
                    ))),
                    "royalty".to_string(),
                    AccessRule::DenyAll,
                );
                access_rules.set_group_and_mutability(
                    AccessRuleKey::Native(NativeFn::Method(NativeMethod::Component(
                        ComponentMethod::SetRoyaltyConfig,
                    ))),
                    "royalty".to_string(),
                    AccessRule::DenyAll,
                );
                access_rules.set_group_access_rule_and_mutability(
                    "royalty".to_string(),
                    AccessRule::AllowAll,
                    AccessRule::AllowAll,
                );

                let node = RENode::Component(
                    ComponentInfoSubstate::new(package_address, blueprint_name),
                    ComponentStateSubstate::new(state),
                    royalty_config,
                    royalty_accumulator,
                    MetadataSubstate {
                        metadata: HashMap::new(),
                    },
                    AccessRulesChainSubstate {
                        access_rules_chain: vec![access_rules],
                    },
                );

                (node_id, node)
            }
            ScryptoRENode::KeyValueStore => {
                let node_id = self.allocate_node_id(RENodeType::KeyValueStore)?;
                let node = RENode::KeyValueStore(KeyValueStore::new());
                (node_id, node)
            }
        };

        self.create_node(node_id, node)?;

        Ok(node_id)
    }

    fn sys_drop_node(&mut self, node_id: RENodeId) -> Result<(), RuntimeError> {
        self.drop_node(node_id)?;
        Ok(())
    }

    fn sys_get_visible_nodes(&mut self) -> Result<Vec<RENodeId>, RuntimeError> {
        self.get_visible_node_ids()
    }

    fn sys_lock_substate(
        &mut self,
        node_id: RENodeId,
        offset: SubstateOffset,
        mutable: bool,
    ) -> Result<LockHandle, RuntimeError> {
        let flags = if mutable {
            LockFlags::MUTABLE
        } else {
            // TODO: Do we want to expose full flag functionality to Scrypto?
            LockFlags::read_only()
        };

        self.lock_substate(node_id, offset, flags)
    }

    fn sys_read(&mut self, lock_handle: LockHandle) -> Result<Vec<u8>, RuntimeError> {
        self.get_ref(lock_handle)
            .map(|substate_ref| substate_ref.to_scrypto_value().raw)
    }

    fn sys_write(&mut self, lock_handle: LockHandle, buffer: Vec<u8>) -> Result<(), RuntimeError> {
        let offset = self.get_lock_info(lock_handle)?.offset;
        let substate = RuntimeSubstate::decode_from_buffer(&offset, &buffer)?;
        let mut substate_mut = self.get_ref_mut(lock_handle)?;

        match substate {
            RuntimeSubstate::ComponentState(next) => *substate_mut.component_state() = next,
            RuntimeSubstate::KeyValueStoreEntry(next) => {
                *substate_mut.kv_store_entry() = next;
            }
            RuntimeSubstate::NonFungible(next) => {
                *substate_mut.non_fungible() = next;
            }
            _ => return Err(RuntimeError::KernelError(KernelError::InvalidOverwrite)),
        }

        Ok(())
    }

    fn sys_drop_lock(&mut self, lock_handle: LockHandle) -> Result<(), RuntimeError> {
        self.drop_lock(lock_handle)
    }

    fn sys_get_actor(&mut self) -> Result<ScryptoActor, RuntimeError> {
        let actor = match self.get_actor() {
            REActor::Method(
                ResolvedMethod::Scrypto {
                    package_address,
                    blueprint_name,
                    ..
                },
                ResolvedReceiver {
                    receiver: RENodeId::Component(component_id),
                    ..
                },
            ) => ScryptoActor::Component(
                *component_id,
                package_address.clone(),
                blueprint_name.clone(),
            ),
            REActor::Function(ResolvedFunction::Scrypto {
                package_address,
                blueprint_name,
                ..
            }) => ScryptoActor::blueprint(*package_address, blueprint_name.clone()),

            _ => panic!("Should not get here."),
        };

        Ok(actor)
    }

    fn sys_generate_uuid(&mut self) -> Result<u128, RuntimeError> {
        self.generate_uuid()
    }

    fn sys_get_transaction_hash(&mut self) -> Result<Hash, RuntimeError> {
        self.read_transaction_hash()
    }
}
