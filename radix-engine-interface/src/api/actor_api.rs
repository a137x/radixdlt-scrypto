use crate::types::*;
use radix_engine_interface::api::LockFlags;
use sbor::rust::fmt::Debug;

/// Api which exposes methods in the context of the actor
pub trait ClientActorApi<E: Debug> {
    /// Lock a field in the current object actor for reading/writing
    fn lock_field(&mut self, field: u8, flags: LockFlags) -> Result<LockHandle, E>;

    /// Lock a field in the current object actor for reading/writing
    fn lock_transient_field(&mut self, field: u8, flags: LockFlags) -> Result<LockHandle, E>;

    // TODO: Should this be exposed as a virtual field instead?
    /// Lock a field in the current object actor's parent for reading/writing
    fn lock_parent_field(&mut self, field: u8, flags: LockFlags) -> Result<LockHandle, E>;

    // TODO: Add specific object read/write lock apis

    fn get_info(&mut self) -> Result<ObjectInfo, E>;

    fn get_global_address(&mut self) -> Result<GlobalAddress, E>;

    fn get_blueprint(&mut self) -> Result<Blueprint, E>;
}
