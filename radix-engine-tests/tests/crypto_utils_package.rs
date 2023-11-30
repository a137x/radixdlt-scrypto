use radix_engine::blueprints::crypto_utils::{
    CryptoUtilsBlsVerifyInput, CryptoUtilsNativePackage, CRYPTO_UTILS_BLUEPRINT,
};
use radix_engine::types::*;
use radix_engine::vm::NoExtension;
use radix_engine_interface::blueprints::package::CRYPTO_UTILS_CODE_ID;
use radix_engine_stores::memory_db::InMemorySubstateDatabase;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::prelude::*;

#[cfg(test)]
fn crypto_utils_bls_verify(
    runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>,
    msg: &str,
    pk: &str,
    sig: &str,
) -> bool {
    let msg_hash = hash(msg);
    let pub_key = BlsPublicKey::from_str(pk).unwrap();
    let signature = BlsSignature::from_str(sig).unwrap();

    let receipt = runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee(runner.faucet_component(), 500u32)
            .call_function(
                CRYPTO_UTILS_PACKAGE,
                CRYPTO_UTILS_BLUEPRINT,
                "bls_verify",
                CryptoUtilsBlsVerifyInput {
                    msg_hash,
                    pub_key,
                    signature,
                },
            )
            .build(),
        vec![],
    );
    let result = receipt.expect_commit_success();
    result.output(1)
}

#[test]
fn test_crypto_package_bls_verify() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    test_runner.publish_native_package_at_address(
        CRYPTO_UTILS_CODE_ID,
        CryptoUtilsNativePackage::definition(),
        CRYPTO_UTILS_PACKAGE,
    );

    let msg1 = "Test";
    let msg2 = "ExpectFailureTest";
    let pk = "93b1aa7542a5423e21d8e84b4472c31664412cc604a666e9fdf03baf3c758e728c7a11576ebb01110ac39a0df95636e2";
    let msg1_signature = "a2ba96a1fc1e698b7688e077f171fbd7fe99c6bbf240b1421a08e3faa5d6b55523a18b8c77fba5830181dfec716edc3d18a8657bcadd0a83e3cafdad33998d10417f767c536b26b98df41d67ab416c761ad55438f23132a136fc82eb7b290571";

    // Act
    let msg1_verify = crypto_utils_bls_verify(&mut test_runner, msg1, pk, msg1_signature);
    let msg2_verify = crypto_utils_bls_verify(&mut test_runner, msg2, pk, msg1_signature);

    // Assert
    assert!(msg1_verify);
    assert!(!msg2_verify);
}
