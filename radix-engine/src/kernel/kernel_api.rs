use crate::errors::*;
use crate::kernel::*;
use crate::system::node::RENodeInit;
use crate::system::node::RENodeModuleInit;
use crate::system::node_substates::{SubstateRef, SubstateRefMut};
use crate::types::*;
use crate::wasm::WasmEngine;
use radix_engine_interface::api::component::*;
use radix_engine_interface::api::node_modules::auth::*;
use radix_engine_interface::api::node_modules::metadata::*;
use radix_engine_interface::api::package::*;
use radix_engine_interface::api::types::*;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::api::ClientDerefApi;
use radix_engine_interface::blueprints::access_controller::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::clock::*;
use radix_engine_interface::blueprints::epoch_manager::*;
use radix_engine_interface::blueprints::identity::*;
use radix_engine_interface::blueprints::logger::*;
use radix_engine_interface::blueprints::resource::Resource;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::blueprints::transaction_runtime::*;

pub trait KernelNodeApi {
    fn lock_fee(
        &mut self,
        vault_id: VaultId,
        fee: Resource,
        contingent: bool,
    ) -> Result<Resource, RuntimeError>; // TODO: move

    fn get_visible_node_data(
        &mut self,
        node_id: RENodeId,
    ) -> Result<RENodeVisibilityOrigin, RuntimeError>;

    /// Removes an RENode and all of it's children from the Heap
    fn drop_node(&mut self, node_id: RENodeId) -> Result<HeapRENode, RuntimeError>;

    /// Allocates a new node id useable for create_node
    fn allocate_node_id(&mut self, node_type: RENodeType) -> Result<RENodeId, RuntimeError>;

    /// Creates a new RENode
    /// TODO: Remove, replace with lock_substate + get_ref_mut use
    fn create_node(
        &mut self,
        node_id: RENodeId,
        init: RENodeInit,
        node_module_init: BTreeMap<NodeModuleId, RENodeModuleInit>,
    ) -> Result<(), RuntimeError>;
}

pub trait KernelSubstateApi {
    /// Locks a visible substate
    fn lock_substate(
        &mut self,
        node_id: RENodeId,
        module_id: NodeModuleId,
        offset: SubstateOffset,
        flags: LockFlags,
    ) -> Result<LockHandle, RuntimeError>;

    fn get_lock_info(&mut self, lock_handle: LockHandle) -> Result<LockInfo, RuntimeError>;

    /// Drops a lock
    fn drop_lock(&mut self, lock_handle: LockHandle) -> Result<(), RuntimeError>;

    /// Get a non-mutable reference to a locked substate
    fn get_ref(&mut self, lock_handle: LockHandle) -> Result<SubstateRef, RuntimeError>;

    /// Get a mutable reference to a locked substate
    fn get_ref_mut(&mut self, lock_handle: LockHandle) -> Result<SubstateRefMut, RuntimeError>;
}

pub trait KernelWasmApi<W: WasmEngine> {
    fn scrypto_interpreter(&mut self) -> Result<&ScryptoInterpreter<W>, RuntimeError>;

    fn emit_wasm_instantiation_event(&mut self, code: &[u8]) -> Result<(), RuntimeError>;
}

pub trait Invokable<I: Invocation, E> {
    fn invoke(&mut self, invocation: I) -> Result<I::Output, E>;
}

pub trait Executor {
    type Output: Debug;

    fn execute<Y, W>(self, api: &mut Y) -> Result<(Self::Output, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + KernelWasmApi<W> + ClientApi<RuntimeError>,
        W: WasmEngine;
}

pub trait ExecutableInvocation: Invocation {
    type Exec: Executor<Output = Self::Output>;

    fn resolve<Y: KernelSubstateApi + ClientDerefApi<RuntimeError>>(
        self,
        api: &mut Y,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError>;
}

pub trait KernelInvokeApi<E>:
    Invokable<EpochManagerCreateInvocation, E>
    + Invokable<EpochManagerNextRoundInvocation, E>
    + Invokable<EpochManagerGetCurrentEpochInvocation, E>
    + Invokable<EpochManagerSetEpochInvocation, E>
    + Invokable<EpochManagerUpdateValidatorInvocation, E>
    + Invokable<ValidatorRegisterInvocation, E>
    + Invokable<ValidatorUnregisterInvocation, E>
    + Invokable<ValidatorStakeInvocation, E>
    + Invokable<ValidatorUnstakeInvocation, E>
    + Invokable<ValidatorClaimXrdInvocation, E>
    + Invokable<ValidatorUpdateKeyInvocation, E>
    + Invokable<ValidatorUpdateAcceptDelegatedStakeInvocation, E>
    + Invokable<EpochManagerCreateValidatorInvocation, E>
    + Invokable<ClockCreateInvocation, E>
    + Invokable<ClockSetCurrentTimeInvocation, E>
    + Invokable<ClockGetCurrentTimeInvocation, E>
    + Invokable<ClockCompareCurrentTimeInvocation, E>
    + Invokable<MetadataSetInvocation, E>
    + Invokable<MetadataGetInvocation, E>
    + Invokable<AccessRulesAddAccessCheckInvocation, E>
    + Invokable<AccessRulesSetMethodAccessRuleInvocation, E>
    + Invokable<AccessRulesSetMethodMutabilityInvocation, E>
    + Invokable<AccessRulesSetGroupAccessRuleInvocation, E>
    + Invokable<AccessRulesSetGroupMutabilityInvocation, E>
    + Invokable<AccessRulesGetLengthInvocation, E>
    + Invokable<AuthZonePopInvocation, E>
    + Invokable<AuthZonePushInvocation, E>
    + Invokable<AuthZoneCreateProofInvocation, E>
    + Invokable<AuthZoneCreateProofByAmountInvocation, E>
    + Invokable<AuthZoneCreateProofByIdsInvocation, E>
    + Invokable<AuthZoneClearInvocation, E>
    + Invokable<AuthZoneDrainInvocation, E>
    + Invokable<AuthZoneAssertAccessRuleInvocation, E>
    + Invokable<AccessRulesAddAccessCheckInvocation, E>
    + Invokable<ComponentGlobalizeInvocation, E>
    + Invokable<ComponentGlobalizeWithOwnerInvocation, E>
    + Invokable<ComponentSetRoyaltyConfigInvocation, E>
    + Invokable<ComponentClaimRoyaltyInvocation, E>
    + Invokable<PackageSetRoyaltyConfigInvocation, E>
    + Invokable<PackageClaimRoyaltyInvocation, E>
    + Invokable<PackagePublishInvocation, E>
    + Invokable<BucketTakeInvocation, E>
    + Invokable<BucketPutInvocation, E>
    + Invokable<BucketTakeNonFungiblesInvocation, E>
    + Invokable<BucketGetNonFungibleLocalIdsInvocation, E>
    + Invokable<BucketGetAmountInvocation, E>
    + Invokable<BucketGetResourceAddressInvocation, E>
    + Invokable<BucketCreateProofInvocation, E>
    + Invokable<BucketCreateProofInvocation, E>
    + Invokable<ProofCloneInvocation, E>
    + Invokable<ProofGetAmountInvocation, E>
    + Invokable<ProofGetNonFungibleLocalIdsInvocation, E>
    + Invokable<ProofGetResourceAddressInvocation, E>
    + Invokable<ResourceManagerBurnBucketInvocation, E>
    + Invokable<ResourceManagerCreateNonFungibleInvocation, E>
    + Invokable<ResourceManagerCreateFungibleInvocation, E>
    + Invokable<ResourceManagerCreateNonFungibleWithInitialSupplyInvocation, E>
    + Invokable<ResourceManagerCreateUuidNonFungibleWithInitialSupplyInvocation, E>
    + Invokable<ResourceManagerCreateFungibleWithInitialSupplyInvocation, E>
    + Invokable<ResourceManagerBurnInvocation, E>
    + Invokable<ResourceManagerUpdateVaultAuthInvocation, E>
    + Invokable<ResourceManagerSetVaultAuthMutabilityInvocation, E>
    + Invokable<ResourceManagerCreateVaultInvocation, E>
    + Invokable<ResourceManagerCreateBucketInvocation, E>
    + Invokable<ResourceManagerMintNonFungibleInvocation, E>
    + Invokable<ResourceManagerMintUuidNonFungibleInvocation, E>
    + Invokable<ResourceManagerMintFungibleInvocation, E>
    + Invokable<ResourceManagerGetResourceTypeInvocation, E>
    + Invokable<ResourceManagerGetTotalSupplyInvocation, E>
    + Invokable<ResourceManagerUpdateNonFungibleDataInvocation, E>
    + Invokable<ResourceManagerNonFungibleExistsInvocation, E>
    + Invokable<ResourceManagerGetNonFungibleInvocation, E>
    + Invokable<VaultTakeInvocation, E>
    + Invokable<VaultPutInvocation, E>
    + Invokable<VaultLockFeeInvocation, E>
    + Invokable<VaultTakeNonFungiblesInvocation, E>
    + Invokable<VaultGetAmountInvocation, E>
    + Invokable<VaultGetResourceAddressInvocation, E>
    + Invokable<VaultGetNonFungibleLocalIdsInvocation, E>
    + Invokable<VaultCreateProofInvocation, E>
    + Invokable<VaultCreateProofByAmountInvocation, E>
    + Invokable<VaultCreateProofByIdsInvocation, E>
    + Invokable<VaultRecallInvocation, E>
    + Invokable<VaultRecallNonFungiblesInvocation, E>
    + Invokable<WorktopPutInvocation, E>
    + Invokable<WorktopTakeAmountInvocation, E>
    + Invokable<WorktopTakeAllInvocation, E>
    + Invokable<WorktopTakeNonFungiblesInvocation, E>
    + Invokable<WorktopAssertContainsInvocation, E>
    + Invokable<WorktopAssertContainsAmountInvocation, E>
    + Invokable<WorktopAssertContainsNonFungiblesInvocation, E>
    + Invokable<WorktopDrainInvocation, E>
    + Invokable<IdentityCreateInvocation, E>
    + Invokable<TransactionRuntimeGetHashInvocation, E>
    + Invokable<TransactionRuntimeGenerateUuidInvocation, E>
    + Invokable<LoggerLogInvocation, E>
    + Invokable<AccessControllerCreateGlobalInvocation, E>
    + Invokable<AccessControllerCreateProofInvocation, E>
    + Invokable<AccessControllerInitiateRecoveryAsPrimaryInvocation, E>
    + Invokable<AccessControllerInitiateRecoveryAsRecoveryInvocation, E>
    + Invokable<AccessControllerQuickConfirmPrimaryRoleRecoveryProposalInvocation, E>
    + Invokable<AccessControllerQuickConfirmRecoveryRoleRecoveryProposalInvocation, E>
    + Invokable<AccessControllerTimedConfirmRecoveryInvocation, E>
    + Invokable<AccessControllerCancelPrimaryRoleRecoveryProposalInvocation, E>
    + Invokable<AccessControllerCancelRecoveryRoleRecoveryProposalInvocation, E>
    + Invokable<AccessControllerLockPrimaryRoleInvocation, E>
    + Invokable<AccessControllerUnlockPrimaryRoleInvocation, E>
    + Invokable<AccessControllerStopTimedRecoveryInvocation, E>
    + Invokable<AccountCreateInvocation, E>
    + Invokable<AccountNewInvocation, E>
    + Invokable<AccountBalanceInvocation, E>
    + Invokable<AccountLockFeeInvocation, E>
    + Invokable<AccountLockContingentFeeInvocation, E>
    + Invokable<AccountDepositInvocation, E>
    + Invokable<AccountDepositBatchInvocation, E>
    + Invokable<AccountWithdrawInvocation, E>
    + Invokable<AccountWithdrawByAmountInvocation, E>
    + Invokable<AccountWithdrawByIdsInvocation, E>
    + Invokable<AccountLockFeeAndWithdrawInvocation, E>
    + Invokable<AccountLockFeeAndWithdrawByAmountInvocation, E>
    + Invokable<AccountLockFeeAndWithdrawByIdsInvocation, E>
    + Invokable<AccountCreateProofInvocation, E>
    + Invokable<AccountCreateProofByAmountInvocation, E>
    + Invokable<AccountCreateProofByIdsInvocation, E>
{
}

/// Interface of the Kernel, for Kernel modules.
pub trait KernelApi<W: WasmEngine, E>:
    KernelNodeApi + KernelSubstateApi + KernelWasmApi<W> + KernelInvokeApi<E>
{
}
