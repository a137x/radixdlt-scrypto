use crate::engine::*;
use crate::fee::{FeeReserve, FeeReserveError, SystemApiCostingEntry};
use crate::model::Resource;
use crate::types::*;
use radix_engine_interface::api::types::{RENodeId, VaultId};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum CostingError {
    FeeReserveError(FeeReserveError),
}

#[derive(Default)]
pub struct CostingModule;

impl<R: FeeReserve> Module<R> for CostingModule {
    fn pre_sys_call(
        &mut self,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        track: &mut Track<R>,
        input: SysCallInput,
    ) -> Result<(), ModuleError> {
        match input {
            SysCallInput::Invoke {
                depth,
                input_size,
                value_count,
                ..
            } => {
                if depth > 0 {
                    track
                        .fee_reserve
                        .consume_execution(
                            track
                                .fee_table
                                .system_api_cost(SystemApiCostingEntry::Invoke {
                                    input_size,
                                    value_count,
                                }),
                            1,
                            "invoke",
                            false,
                        )
                        .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
                }
            }
            SysCallInput::ReadOwnedNodes => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::ReadOwnedNodes),
                        1,
                        "read_owned_nodes",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::BorrowNode { node_id } => {
                track
                    .fee_reserve
                    .consume_execution(
                        track.fee_table.system_api_cost({
                            match node_id {
                                RENodeId::Global(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::AuthZoneStack(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::FeeReserve(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::Bucket(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::Proof(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::Worktop => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: true,
                                    size: 0,
                                },
                                RENodeId::Vault(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::Component(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::KeyValueStore(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::NonFungibleStore(_) => {
                                    SystemApiCostingEntry::BorrowNode {
                                        // TODO: figure out loaded state and size
                                        loaded: false,
                                        size: 0,
                                    }
                                }
                                RENodeId::ResourceManager(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::Package(_) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::EpochManager(..) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                                RENodeId::Clock(..) => SystemApiCostingEntry::BorrowNode {
                                    // TODO: figure out loaded state and size
                                    loaded: false,
                                    size: 0,
                                },
                            }
                        }),
                        1,
                        "borrow_node",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::DropNode { .. } => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::DropNode { size: 0 }),
                        1,
                        "drop_node",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::CreateNode { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::CreateNode {
                                size: 0, // TODO: get size of the value
                            }),
                        1,
                        "create_node",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::LockSubstate { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::LockSubstate {
                                size: 0, // TODO: get size of the value
                            }),
                        1,
                        "lock_substate",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::GetRef { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::ReadSubstate {
                                size: 0, // TODO: get size of the value
                            }),
                        1,
                        "read_substate",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::GetRefMut { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::WriteSubstate {
                                size: 0, // TODO: get size of the value
                            }),
                        1,
                        "write_substate",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::DropLock { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::DropLock),
                        1,
                        "drop_lock",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::TakeSubstate { .. } => {
                // Costing
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::TakeSubstate {
                                size: 0, // TODO: get size of the value
                            }),
                        1,
                        "take_substate",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::ReadTransactionHash => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::ReadTransactionHash),
                        1,
                        "read_transaction_hash",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::ReadBlob { .. } => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::ReadBlob { size: 0 }), // TODO pass the right size
                        1,
                        "read_blob",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::GenerateUuid => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::GenerateUuid),
                        1,
                        "generate_uuid",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::EmitLog { message, .. } => {
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::EmitLog {
                                size: message.len() as u32,
                            }),
                        1,
                        "emit_log",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
            SysCallInput::EmitEvent { event } => {
                let (native, tracked, size) = match event {
                    Event::Runtime(_) => (true, false, 0),
                    Event::Tracked(TrackedEvent::Native(..)) => (true, true, 0),
                    Event::Tracked(TrackedEvent::Scrypto(value)) => {
                        (false, true, value.len() as u32)
                    }
                };
                track
                    .fee_reserve
                    .consume_execution(
                        track
                            .fee_table
                            .system_api_cost(SystemApiCostingEntry::EmitEvent {
                                native,
                                tracked,
                                size,
                            }),
                        1,
                        "emit_event",
                        false,
                    )
                    .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))?;
            }
        }

        Ok(())
    }

    fn post_sys_call(
        &mut self,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        _track: &mut Track<R>,
        _output: SysCallOutput,
    ) -> Result<(), ModuleError> {
        Ok(())
    }

    fn on_wasm_instantiation(
        &mut self,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        track: &mut Track<R>,
        code: &[u8],
    ) -> Result<(), ModuleError> {
        track
            .fee_reserve
            .consume_execution(
                track.fee_table.wasm_instantiation_per_byte(),
                code.len(),
                "instantiate_wasm",
                false,
            )
            .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))
    }

    fn on_wasm_costing(
        &mut self,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        track: &mut Track<R>,
        units: u32,
    ) -> Result<(), ModuleError> {
        track
            .fee_reserve
            .consume_execution(units, 1, "run_wasm", false)
            .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))
    }

    fn on_lock_fee(
        &mut self,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        track: &mut Track<R>,
        vault_id: VaultId,
        fee: Resource,
        contingent: bool,
    ) -> Result<Resource, ModuleError> {
        track
            .fee_reserve
            .lock_fee(vault_id, fee, contingent)
            .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e)))
    }

    fn pre_execute_invocation(
        &mut self,
        actor: &REActor,
        _call_frame_update: &CallFrameUpdate,
        _call_frame: &CallFrame,
        _heap: &mut Heap,
        track: &mut Track<R>,
    ) -> Result<(), ModuleError> {
        match actor {
            REActor::Function(ResolvedFunction::Native(native_function)) => track
                .fee_reserve
                .consume_execution(
                    track.fee_table.run_native_function_cost(&native_function),
                    1,
                    "run_native_function",
                    false,
                )
                .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e))),
            REActor::Method(ResolvedMethod::Native(native_method), _) => track
                .fee_reserve
                .consume_execution(
                    track.fee_table.run_native_method_cost(&native_method),
                    1,
                    "run_native_method",
                    false,
                )
                .map_err(|e| ModuleError::CostingError(CostingError::FeeReserveError(e))),
            _ => Ok(()),
        }
    }
}
