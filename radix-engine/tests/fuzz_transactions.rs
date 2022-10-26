use radix_engine::constants::{
    DEFAULT_COST_UNIT_PRICE, DEFAULT_MAX_CALL_DEPTH, DEFAULT_SYSTEM_LOAN,
};
use radix_engine::engine::ScryptoInterpreter;
use radix_engine::ledger::TypedInMemorySubstateStore;
use radix_engine::state_manager::StagedSubstateStoreManager;
use radix_engine::transaction::{ExecutionConfig, FeeReserveConfig, TransactionExecutor};
use radix_engine::types::*;
use radix_engine::wasm::{
    DefaultWasmEngine, InstructionCostRules, WasmInstrumenter, WasmMeteringParams,
};
use rand::Rng;
use rand_chacha;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use transaction::builder::{ManifestBuilder, TransactionBuilder};
use transaction::model::{NotarizedTransaction, TransactionHeader};
use transaction::signing::EcdsaSecp256k1PrivateKey;
use transaction::validation::{
    NotarizedTransactionValidator, TestIntentHashManager, TransactionValidator, ValidationConfig,
};

fn execute_single_transaction(transaction: NotarizedTransaction) {
    let validator = NotarizedTransactionValidator::new(ValidationConfig::simulator());

    let transaction = validator
        .validate(transaction, &TestIntentHashManager::new())
        .unwrap();

    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut scrypto_interpreter = ScryptoInterpreter {
        wasm_engine: DefaultWasmEngine::new(),
        wasm_instrumenter: WasmInstrumenter::new(),
        wasm_metering_params: WasmMeteringParams::new(
            InstructionCostRules::tiered(1, 5, 10, 5000),
            512,
        ),
        phantom: PhantomData,
    };
    let execution_config = ExecutionConfig {
        max_call_depth: DEFAULT_MAX_CALL_DEPTH,
        trace: false,
    };
    let fee_reserve_config = FeeReserveConfig {
        cost_unit_price: DEFAULT_COST_UNIT_PRICE.parse().unwrap(),
        system_loan: DEFAULT_SYSTEM_LOAN,
    };

    let mut staged_store_manager = StagedSubstateStoreManager::new(&mut store);
    let staged_node = staged_store_manager.new_child_node(0);

    let mut staged_store = staged_store_manager.get_output_store(staged_node);
    let mut transaction_executor =
        TransactionExecutor::new(&mut staged_store, &mut scrypto_interpreter);
    transaction_executor.execute_and_commit(&transaction, &fee_reserve_config, &execution_config);
}

struct TransactionFuzzer {
    rng: ChaCha8Rng,
}

impl TransactionFuzzer {
    fn new() -> Self {
        let rng = ChaCha8Rng::seed_from_u64(1234);
        Self { rng }
    }

    fn next_transaction(&mut self) -> NotarizedTransaction {
        let mut builder = ManifestBuilder::new(&NetworkDefinition::simulator());
        let instruction_count = self.rng.gen_range(0u32..20u32);
        for _ in 0..instruction_count {
            let next = self.rng.gen_range(0u32..4u32);
            match next {
                0 => {
                    builder.take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.call_function(
                            ACCOUNT_PACKAGE,
                            "Account",
                            "new_with_resource",
                            args!(AccessRule::AllowAll, scrypto::resource::Bucket(bucket_id)),
                        )
                    });
                }
                1 => {
                    builder.call_function(
                        ACCOUNT_PACKAGE,
                        "Account",
                        "new",
                        args!(AccessRule::AllowAll),
                    );
                }
                2 => {
                    builder.take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.call_function(
                            ACCOUNT_PACKAGE,
                            "Account",
                            "new_with_resource",
                            args!(AccessRule::AllowAll, scrypto::resource::Bucket(bucket_id)),
                        )
                    });
                }
                3 => {
                    builder.call_method(SYS_FAUCET_COMPONENT, "lock_fee", args!(dec!("100")));
                }
                _ => panic!("Unexpected"),
            }
        }

        let manifest = builder.build();
        let private_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap();
        let header = TransactionHeader {
            version: 1,
            network_id: NetworkDefinition::simulator().id,
            start_epoch_inclusive: 0,
            end_epoch_exclusive: 100,
            nonce: 5,
            notary_public_key: private_key.public_key().into(),
            notary_as_signatory: false,
            cost_unit_limit: 10_000_000,
            tip_percentage: 0,
        };

        TransactionBuilder::new()
            .header(header)
            .manifest(manifest)
            .sign(&private_key)
            .notarize(&private_key)
            .build()
    }
}

#[test]
fn simple_transaction_fuzz_test() {
    let mut fuzzer = TransactionFuzzer::new();
    let transactions: Vec<NotarizedTransaction> = (0..200)
        .into_iter()
        .map(|_| fuzzer.next_transaction())
        .collect();
    transactions.into_par_iter().for_each(|transaction| {
        execute_single_transaction(transaction);
    });
}
