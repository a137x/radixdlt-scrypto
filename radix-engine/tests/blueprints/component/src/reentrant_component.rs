use radix_engine_interface::api::types::*;
use radix_engine_interface::wasm::*;
use scrypto::engine::scrypto_env::*;
use scrypto::prelude::*;

blueprint! {
    struct ReentrantComponent {}

    impl ReentrantComponent {
        pub fn new() -> ComponentAddress {
            Self {}.instantiate().globalize()
        }

        pub fn mut_func(&mut self) {}

        pub fn call_mut_self(&mut self) {
            if let ScryptoActor::Component(component_id, ..) = Runtime::actor() {
                let input = RadixEngineInput::Invoke(SerializedInvocation::Scrypto(
                    ScryptoInvocation::Method(
                        ScryptoMethodIdent {
                            receiver: ScryptoReceiver::Component(component_id),
                            method_name: "mut_func".to_string(),
                        },
                        args!(),
                    ),
                ));
                let _: Vec<u8> = call_engine(input);
            }
        }

        pub fn func(&self) {}

        pub fn call_self(&self) {
            if let ScryptoActor::Component(component_id, ..) = Runtime::actor() {
                let input = RadixEngineInput::Invoke(SerializedInvocation::Scrypto(
                    ScryptoInvocation::Method(
                        ScryptoMethodIdent {
                            receiver: ScryptoReceiver::Component(component_id),
                            method_name: "func".to_string(),
                        },
                        args!(),
                    ),
                ));
                let _: Vec<u8> = call_engine(input);
            }
        }

        pub fn call_mut_self_2(&self) {
            if let ScryptoActor::Component(component_id, ..) = Runtime::actor() {
                let input = RadixEngineInput::Invoke(SerializedInvocation::Scrypto(
                    ScryptoInvocation::Method(
                        ScryptoMethodIdent {
                            receiver: ScryptoReceiver::Component(component_id),
                            method_name: "mut_func".to_string(),
                        },
                        args!(),
                    ),
                ));
                let _: Vec<u8> = call_engine(input);
            }
        }
    }
}
