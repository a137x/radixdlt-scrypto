use crate::types::*;
use crate::vm::wasm::{constants::*, errors::*, PrepareError};
use radix_engine_interface::blueprints::package::BlueprintDefinitionInit;
use syn::Ident;
use wasm_instrument::{
    gas_metering::{self, Rules},
    inject_stack_limiter,
    utils::module_info::ModuleInfo,
};
use wasmparser::{ExternalKind, FuncType, Operator, Type, TypeRef, ValType, WasmFeatures};

use super::WasmiModule;
#[derive(Debug)]
pub struct WasmModule {
    module: ModuleInfo,
}

impl WasmModule {
    pub fn init(code: &[u8]) -> Result<Self, PrepareError> {
        // deserialize
        let module = ModuleInfo::new(code).map_err(|_| PrepareError::DeserializationError)?;

        // Radix Engine supports MVP + proposals: mutable globals and sign-extension-ops
        let features = WasmFeatures {
            mutable_global: true,
            saturating_float_to_int: false,
            sign_extension: true,
            reference_types: false,
            multi_value: false,
            bulk_memory: false,
            simd: false,
            relaxed_simd: false,
            threads: false,
            tail_call: false,
            floats: false,
            multi_memory: false,
            exceptions: false,
            memory64: false,
            extended_const: false,
            component_model: false,
            function_references: false,
            memory_control: false,
            gc: false,
        };

        module
            .validate(features)
            .map_err(|err| PrepareError::ValidationError(err.to_string()))?;

        Ok(Self { module })
    }

    pub fn enforce_no_start_function(self) -> Result<Self, PrepareError> {
        if self.module.start_function.is_some() {
            Err(PrepareError::StartFunctionNotAllowed)
        } else {
            Ok(self)
        }
    }

    pub fn enforce_import_limit(self) -> Result<Self, PrepareError> {
        // Only allow `env::radix_engine` import
        for entry in self
            .module
            .import_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .unwrap_or(vec![])
        {
            if entry.module == MODULE_ENV_NAME {
                match entry.name {
                    CONSUME_BUFFER_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    CONSUME_BUFFER_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    CALL_METHOD_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                ],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    CALL_METHOD_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    CALL_FUNCTION_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                ],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    CALL_METHOD_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    DROP_OBJECT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    DROP_OBJECT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_STORE_OPEN_ENTRY_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                ],
                                vec![ValType::I32],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_STORE_OPEN_ENTRY_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_ENTRY_GET_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_ENTRY_GET_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_ENTRY_SET_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_ENTRY_SET_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_ENTRY_RELEASE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_ENTRY_RELEASE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_STORE_REMOVE_ENTRY_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_STORE_REMOVE_ENTRY_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    ACTOR_OPEN_FIELD_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32],
                                vec![ValType::I32],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    ACTOR_OPEN_FIELD_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    ACTOR_CALL_MODULE_METHOD_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                    ValType::I32,
                                ],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    ACTOR_CALL_MODULE_METHOD_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FIELD_LOCK_READ_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FIELD_LOCK_READ_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FIELD_LOCK_WRITE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FIELD_LOCK_WRITE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FIELD_LOCK_RELEASE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FIELD_LOCK_RELEASE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_NODE_ID_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_NODE_ID_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_GLOBAL_ADDRESS_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_GLOBAL_ADDRESS_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_BLUEPRINT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_BLUEPRINT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_AUTH_ZONE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_AUTH_ZONE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }

                    NEW_OBJECT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }

                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    NEW_OBJECT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }

                    EXECUTION_COST_UNIT_LIMIT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I32],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    EXECUTION_COST_UNIT_LIMIT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    EXECUTION_COST_UNIT_PRICE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    EXECUTION_COST_UNIT_PRICE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FINALIZATION_COST_UNIT_LIMIT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I32],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FINALIZATION_COST_UNIT_LIMIT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FINALIZATION_COST_UNIT_PRICE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FINALIZATION_COST_UNIT_PRICE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    TIP_PERCENTAGE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I32],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    TIP_PERCENTAGE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    FEE_BALANCE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    FEE_BALANCE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }

                    ALLOCATE_GLOBAL_ADDRESS_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    ALLOCATE_GLOBAL_ADDRESS_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_RESERVATION_ADDRESS_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_RESERVATION_ADDRESS_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GLOBALIZE_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GLOBALIZE_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    KEY_VALUE_STORE_NEW_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    KEY_VALUE_STORE_NEW_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_BLUEPRINT_ID_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_BLUEPRINT_ID_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GET_OUTER_OBJECT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_OUTER_OBJECT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    EMIT_EVENT_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    EMIT_EVENT_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    EMIT_LOG_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    EMIT_LOG_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    PANIC_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![ValType::I32, ValType::I32],
                                vec![],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(PANIC_FUNCTION_NAME.to_string()),
                            ));
                        }
                    }
                    GET_TRANSACTION_HASH_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GET_TRANSACTION_HASH_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    GENERATE_RUID_FUNCTION_NAME => {
                        if let TypeRef::Func(type_index) = entry.ty {
                            if Self::function_type_matches(
                                &self.module,
                                type_index,
                                vec![],
                                vec![ValType::I64],
                            ) {
                                continue;
                            }
                            return Err(PrepareError::InvalidImport(
                                InvalidImport::InvalidFunctionType(
                                    GENERATE_RUID_FUNCTION_NAME.to_string(),
                                ),
                            ));
                        }
                    }
                    _ => {}
                };
            }

            return Err(PrepareError::InvalidImport(InvalidImport::ImportNotAllowed));
        }

        Ok(self)
    }

    pub fn enforce_memory_limit_and_inject_max(
        mut self,
        max_memory_size_in_pages: u32,
    ) -> Result<Self, PrepareError> {
        // Check if memory section exists
        let memory_section = self
            .module
            .memory_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .ok_or(PrepareError::InvalidMemory(
                InvalidMemory::MissingMemorySection,
            ))?;

        // Check if there is only one memory definition
        let mut memory = match memory_section.len() {
            0 => Err(PrepareError::InvalidMemory(
                InvalidMemory::NoMemoryDefinition,
            )),
            1 => Ok(memory_section[0]),
            _ => Err(PrepareError::InvalidMemory(
                InvalidMemory::TooManyMemoryDefinition,
            )),
        }?;

        // Check the memory limits
        if memory.initial > max_memory_size_in_pages.into() {
            return Err(PrepareError::InvalidMemory(
                InvalidMemory::MemorySizeLimitExceeded,
            ));
        }
        if let Some(max) = memory.maximum {
            if max > max_memory_size_in_pages.into() {
                return Err(PrepareError::InvalidMemory(
                    InvalidMemory::MemorySizeLimitExceeded,
                ));
            }
        } else {
            memory.maximum = Some(max_memory_size_in_pages.into());
            self.module
                .modify_memory_type(0, memory)
                .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?;
        }

        // Check if the memory is exported
        if !self
            .module
            .export_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .unwrap_or(vec![])
            .iter()
            .any(|e| e.kind == ExternalKind::Memory && e.name == EXPORT_MEMORY)
        {
            return Err(PrepareError::InvalidMemory(
                InvalidMemory::MemoryNotExported,
            ));
        }

        Ok(self)
    }

    pub fn enforce_table_limit(self, max_initial_table_size: u32) -> Result<Self, PrepareError> {
        let section = self
            .module
            .table_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?;

        if let Some(section) = section {
            if section.len() > 1 {
                // Sanity check MVP rule
                return Err(PrepareError::InvalidTable(InvalidTable::MoreThanOneTable));
            }

            if let Some(table) = section.get(0) {
                if table.ty.initial > max_initial_table_size {
                    return Err(PrepareError::InvalidTable(
                        InvalidTable::InitialTableSizeLimitExceeded,
                    ));
                }
            }
        }

        Ok(self)
    }

    pub fn enforce_br_table_limit(
        self,
        max_number_of_br_table_targets: u32,
    ) -> Result<Self, PrepareError> {
        for fb in self
            .module
            .code_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .unwrap_or(vec![])
        {
            let reader = fb
                .get_operators_reader()
                .map_err(|err| PrepareError::WasmParserError(err.to_string()))?;

            for op in reader {
                let inst = op.map_err(|err| PrepareError::WasmParserError(err.to_string()))?;

                if let Operator::BrTable {
                    targets: table_data,
                } = inst
                {
                    if table_data.len() > max_number_of_br_table_targets {
                        return Err(PrepareError::TooManyTargetsInBrTable);
                    }
                }
            }
        }
        Ok(self)
    }

    pub fn enforce_function_limit(
        self,
        max_number_of_functions: u32,
        max_number_of_function_params: u32,
        max_number_of_function_locals: u32,
    ) -> Result<Self, PrepareError> {
        if self.module.num_local_functions() > max_number_of_functions {
            return Err(PrepareError::TooManyFunctions);
        }

        for func_idx in 0..self.module.num_local_functions() {
            if let wasmparser::Type::Func(ty) = self
                .module
                .get_type_by_func_idx(func_idx)
                .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            {
                if ty.params().len() > max_number_of_function_params as usize {
                    return Err(PrepareError::TooManyFunctionParams);
                }
            }
        }

        for func_body in self
            .module
            .code_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .unwrap_or(vec![])
        {
            let local_reader = func_body
                .get_locals_reader()
                .map_err(|err| PrepareError::WasmParserError(err.to_string()))?;
            let mut locals_count = 0;

            // According to the documentation local_reader.get_count() would do the job here
            // see: https://docs.rs/wasmparser/latest/wasmparser/struct.LocalsReader.html#method.get_count
            // But the description is misleading, get_count() returns the number of different types of
            // locals (or number of LocalReader iterator items).
            // To get the number of locals we need to iterate over LocalReader, which
            // returns following tuple for each item:
            //  ( u32, ValType) - where u32 is the number of locals of ValType
            for local in local_reader.into_iter() {
                // Number of locals of some type
                let (count, _ty) =
                    local.map_err(|err| PrepareError::WasmParserError(err.to_string()))?;
                locals_count += count;
            }

            if locals_count > max_number_of_function_locals {
                return Err(PrepareError::TooManyFunctionLocals);
            }
        }

        Ok(self)
    }

    pub fn enforce_export_names(self) -> Result<Self, PrepareError> {
        // Any exported name should follow Rust Identifier specification
        for name in &self.module.export_names {
            syn::parse_str::<Ident>(name)
                .map_err(|_| PrepareError::InvalidExportName(name.to_string()))?;
        }

        Ok(self)
    }

    pub fn enforce_global_limit(self, max_number_of_globals: u32) -> Result<Self, PrepareError> {
        if self.module.num_local_globals() > max_number_of_globals {
            return Err(PrepareError::TooManyGlobals);
        }

        Ok(self)
    }

    pub fn enforce_export_constraints<'a, I: Iterator<Item = &'a BlueprintDefinitionInit>>(
        self,
        blueprints: I,
    ) -> Result<Self, PrepareError> {
        let exports = self
            .module
            .export_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?;

        if let Some(exports) = exports {
            for blueprint_def_init in blueprints {
                for export_name in blueprint_def_init.schema.exports() {
                    if !exports.iter().any(|x| {
                        x.name.eq(&export_name) && {
                            if let ExternalKind::Func = x.kind {
                                Self::function_matches(
                                    &self.module,
                                    x.index as usize,
                                    vec![ValType::I64],
                                    vec![ValType::I64],
                                )
                            } else {
                                false
                            }
                        }
                    }) {
                        return Err(PrepareError::MissingExport {
                            export_name: export_name.to_string(),
                        });
                    }
                }
            }

            Ok(self)
        } else {
            Err(PrepareError::NoExportSection)
        }
    }

    pub fn inject_instruction_metering<R: Rules>(
        mut self,
        rules: &R,
    ) -> Result<Self, PrepareError> {
        let backend = gas_metering::host_function::Injector::new(
            MODULE_ENV_NAME,
            CONSUME_WASM_EXECUTION_UNITS_FUNCTION_NAME,
        );
        gas_metering::inject(&mut self.module, backend, rules).map_err(|err| {
            PrepareError::RejectedByInstructionMetering {
                reason: err.to_string(),
            }
        })?;

        Ok(self)
    }

    pub fn inject_stack_metering(mut self, wasm_max_stack_size: u32) -> Result<Self, PrepareError> {
        inject_stack_limiter(&mut self.module, wasm_max_stack_size).map_err(|err| {
            PrepareError::RejectedByStackMetering {
                reason: err.to_string(),
            }
        })?;
        Ok(self)
    }

    pub fn ensure_instantiatable(self) -> Result<Self, PrepareError> {
        // During instantiation time, the following procedures are applied:

        // 1. Resolve imports with external values
        // This should always succeed as we only allow `env::radix_engine` function import

        // 2. Allocate externals, functions, tables, memory and globals
        // This should always succeed as we enforce an upper bound for each type

        // 3. Update table with elements
        // It may fail if the offset is out of bound

        // 4. Update memory with data segments
        // It may fail if the offset is out of bound

        // Because the offset can be an `InitExpr` that requires evaluation against an WASM instance,
        // we're using the `wasmi` logic as a shortcut.
        let code = self.module.bytes();
        WasmiModule::new(&code[..]).map_err(|e| PrepareError::NotInstantiatable {
            reason: format!("{:?}", e),
        })?;

        Ok(self)
    }

    pub fn ensure_compilable(self) -> Result<Self, PrepareError> {
        // TODO: Understand WASM JIT compilability
        //
        // Can we make the assumption that all "prepared" modules are compilable,
        // if machine resource is "sufficient"?
        //
        // Another option is to attempt to compile, although it may make RE protocol
        // coupled with a specific implementation.

        Ok(self)
    }

    pub fn to_bytes(self) -> Result<(Vec<u8>, Vec<String>), PrepareError> {
        let mut function_exports = vec![];

        for export in self
            .module
            .export_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))?
            .unwrap_or(vec![])
        {
            if let wasmparser::ExternalKind::Func = export.kind {
                function_exports.push(export.name.to_string());
            }
        }
        let code = self.module.bytes();

        Ok((code, function_exports))
    }

    fn function_matches(
        module: &ModuleInfo,
        func_index: usize,
        params: Vec<ValType>,
        results: Vec<ValType>,
    ) -> bool {
        match module.function_map.get(func_index) {
            Some(type_index) => Self::function_type_matches(module, *type_index, params, results),
            None => false,
        }
    }

    fn function_type_matches(
        module: &ModuleInfo,
        type_index: u32,
        params: Vec<ValType>,
        results: Vec<ValType>,
    ) -> bool {
        let ty = module.get_type_by_idx(type_index);
        match ty {
            Ok(ty) => match ty {
                Type::Func(ty) => ty == &FuncType::new(params, results),
                _ => false,
            },
            Err(_) => false,
        }
    }

    #[cfg(feature = "radix_engine_tests")]
    pub fn contains_sign_ext_ops(self) -> bool {
        for func_body in self
            .module
            .code_section()
            .map_err(|err| PrepareError::ModuleInfoError(err.to_string()))
            .unwrap()
            .expect("no code section")
        {
            let reader = func_body
                .get_operators_reader()
                .map_err(|err| PrepareError::WasmParserError(err.to_string()))
                .unwrap();
            for op in reader {
                let inst = op
                    .map_err(|err| PrepareError::WasmParserError(err.to_string()))
                    .unwrap();

                match inst {
                    Operator::I32Extend8S
                    | Operator::I32Extend16S
                    | Operator::I64Extend8S
                    | Operator::I64Extend16S
                    | Operator::I64Extend32S => return true,
                    _ => (),
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use radix_engine_interface::blueprints::package::BlueprintType;
    use radix_engine_interface::schema::{
        BlueprintFunctionsSchemaInit, BlueprintSchemaInit, BlueprintStateSchemaInit, FieldSchema,
        FunctionSchemaInit, TypeRef,
    };
    use sbor::basic_well_known_types::{ANY_TYPE, UNIT_TYPE};
    use wabt::wat2wasm;

    macro_rules! assert_invalid_wasm {
        ($wat: expr, $err: expr) => {
            let code = wat2wasm($wat).unwrap();
            assert_eq!($err, WasmModule::init(&code).unwrap_err());
        };

        ($wat: expr, $err: expr, $func: expr) => {
            let code = wat2wasm($wat).unwrap();
            assert_eq!($err, WasmModule::init(&code).and_then($func).unwrap_err());
        };
    }

    #[test]
    fn test_floating_point() {
        // return
        assert_invalid_wasm!(
            r#"
            (module
                (func (result f64)
                    f64.const 123
                )
            )
            "#,
            PrepareError::ValidationError(
                "WasmParserError(BinaryReaderError { floating-point support is disabled (at offset 0xb) })".to_string()
            )
        );
        // input
        assert_invalid_wasm!(
            r#"
            (module
                (func (param f64)
                )
            )
            "#,
            PrepareError::ValidationError(
                "WasmParserError(BinaryReaderError { floating-point support is disabled (at offset 0xb) })".to_string()
            )
        );
        // instruction
        assert_invalid_wasm!(
            r#"
            (module
                (func
                    f64.const 1
                    f64.const 2
                    f64.add
                    drop
                )
            )
            "#,
            PrepareError::ValidationError(
                "WasmParserError(BinaryReaderError { floating-point instruction disallowed (at offset 0x17) })".to_string()
            )
        );
        // global
        assert_invalid_wasm!(
            r#"
            (module
                (global $fp f32 (f32.const 10))
            )
            "#,
            PrepareError::ValidationError(
                "WasmParserError(BinaryReaderError { floating-point support is disabled (at offset 0xb) })".to_string()
            )
        );
    }

    #[test]
    fn test_start_function() {
        assert_invalid_wasm!(
            r#"
            (module
                (func $main)
                (start $main)
            )
            "#,
            PrepareError::StartFunctionNotAllowed,
            WasmModule::enforce_no_start_function
        );
    }

    #[test]
    fn test_memory() {
        assert_invalid_wasm!(
            r#"
            (module
            )
            "#,
            PrepareError::InvalidMemory(InvalidMemory::MissingMemorySection),
            |x| WasmModule::enforce_memory_limit_and_inject_max(x, 5)
        );
        // NOTE: Disabled as MVP only allow 1 memory definition
        // assert_invalid_wasm!(
        //     r#"
        //     (module
        //         (memory 2)
        //         (memory 2)
        //     )
        //     "#,
        //     PrepareError::InvalidMemory(InvalidMemory::TooManyMemories),
        //     |x| WasmModule::enforce_memory_limit(x, 5)
        // );
        assert_invalid_wasm!(
            r#"
            (module
                (memory 6)
            )
            "#,
            PrepareError::InvalidMemory(InvalidMemory::MemorySizeLimitExceeded),
            |x| WasmModule::enforce_memory_limit_and_inject_max(x, 5)
        );
        assert_invalid_wasm!(
            r#"
            (module
                (memory 2)
            )
            "#,
            PrepareError::InvalidMemory(InvalidMemory::MemoryNotExported),
            |x| WasmModule::enforce_memory_limit_and_inject_max(x, 5)
        );
    }

    #[test]
    fn test_table() {
        assert_invalid_wasm!(
            r#"
            (module
                (table 6 funcref)
            )
            "#,
            PrepareError::InvalidTable(InvalidTable::InitialTableSizeLimitExceeded),
            |x| WasmModule::enforce_table_limit(x, 5)
        );
    }

    #[test]
    fn test_br_table() {
        assert_invalid_wasm!(
            r#"
            (module
                (func (param i32) (result i32)
                    (block
                        (block
                            (br_table 1 0 1 0 1 0 1 (local.get 0))
                            (return (i32.const 21))
                        )
                        (return (i32.const 20))
                    )
                    (i32.const 22)
                )
            )
            "#,
            PrepareError::TooManyTargetsInBrTable,
            |x| WasmModule::enforce_br_table_limit(x, 5)
        );
    }

    #[test]
    fn test_function_limits() {
        assert_invalid_wasm!(
            r#"
            (module
                (func (result i32)
                    (i32.const 11)
                )
                (func (result i32)
                    (i32.const 22)
                )
                (func (result i32)
                    (i32.const 33)
                )
            )
            "#,
            PrepareError::TooManyFunctions,
            |x| WasmModule::enforce_function_limit(x, 2, 3, 3)
        );

        assert_invalid_wasm!(
            r#"
            (module
                (func (param i32 i32 i32 i32) (result i32)
                    (i32.const 22)
                )
            )
            "#,
            PrepareError::TooManyFunctionParams,
            |x| WasmModule::enforce_function_limit(x, 2, 3, 3)
        );

        assert_invalid_wasm!(
            r#"
            (module
                (func (result i32)
                    (local $v1 i32)

                    (local.set $v1 (i32.const 1))

                    (i32.const 22)
                )
                (func (result i32)
                    (local $v1 i32)
                    (local $v2 i64)
                    (local $v3 i64)
                    (local $v4 i32)

                    (local.set $v1 (i32.const 1))
                    (local.set $v2 (i64.const 2))
                    (local.set $v3 (i64.const 3))
                    (local.set $v4 (i32.const 4))

                    (i32.const 22)
                )
            )
            "#,
            PrepareError::TooManyFunctionLocals,
            |x| WasmModule::enforce_function_limit(x, 2, 3, 3)
        );
    }

    #[test]
    fn test_blueprint_constraints() {
        let mut blueprints = BTreeMap::new();
        blueprints.insert(
            "Test".to_string(),
            BlueprintDefinitionInit {
                blueprint_type: BlueprintType::default(),
                is_transient: false,
                feature_set: btreeset!(),
                dependencies: btreeset!(),

                schema: BlueprintSchemaInit {
                    generics: vec![],
                    schema: ScryptoSchema {
                        type_kinds: vec![],
                        type_metadata: vec![],
                        type_validations: vec![],
                    },
                    state: BlueprintStateSchemaInit {
                        fields: vec![FieldSchema::static_field(LocalTypeIndex::WellKnown(
                            UNIT_TYPE,
                        ))],
                        collections: vec![],
                    },
                    events: Default::default(),
                    functions: BlueprintFunctionsSchemaInit {
                        functions: btreemap!(
                            "f".to_string() => FunctionSchemaInit {
                                receiver: Option::None,
                                input: TypeRef::Static(LocalTypeIndex::WellKnown(ANY_TYPE)),
                                output: TypeRef::Static(LocalTypeIndex::WellKnown(UNIT_TYPE)),
                                export: "Test_f".to_string(),
                            }
                        ),
                    },
                    hooks: BlueprintHooksInit::default(),
                },

                royalty_config: Default::default(),
                auth_config: Default::default(),
            },
        );

        assert_invalid_wasm!(
            r#"
            (module
            )
            "#,
            PrepareError::NoExportSection,
            |x| WasmModule::enforce_export_constraints(x, blueprints.values())
        );
        // symbol not found
        assert_invalid_wasm!(
            r#"
            (module
                (func (export "foo") (result i32)
                    (i32.const 0)
                )
            )
            "#,
            PrepareError::MissingExport {
                export_name: "Test_f".to_string()
            },
            |x| WasmModule::enforce_export_constraints(x, blueprints.values())
        );
        // signature does not match
        assert_invalid_wasm!(
            r#"
            (module
                (func (export "Test_f") (result i32)
                    (i32.const 0)
                )
            )
            "#,
            PrepareError::MissingExport {
                export_name: "Test_f".to_string()
            },
            |x| WasmModule::enforce_export_constraints(x, blueprints.values())
        );
    }
}
