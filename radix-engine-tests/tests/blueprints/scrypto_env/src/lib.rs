use scrypto::api::*;
use scrypto::prelude::*;

#[blueprint]
mod scrypto_env_test {
    struct ScryptoEnvTest {}

    impl ScryptoEnvTest {
        pub fn create_node_with_invalid_blueprint() {
            ScryptoEnv
                .new_simple_object(
                    "invalid_blueprint",
                    vec![FieldValue::new(&ScryptoEnvTest {})],
                )
                .unwrap();
        }

        pub fn create_and_open_mut_substate_twice(heap: bool) {
            let obj = Self {}.instantiate();
            if heap {
                obj.open_mut_substate_twice();
                obj.prepare_to_globalize(OwnerRole::None).globalize();
            } else {
                let globalized = obj.prepare_to_globalize(OwnerRole::None).globalize();
                globalized.open_mut_substate_twice();
            }
        }

        pub fn open_mut_substate_twice(&mut self) {
            ScryptoEnv
                .actor_open_field(OBJECT_HANDLE_SELF, 0u8, LockFlags::MUTABLE)
                .unwrap();

            ScryptoEnv
                .actor_open_field(OBJECT_HANDLE_SELF, 0u8, LockFlags::MUTABLE)
                .unwrap();
        }

        pub fn bech32_encode_address(address: ComponentAddress) -> String {
            Runtime::bech32_encode_address(address)
        }
    }
}
