use crate::engine::{Kernel, RuntimeError};
use crate::fee::FeeReserve;
use crate::wasm::WasmEngine;
use radix_engine_interface::api::api::InvokableModel;

impl<'g, 's, W, R> InvokableModel<RuntimeError> for Kernel<'g, 's, W, R>
where
    W: WasmEngine,
    R: FeeReserve,
{
}
