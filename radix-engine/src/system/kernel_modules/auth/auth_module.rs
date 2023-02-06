use crate::blueprints::clock::Clock;
use crate::blueprints::epoch_manager::EpochManager;
use crate::errors::*;
use crate::kernel::*;
use crate::types::*;
use radix_engine_interface::api::types::{
    AuthZoneStackOffset, ComponentOffset, GlobalAddress, PackageOffset, RENodeId, SubstateOffset,
    VaultOffset,
};
use radix_engine_interface::api::ClientActorApi;

use super::auth_converter::convert_contextless;
use super::method_authorization::MethodAuthorization;
use super::method_authorization::MethodAuthorizationError;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub enum AuthError {
    VisibilityError(RENodeId),
    Unauthorized {
        actor: ResolvedActor,
        authorization: MethodAuthorization,
        error: MethodAuthorizationError,
    },
}

pub struct AuthModule;

impl AuthModule {
    pub fn on_before_frame_start<Y>(
        actor: &ResolvedActor,
        system_api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        if matches!(
            actor.identifier,
            FnIdentifier::Native(NativeFn::AuthZoneStack(..))
        ) {
            return Ok(());
        }

        let method_auths = match &actor {
            ResolvedActor {
                identifier: FnIdentifier::Native(native_function),
                receiver: None,
            } => match native_function {
                NativeFn::EpochManager(EpochManagerFn::Create) => EpochManager::create_auth(),
                NativeFn::Clock(ClockFn::Create) => Clock::create_auth(),
                _ => vec![],
            },
            ResolvedActor {
                identifier: FnIdentifier::Native(native_fn),
                receiver: Some(resolved_receiver),
            } => {
                match (native_fn, resolved_receiver) {
                    // SetAccessRule auth is done manually within the method
                    (NativeFn::AccessRulesChain(AccessRulesChainFn::SetMethodAccessRule), ..) => {
                        vec![]
                    }
                    (method, ..)
                        if matches!(method, NativeFn::Metadata(..))
                            || matches!(method, NativeFn::EpochManager(..))
                            || matches!(method, NativeFn::Validator(..))
                            || matches!(method, NativeFn::ResourceManager(..))
                            || matches!(method, NativeFn::Package(..))
                            || matches!(method, NativeFn::Clock(..))
                            || matches!(method, NativeFn::Component(..))
                            || matches!(method, NativeFn::Account(..))
                            || matches!(method, NativeFn::AccessController(..)) =>
                    {
                        let handle = system_api.lock_substate(
                            resolved_receiver.receiver,
                            NodeModuleId::AccessRules,
                            SubstateOffset::AccessRulesChain(
                                AccessRulesChainOffset::AccessRulesChain,
                            ),
                            LockFlags::read_only(),
                        )?;
                        let substate_ref = system_api.get_ref(handle)?;
                        let substate = substate_ref.access_rules_chain();
                        let auth = substate.native_fn_authorization(*method);
                        system_api.drop_lock(handle)?;
                        auth
                    }
                    (
                        NativeFn::Vault(ref vault_fn),
                        ResolvedReceiver {
                            receiver: RENodeId::Vault(vault_id),
                            ..
                        },
                    ) => {
                        let vault_node_id = RENodeId::Vault(*vault_id);
                        let visibility = system_api.get_visible_node_data(vault_node_id)?;

                        let resource_address = {
                            let offset = SubstateOffset::Vault(VaultOffset::Vault);
                            let handle = system_api.lock_substate(
                                vault_node_id,
                                NodeModuleId::SELF,
                                offset,
                                LockFlags::read_only(),
                            )?;
                            let substate_ref = system_api.get_ref(handle)?;
                            let resource_address = substate_ref.vault().resource_address();
                            system_api.drop_lock(handle)?;
                            resource_address
                        };
                        let node_id = RENodeId::Global(GlobalAddress::Resource(resource_address));
                        let handle = system_api.lock_substate(
                            node_id,
                            NodeModuleId::AccessRules1,
                            SubstateOffset::AccessRulesChain(
                                AccessRulesChainOffset::AccessRulesChain,
                            ),
                            LockFlags::read_only(),
                        )?;

                        let substate_ref = system_api.get_ref(handle)?;
                        let substate = substate_ref.access_rules_chain();

                        // TODO: Revisit what the correct abstraction is for visibility in the auth module
                        let auth = match visibility {
                            RENodeVisibilityOrigin::Normal => {
                                substate.native_fn_authorization(NativeFn::Vault(vault_fn.clone()))
                            }
                            RENodeVisibilityOrigin::DirectAccess => match vault_fn {
                                // TODO: Do we want to allow recaller to be able to withdraw from
                                // TODO: any visible vault?
                                VaultFn::Recall | VaultFn::RecallNonFungibles => {
                                    let access_rule =
                                        substate.access_rules_chain[0].get_group("recall");
                                    let authorization = convert_contextless(access_rule);
                                    vec![authorization]
                                }
                                _ => {
                                    return Err(RuntimeError::ModuleError(ModuleError::AuthError(
                                        AuthError::VisibilityError(vault_node_id),
                                    )));
                                }
                            },
                        };

                        system_api.drop_lock(handle)?;
                        auth
                    }
                    _ => vec![],
                }
            }
            ResolvedActor {
                identifier: FnIdentifier::Scrypto(method_identifier),
                receiver:
                    Some(ResolvedReceiver {
                        receiver: RENodeId::Component(component_id),
                        ..
                    }),
            } => {
                let node_id =
                    RENodeId::Global(GlobalAddress::Package(method_identifier.package_address));
                let offset = SubstateOffset::Package(PackageOffset::Info);
                let handle = system_api.lock_substate(
                    node_id,
                    NodeModuleId::SELF,
                    offset,
                    LockFlags::read_only(),
                )?;

                // Assume that package_address/blueprint is the original impl of Component for now
                // TODO: Remove this assumption
                let substate_ref = system_api.get_ref(handle)?;
                let package = substate_ref.package_info();
                let schema = package
                    .blueprint_abi(&method_identifier.blueprint_name)
                    .expect("Blueprint not found for existing component")
                    .structure
                    .clone();
                system_api.drop_lock(handle)?;

                let component_node_id = RENodeId::Component(*component_id);
                let state = {
                    let offset = SubstateOffset::Component(ComponentOffset::State0);
                    let handle = system_api.lock_substate(
                        component_node_id,
                        NodeModuleId::SELF,
                        offset,
                        LockFlags::read_only(),
                    )?;
                    let substate_ref = system_api.get_ref(handle)?;
                    let state = substate_ref.component_state().clone(); // TODO: Remove clone
                    system_api.drop_lock(handle)?;
                    state
                };
                {
                    let handle = system_api.lock_substate(
                        component_node_id,
                        NodeModuleId::AccessRules,
                        SubstateOffset::AccessRulesChain(AccessRulesChainOffset::AccessRulesChain),
                        LockFlags::read_only(),
                    )?;
                    let substate_ref = system_api.get_ref(handle)?;
                    let access_rules = substate_ref.access_rules_chain();
                    let auth = access_rules.method_authorization(
                        &state,
                        &schema,
                        method_identifier.ident.clone(),
                    );
                    system_api.drop_lock(handle)?;
                    auth
                }
            }

            _ => vec![],
        };

        let handle = system_api.lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::read_only(),
        )?;
        let substate_ref = system_api.get_ref(handle)?;
        let auth_zone_stack = substate_ref.auth_zone_stack();
        let is_barrier = Self::is_barrier(actor);

        // Authorization check
        auth_zone_stack
            .check_auth(is_barrier, method_auths)
            .map_err(|(authorization, error)| {
                RuntimeError::ModuleError(ModuleError::AuthError(AuthError::Unauthorized {
                    actor: actor.clone(),
                    authorization,
                    error,
                }))
            })?;

        system_api.drop_lock(handle)?;

        Ok(())
    }

    pub fn on_call_frame_enter<Y: KernelNodeApi + KernelSubstateApi>(
        call_frame_update: &mut CallFrameUpdate,
        actor: &ResolvedActor,
        system_api: &mut Y,
    ) -> Result<(), RuntimeError> {
        call_frame_update
            .node_refs_to_copy
            .insert(RENodeId::AuthZoneStack);

        if !matches!(
            actor.identifier,
            FnIdentifier::Native(NativeFn::AuthZoneStack(..))
                | FnIdentifier::Native(NativeFn::AccessRulesChain(..))
        ) {
            let handle = system_api.lock_substate(
                RENodeId::AuthZoneStack,
                NodeModuleId::SELF,
                SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
                LockFlags::MUTABLE,
            )?;
            let mut substate_ref_mut = system_api.get_ref_mut(handle)?;
            let auth_zone_stack = substate_ref_mut.auth_zone_stack();

            // New auth zone frame managed by the AuthModule
            let is_barrier = Self::is_barrier(actor);

            // Add Package Actor Auth
            let id = scrypto_encode(&actor.identifier.package_identifier()).unwrap();
            let non_fungible_global_id =
                NonFungibleGlobalId::new(PACKAGE_TOKEN, NonFungibleLocalId::Bytes(id));
            let mut virtual_non_fungibles = BTreeSet::new();
            virtual_non_fungibles.insert(non_fungible_global_id);

            auth_zone_stack.new_frame(virtual_non_fungibles, is_barrier);
            system_api.drop_lock(handle)?;
        }

        Ok(())
    }

    fn is_barrier(actor: &ResolvedActor) -> bool {
        matches!(
            actor,
            ResolvedActor {
                identifier: FnIdentifier::Scrypto(..),
                receiver: Some(ResolvedReceiver {
                    derefed_from: Some((RENodeId::Global(GlobalAddress::Component(..)), _)),
                    ..
                })
            }
        )
    }

    pub fn on_call_frame_exit<Y>(api: &mut Y) -> Result<(), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientActorApi<RuntimeError>,
    {
        if matches!(
            api.fn_identifier()?,
            FnIdentifier::Native(NativeFn::AuthZoneStack(..))
                | FnIdentifier::Native(NativeFn::AccessRulesChain(..)),
        ) {
            return Ok(());
        }

        let handle = api.lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;
        {
            let mut substate_ref_mut = api.get_ref_mut(handle)?;
            let auth_zone_stack = substate_ref_mut.auth_zone_stack();
            auth_zone_stack.pop_frame();
        }
        api.drop_lock(handle)?;

        Ok(())
    }
}
