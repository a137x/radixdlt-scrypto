use super::{CodeKey, MeteredCodeKey, PrepareError};
use crate::types::*;
use crate::vm::wasm::{WasmInstrumenterConfigV1, WasmModule};
use sbor::rust::sync::Arc;

pub const DEFAULT_CACHE_SIZE: usize = 1000;

pub struct WasmInstrumenter {
    // This flag disables cache in wasm_instrumenter/wasmi/wasmer to prevent non-determinism when fuzzing
    #[cfg(all(not(feature = "radix_engine_fuzzing"), not(feature = "moka")))]
    cache: RefCell<lru::LruCache<MeteredCodeKey, Arc<Vec<u8>>>>,
    #[cfg(all(not(feature = "radix_engine_fuzzing"), feature = "moka"))]
    cache: moka::sync::Cache<MeteredCodeKey, Arc<Vec<u8>>>,
    #[cfg(feature = "radix_engine_fuzzing")]
    #[allow(dead_code)]
    cache: usize,
}

#[derive(Debug, Clone)]
pub struct InstrumenterOptions {
    max_cache_size: usize,
}

impl Default for InstrumenterOptions {
    fn default() -> Self {
        InstrumenterOptions {
            max_cache_size: DEFAULT_CACHE_SIZE,
        }
    }
}

impl Default for WasmInstrumenter {
    fn default() -> Self {
        Self::new(InstrumenterOptions::default())
    }
}

pub struct InstrumentedCode {
    pub metered_code_key: MeteredCodeKey,
    pub code: Arc<Vec<u8>>,
}

impl WasmInstrumenter {
    pub fn new(options: InstrumenterOptions) -> Self {
        #[cfg(all(not(feature = "radix_engine_fuzzing"), not(feature = "moka")))]
        let cache = RefCell::new(lru::LruCache::new(
            NonZeroUsize::new(options.max_cache_size).unwrap(),
        ));
        #[cfg(all(not(feature = "radix_engine_fuzzing"), feature = "moka"))]
        let cache = moka::sync::Cache::builder()
            .weigher(|_key: &MeteredCodeKey, _value: &Arc<Vec<u8>>| -> u32 {
                // No sophisticated weighing mechanism, just keep a fixed size cache
                1u32
            })
            .max_capacity(options.max_cache_size as u64)
            .build();
        #[cfg(feature = "radix_engine_fuzzing")]
        let cache = options.max_cache_size;

        Self { cache }
    }

    pub fn instrument(
        &self,
        code_key: CodeKey,
        code: &[u8],
        wasm_instrumenter_config: &WasmInstrumenterConfigV1,
    ) -> Result<InstrumentedCode, PrepareError> {
        let metered_code_key = (code_key, wasm_instrumenter_config.version());

        #[cfg(not(feature = "radix_engine_fuzzing"))]
        {
            #[cfg(not(feature = "moka"))]
            {
                if let Some(cached) = self.cache.borrow_mut().get(&metered_code_key) {
                    return Ok(InstrumentedCode {
                        metered_code_key,
                        code: cached.clone(),
                    });
                }
            }
            #[cfg(feature = "moka")]
            if let Some(cached) = self.cache.get(&metered_code_key) {
                return Ok(InstrumentedCode {
                    metered_code_key,
                    code: cached.clone(),
                });
            }
        }

        let instrumented_ref = Arc::new(self.instrument_no_cache(code, &wasm_instrumenter_config)?);

        #[cfg(not(feature = "radix_engine_fuzzing"))]
        {
            #[cfg(not(feature = "moka"))]
            self.cache
                .borrow_mut()
                .put(metered_code_key, instrumented_ref.clone());
            #[cfg(feature = "moka")]
            self.cache
                .insert(metered_code_key, instrumented_ref.clone());
        }

        Ok(InstrumentedCode {
            metered_code_key,
            code: instrumented_ref,
        })
    }

    pub fn instrument_no_cache(
        &self,
        code: &[u8],
        instrumenter_config: &WasmInstrumenterConfigV1,
    ) -> Result<Vec<u8>, PrepareError> {
        WasmModule::init(code)
            .and_then(|m| m.inject_instruction_metering(instrumenter_config))
            .and_then(|m| m.inject_stack_metering(instrumenter_config.max_stack_size()))
            .and_then(|m| m.to_bytes())
            .map(|m| m.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::wasm::{WasmInstrumenterConfigV1, WasmModule};
    use wabt::wat2wasm;

    #[test]
    fn test_cost_rules() {
        let code = wat2wasm(
            r#"
            (module
                (func (param $p0 i32) (result i32)
                    local.get $p0
                    i32.const 5
                    i32.mul
                )
                (func (param $p0 i32) (result i32)
                    local.get $p0
                    call 0
                )
            )
            "#,
        )
        .unwrap();
        let config = WasmInstrumenterConfigV1::new();
        let transformed = WasmModule::init(&code)
            .unwrap()
            .inject_instruction_metering(&config)
            .unwrap()
            .to_bytes()
            .unwrap()
            .0;

        // Costs:
        // 3 = 1 (local.get) + 1 (i32.const) + 1 (i32.mul)
        // 2 = 1 (local.get) + 1 (call)
        let expected = wat2wasm(
            r#"
            (module
                (type (;0;) (func (param i32) (result i32)))
                (type (;1;) (func (param i32)))
                (import "env" "gas" (func (;0;) (type 1)))
                (func (;1;) (type 0) (param i32) (result i32)
                  i32.const 3
                  call 0
                  local.get 0
                  i32.const 5
                  i32.mul)
                (func (;2;) (type 0) (param i32) (result i32)
                  i32.const 2
                  call 0
                  local.get 0
                  call 1))
            "#,
        )
        .unwrap();

        assert_eq!(transformed, expected);
    }
}
