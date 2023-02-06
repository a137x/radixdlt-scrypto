use crate::errors::RuntimeError;
use crate::kernel::kernel_api::KernelSubstateApi;
use crate::kernel::*;
use crate::system::global::GlobalAddressSubstate;
use crate::system::kernel_modules::auth::method_authorization::*;
use crate::system::node::RENodeInit;
use crate::system::node::RENodeModuleInit;
use crate::system::node_modules::auth::AccessRulesChainSubstate;
use crate::types::*;
use crate::wasm::WasmEngine;
use radix_engine_interface::api::node_modules::auth::AuthAddresses;
use radix_engine_interface::api::types::*;
use radix_engine_interface::api::types::{
    ClockFn, ClockOffset, GlobalAddress, NativeFn, RENodeId, SubstateOffset,
};
use radix_engine_interface::api::ClientDerefApi;
use radix_engine_interface::api::ClientSubstateApi;
use radix_engine_interface::blueprints::clock::ClockCreateInvocation;
use radix_engine_interface::blueprints::clock::ClockGetCurrentTimeInvocation;
use radix_engine_interface::blueprints::clock::ClockSetCurrentTimeInvocation;
use radix_engine_interface::blueprints::clock::TimePrecision;
use radix_engine_interface::blueprints::clock::*;
use radix_engine_interface::blueprints::resource::require;
use radix_engine_interface::blueprints::resource::AccessRuleKey;
use radix_engine_interface::blueprints::resource::AccessRules;
use radix_engine_interface::rule;
use radix_engine_interface::time::*;

use super::CurrentTimeRoundedToMinutesSubstate;

const SECONDS_TO_MS_FACTOR: i64 = 1000;
const MINUTES_TO_SECONDS_FACTOR: i64 = 60;
const MINUTES_TO_MS_FACTOR: i64 = SECONDS_TO_MS_FACTOR * MINUTES_TO_SECONDS_FACTOR;

pub struct Clock;

impl ExecutableInvocation for ClockCreateInvocation {
    type Exec = Self;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        _deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError>
    where
        Self: Sized,
    {
        let actor = ResolvedActor::function(NativeFn::Clock(ClockFn::Create));
        let call_frame_update = CallFrameUpdate::empty();

        Ok((actor, call_frame_update, self))
    }
}

impl Executor for ClockCreateInvocation {
    type Output = ComponentAddress;

    fn execute<Y, W: WasmEngine>(
        self,
        system_api: &mut Y,
    ) -> Result<(Self::Output, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientSubstateApi<RuntimeError>,
    {
        let underlying_node_id = system_api.allocate_node_id(RENodeType::Clock)?;

        let mut access_rules = AccessRules::new();
        access_rules.set_method_access_rule(
            AccessRuleKey::Native(NativeFn::Clock(ClockFn::SetCurrentTime)),
            rule!(require(AuthAddresses::validator_role())),
        );
        access_rules.set_method_access_rule(
            AccessRuleKey::Native(NativeFn::Clock(ClockFn::GetCurrentTime)),
            rule!(allow_all),
        );
        access_rules.set_method_access_rule(
            AccessRuleKey::Native(NativeFn::Clock(ClockFn::CompareCurrentTime)),
            rule!(allow_all),
        );

        let mut node_modules = BTreeMap::new();
        node_modules.insert(
            NodeModuleId::AccessRules,
            RENodeModuleInit::AccessRulesChain(AccessRulesChainSubstate {
                access_rules_chain: vec![access_rules],
            }),
        );

        system_api.create_node(
            underlying_node_id,
            RENodeInit::Clock(CurrentTimeRoundedToMinutesSubstate {
                current_time_rounded_to_minutes_ms: 0,
            }),
            node_modules,
        )?;

        let global_node_id = RENodeId::Global(GlobalAddress::Component(ComponentAddress::Clock(
            self.component_address,
        )));
        system_api.create_node(
            global_node_id,
            RENodeInit::Global(GlobalAddressSubstate::Clock(underlying_node_id.into())),
            BTreeMap::new(),
        )?;

        let system_address: ComponentAddress = global_node_id.into();
        let mut node_refs_to_copy = HashSet::new();
        node_refs_to_copy.insert(global_node_id);

        let update = CallFrameUpdate {
            node_refs_to_copy,
            nodes_to_move: vec![],
        };

        Ok((system_address, update))
    }
}

pub struct ClockSetCurrentTimeExecutable(RENodeId, i64);

impl ExecutableInvocation for ClockSetCurrentTimeInvocation {
    type Exec = ClockSetCurrentTimeExecutable;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError>
    where
        Self: Sized,
    {
        let mut call_frame_update = CallFrameUpdate::empty();
        let receiver = RENodeId::Global(GlobalAddress::Component(self.receiver));
        let resolved_receiver = deref_and_update(receiver, &mut call_frame_update, deref)?;

        let actor =
            ResolvedActor::method(NativeFn::Clock(ClockFn::SetCurrentTime), resolved_receiver);
        let executor =
            ClockSetCurrentTimeExecutable(resolved_receiver.receiver, self.current_time_ms);

        Ok((actor, call_frame_update, executor))
    }
}

impl Executor for ClockSetCurrentTimeExecutable {
    type Output = ();

    fn execute<Y, W: WasmEngine>(
        self,
        system_api: &mut Y,
    ) -> Result<((), CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        let node_id = self.0;

        let current_time_ms = self.1;
        let current_time_rounded_to_minutes =
            (current_time_ms / MINUTES_TO_MS_FACTOR) * MINUTES_TO_MS_FACTOR;

        let offset = SubstateOffset::Clock(ClockOffset::CurrentTimeRoundedToMinutes);
        let handle =
            system_api.lock_substate(node_id, NodeModuleId::SELF, offset, LockFlags::MUTABLE)?;
        let mut substate_ref = system_api.get_ref_mut(handle)?;
        substate_ref
            .current_time_rounded_to_minutes()
            .current_time_rounded_to_minutes_ms = current_time_rounded_to_minutes;

        Ok(((), CallFrameUpdate::empty()))
    }
}

pub struct ClockGetCurrentTimeExecutable(RENodeId, TimePrecision);

impl ExecutableInvocation for ClockGetCurrentTimeInvocation {
    type Exec = ClockGetCurrentTimeExecutable;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError>
    where
        Self: Sized,
    {
        let mut call_frame_update = CallFrameUpdate::empty();
        let receiver = RENodeId::Global(GlobalAddress::Component(self.receiver));
        let resolved_receiver = deref_and_update(receiver, &mut call_frame_update, deref)?;

        let actor =
            ResolvedActor::method(NativeFn::Clock(ClockFn::GetCurrentTime), resolved_receiver);
        let executor = ClockGetCurrentTimeExecutable(resolved_receiver.receiver, self.precision);

        Ok((actor, call_frame_update, executor))
    }
}

impl Executor for ClockGetCurrentTimeExecutable {
    type Output = Instant;

    fn execute<Y, W: WasmEngine>(
        self,
        system_api: &mut Y,
    ) -> Result<(Instant, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        let node_id = self.0;
        let precision = self.1;

        match precision {
            TimePrecision::Minute => {
                let offset = SubstateOffset::Clock(ClockOffset::CurrentTimeRoundedToMinutes);
                let handle = system_api.lock_substate(
                    node_id,
                    NodeModuleId::SELF,
                    offset,
                    LockFlags::read_only(),
                )?;
                let substate_ref = system_api.get_ref(handle)?;
                let substate = substate_ref.current_time_rounded_to_minutes();
                let instant = Instant::new(
                    substate.current_time_rounded_to_minutes_ms / SECONDS_TO_MS_FACTOR,
                );
                Ok((instant, CallFrameUpdate::empty()))
            }
        }
    }
}

pub struct ClockCompareCurrentTimeExecutable {
    node_id: RENodeId,
    instant: Instant,
    precision: TimePrecision,
    operator: TimeComparisonOperator,
}

impl ExecutableInvocation for ClockCompareCurrentTimeInvocation {
    type Exec = ClockCompareCurrentTimeExecutable;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError>
    where
        Self: Sized,
    {
        let mut call_frame_update = CallFrameUpdate::empty();
        let receiver = RENodeId::Global(GlobalAddress::Component(self.receiver));
        let resolved_receiver = deref_and_update(receiver, &mut call_frame_update, deref)?;

        let actor = ResolvedActor::method(
            NativeFn::Clock(ClockFn::CompareCurrentTime),
            resolved_receiver,
        );
        let executor = ClockCompareCurrentTimeExecutable {
            node_id: resolved_receiver.receiver,
            instant: self.instant,
            precision: self.precision,
            operator: self.operator,
        };

        Ok((actor, call_frame_update, executor))
    }
}

impl Executor for ClockCompareCurrentTimeExecutable {
    type Output = bool;

    fn execute<Y, W: WasmEngine>(
        self,
        system_api: &mut Y,
    ) -> Result<(bool, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        match self.precision {
            TimePrecision::Minute => {
                let offset = SubstateOffset::Clock(ClockOffset::CurrentTimeRoundedToMinutes);
                let handle = system_api.lock_substate(
                    self.node_id,
                    NodeModuleId::SELF,
                    offset,
                    LockFlags::read_only(),
                )?;
                let substate_ref = system_api.get_ref(handle)?;
                let substate = substate_ref.current_time_rounded_to_minutes();
                let current_time_instant = Instant::new(
                    substate.current_time_rounded_to_minutes_ms / SECONDS_TO_MS_FACTOR,
                );
                let other_instant_rounded = Instant::new(
                    (self.instant.seconds_since_unix_epoch / MINUTES_TO_SECONDS_FACTOR)
                        * MINUTES_TO_SECONDS_FACTOR,
                );
                let result = current_time_instant.compare(other_instant_rounded, self.operator);
                Ok((result, CallFrameUpdate::empty()))
            }
        }
    }
}

impl Clock {
    pub fn create_auth() -> Vec<MethodAuthorization> {
        vec![MethodAuthorization::Protected(HardAuthRule::ProofRule(
            HardProofRule::Require(HardResourceOrNonFungible::NonFungible(
                AuthAddresses::system_role(),
            )),
        ))]
    }
}
