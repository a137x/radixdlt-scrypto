use radix_engine::types::*;
use resource_tests::resource::{
    FungibleResourceFuzzGetBucketAction, ResourceFuzzTransformBucketAction,
    ResourceFuzzUseBucketAction,
};
use resource_tests::{FuzzAction, FuzzTest, TestFuzzer, TxnFuzzer};

#[test]
fn fuzz_fungible_resource() {
    struct FungibleResourceFuzzer;
    impl TxnFuzzer for FungibleResourceFuzzer {
        fn next_txn_intent(fuzzer: &mut TestFuzzer) -> Vec<FuzzAction> {
            let action1: FungibleResourceFuzzGetBucketAction =
                FungibleResourceFuzzGetBucketAction::from_repr(fuzzer.next_u8(4u8)).unwrap();

            let action2: ResourceFuzzUseBucketAction =
                ResourceFuzzUseBucketAction::from_repr(fuzzer.next_u8(2u8)).unwrap();

            vec![
                FuzzAction::FungibleGetBucket(action1),
                FuzzAction::FungibleBucketTransform(ResourceFuzzTransformBucketAction::Combine),
                FuzzAction::FungibleUseBucket(action2),
            ]
        }
    }

    FuzzTest::<FungibleResourceFuzzer>::run_fuzz(16, 1000);
}