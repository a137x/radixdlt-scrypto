//! This module contains the internal representation of the testing environment which is a self
//! contained Radix Engine implemented as a self-referencing struct.

use super::*;
use crate::prelude::*;

/// The implementation of a self-contained Radix Engine.
///
/// This is a self-contained Radix Engine that uses the [`ouroboros`] crate for self-referencing to
/// allow the entire Radix Engine stack to be stored in a single struct where some members reference
/// one another. As an example: the [`Track`] references the substate database stored in the same
/// object as it.
#[ouroboros::self_referencing(no_doc)]
pub(super) struct EncapsulatedRadixEngine {
    pub(super) substate_db: InMemorySubstateDatabase,
    pub(super) scrypto_vm: ScryptoVm<DefaultWasmEngine>,
    pub(super) native_vm: NativeVm<NoExtension>,
    pub(super) id_allocator: IdAllocator,

    #[borrows(substate_db)]
    #[covariant]
    pub(super) track: TestTrack<'this>,

    #[borrows(scrypto_vm)]
    #[covariant]
    pub(super) system_config: TestSystemConfig<'this>,

    #[borrows(mut system_config, mut track, mut id_allocator)]
    #[not_covariant]
    pub(super) kernel: TestKernel<'this>,
}

impl EncapsulatedRadixEngine {
    pub(super) fn create(
        substate_db: InMemorySubstateDatabase,
        scrypto_vm: ScryptoVm<DefaultWasmEngine>,
        native_vm: NativeVm<NoExtension>,
        id_allocator: IdAllocator,
        track_builder: impl FnOnce(&InMemorySubstateDatabase) -> TestTrack<'_>,
        system_config_builder: impl FnOnce(&ScryptoVm<DefaultWasmEngine>) -> TestSystemConfig<'_>,
        kernel_builder: impl for<'a> FnOnce(
            &'a mut TestSystemConfig<'a>,
            &'a mut TestTrack<'a>,
            &'a mut IdAllocator,
        ) -> TestKernel<'a>,
    ) -> Self {
        EncapsulatedRadixEngineBuilder {
            substate_db,
            scrypto_vm,
            native_vm,
            id_allocator,
            track_builder,
            system_config_builder,
            kernel_builder,
        }
        .build()
    }
}
