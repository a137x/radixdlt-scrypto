use transaction::errors::*;

use crate::engine::REActor;
use crate::model::*;
use crate::types::*;
use crate::wasm::WasmError;
use sbor::*;
use scrypto::core::{FnIdent, ReceiverMethodIdent};

use super::AuthError;
use super::CostingError;
use super::ExecutionTraceError;
use super::TrackError;

/// Represents an error which causes a tranasction to be rejected.
#[derive(Debug, Clone, PartialEq, Eq, TypeId, Encode, Decode)]
pub enum RejectionError {
    SuccessButFeeLoanNotRepaid,
    ErrorBeforeFeeLoanRepaid(RuntimeError),
}

impl fmt::Display for RejectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents an error when executing a transaction.
#[derive(Debug, Clone, PartialEq, Eq, TypeId, Encode, Decode)]
pub enum RuntimeError {
    /// An error occurred within the kernel.
    KernelError(KernelError),

    /// An error occurred within a kernel module.
    ModuleError(ModuleError),

    /// An error occurred within application logic, like the RE models.
    ApplicationError(ApplicationError),
}

impl From<KernelError> for RuntimeError {
    fn from(error: KernelError) -> Self {
        RuntimeError::KernelError(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum KernelError {
    // invocation
    WasmError(WasmError),
    RENodeNotVisible(RENodeId),
    InvokeMethodInvalidReceiver(RENodeId),

    InvalidReferencePass(GlobalAddress),
    InvalidReferenceReturn(GlobalAddress),
    InvalidReferenceWrite(GlobalAddress),
    GlobalAddressNotFound(GlobalAddress),

    MaxCallDepthLimitReached,
    FnIdentNotFound(FnIdent),
    MethodNotFound(ReceiverMethodIdent),
    FunctionNotFound(FunctionIdent),
    InvalidFnInput2(FnIdent),
    InvalidFnInput { fn_identifier: FunctionIdent },
    InvalidFnOutput { fn_identifier: FunctionIdent },

    // ID allocation
    IdAllocationError(IdAllocationError),

    // SBOR decoding
    DecodeError(DecodeError),

    // RENode
    BucketNotFound(BucketId),
    ProofNotFound(ProofId),
    PackageNotFound(PackageAddress),
    BlueprintNotFound(PackageAddress, String),
    ResourceManagerNotFound(ResourceAddress),
    WorktopNotFound,
    RENodeNotFound(RENodeId),
    StoredNodeRemoved(RENodeId),
    RENodeGlobalizeTypeNotAllowed(RENodeId),
    RENodeCreateInvalidPermission,
    RENodeCreateNodeNotFound(RENodeId),
    RENodeAlreadyTouched,
    RENodeNotInTrack,

    // Substate
    MovingLockedRENode(RENodeId),
    LockDoesNotExist(LockHandle),
    LockNotMutable(LockHandle),
    OffsetNotAvailable(SubstateOffset),
    InvalidOverwrite,
    SubstateNotReadable(REActor, SubstateId),
    SubstateNotWriteable(REActor, SubstateId),
    TrackError(TrackError),

    // constraints
    ValueNotAllowed,
    BucketNotAllowed,
    ProofNotAllowed,
    VaultNotAllowed,
    KeyValueStoreNotAllowed,
    CantMoveLockedBucket,
    CantMoveRestrictedProof,
    CantMoveWorktop,
    CantMoveGlobal,
    CantMoveAuthZone,
    DropFailure(DropFailure),

    BlobNotFound(Hash),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum ModuleError {
    AuthError(AuthError),
    CostingError(CostingError),
    ExecutionTraceError(ExecutionTraceError),
}

#[derive(Debug)]
pub enum InvokeError<E> {
    Error(E),
    Downstream(RuntimeError),
}

impl<E> From<RuntimeError> for InvokeError<E> {
    fn from(runtime_error: RuntimeError) -> Self {
        InvokeError::Downstream(runtime_error)
    }
}

impl<E> InvokeError<E> {
    pub fn error(error: E) -> Self {
        InvokeError::Error(error)
    }

    pub fn downstream(runtime_error: RuntimeError) -> Self {
        InvokeError::Downstream(runtime_error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypeId, Encode, Decode)]
pub enum ApplicationError {
    TransactionProcessorError(TransactionProcessorError),

    PackageError(PackageError),

    SystemError(SystemError),

    ResourceManagerError(ResourceManagerError),

    ComponentError(ComponentError),

    BucketError(BucketError),

    ProofError(ProofError),

    VaultError(VaultError),

    WorktopError(WorktopError),

    AuthZoneError(AuthZoneError),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum DropFailure {
    System,
    Resource,
    Component,
    Bucket,
    Worktop,
    Vault,
    Package,
    KeyValueStore,
    NonFungibleStore,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
