use sbor::*;
use scrypto::api::key_value_entry_api::ClientKeyValueEntryApi;
use scrypto::api::key_value_store_api::ClientKeyValueStoreApi;
use scrypto::api::*;
use scrypto::prelude::wasm_api::kv_entry_set;
use scrypto::prelude::*;

#[derive(Sbor, ScryptoEvent)]
struct TestEvent {
    message: String,
}

#[blueprint]
#[events(TestEvent)]
mod transaction_limits {
    struct TransactionLimitTest {
        kv_store: KeyValueStore<u32, u32>,
    }

    impl TransactionLimitTest {
        pub fn new() -> Global<TransactionLimitTest> {
            TransactionLimitTest {
                kv_store: KeyValueStore::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn read_non_existent_entries_from_kv_store(&self, n: u32) {
            for i in 0..n {
                self.kv_store.get(&i);
            }
        }

        pub fn write_entries_to_kv_store(&self, n: u32) {
            for i in 0..n {
                self.kv_store.insert(i, i);
            }
        }

        pub fn write_entries_to_heap_kv_store(n: u32) {
            let kv_store = KeyValueStore::new();
            for i in 0..n {
                kv_store.insert(i, i);
            }
            TransactionLimitTest { kv_store }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        pub fn recursive_with_memory(n: u32, m: usize) {
            if n > 1 {
                let _v: Vec<u8> = Vec::with_capacity(m);
                Blueprint::<TransactionLimitTest>::recursive_with_memory(n - 1, m);
            }
        }

        pub fn emit_event_of_size(n: usize) {
            let name = "TestEvent";
            let buf = scrypto_encode(&TestEvent {
                message: "a".repeat(n),
            })
            .unwrap();
            unsafe { wasm_api::emit_event(name.as_ptr(), name.len(), buf.as_ptr(), buf.len(), 0) }
        }

        pub fn emit_log_of_size(n: usize) {
            let level = scrypto_encode(&Level::Debug).unwrap();
            let buf = "a".repeat(n);
            unsafe { wasm_api::emit_log(level.as_ptr(), level.len(), buf.as_ptr(), buf.len()) }
        }

        pub fn panic_of_size(n: usize) {
            let buf = "a".repeat(n);
            unsafe { wasm_api::panic(buf.as_ptr(), buf.len()) }
        }
    }
}

#[blueprint]
mod transaction_limits_substate {
    struct TransactionLimitSubstateTest {
        kv_store: KeyValueStore<u32, Vec<u8>>,
    }

    impl TransactionLimitSubstateTest {
        pub fn write_large_value(raw_array_size: usize) -> Global<TransactionLimitSubstateTest> {
            // SBOR encoding of Vec<u8>
            let mut buf = Vec::new();
            let mut encoder = VecEncoder::<ScryptoCustomValueKind>::new(&mut buf, 100);
            encoder
                .write_payload_prefix(SCRYPTO_SBOR_V1_PAYLOAD_PREFIX)
                .unwrap();
            encoder.write_value_kind(ValueKind::Array).unwrap();
            encoder.write_value_kind(ValueKind::U8).unwrap();
            encoder.write_size(raw_array_size).unwrap();
            buf.reserve(raw_array_size);
            let new_len = buf.len() + raw_array_size;
            unsafe { buf.set_len(new_len) };

            // Create a KVStore
            let kv_store = KeyValueStore::<u32, Vec<u8>>::new();

            // Insert into store
            let key_payload = scrypto_encode(&1u32).unwrap();
            let handle = ScryptoEnv
                .key_value_store_open_entry(
                    kv_store.id.as_node_id(),
                    &key_payload,
                    LockFlags::MUTABLE,
                )
                .unwrap();
            unsafe { kv_entry_set(handle, buf.as_ptr(), buf.len()) };
            ScryptoEnv.key_value_entry_close(handle).unwrap();

            // Put the kv store into a component
            TransactionLimitSubstateTest { kv_store }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
        }
    }
}

#[blueprint]
mod invoke_limits {
    struct InvokeLimitsTest {}

    impl InvokeLimitsTest {
        pub fn call(raw_array_size: usize) {
            // SBOR encoding of (Vec<u8>)
            let mut buf = Vec::new();
            let mut encoder = VecEncoder::<ScryptoCustomValueKind>::new(&mut buf, 100);
            encoder
                .write_payload_prefix(SCRYPTO_SBOR_V1_PAYLOAD_PREFIX)
                .unwrap();
            encoder.write_value_kind(ValueKind::Tuple).unwrap();
            encoder.write_size(1).unwrap();
            encoder.write_value_kind(ValueKind::Array).unwrap();
            encoder.write_value_kind(ValueKind::U8).unwrap();
            encoder.write_size(raw_array_size).unwrap();
            buf.reserve(raw_array_size);
            let new_len = buf.len() + raw_array_size;
            unsafe { buf.set_len(new_len) };

            ScryptoEnv
                .call_function(
                    Runtime::package_address(),
                    "InvokeLimitsTest",
                    "callee",
                    buf,
                )
                .unwrap();
        }

        pub fn callee(_: Vec<u8>) {}
    }
}
