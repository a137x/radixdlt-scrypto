use radix_engine::blueprints::epoch_manager::{Validator, ValidatorError};
use radix_engine::errors::{ApplicationError, ModuleError, RuntimeError};
use radix_engine::system::bootstrap::*;
use radix_engine::system::system_modules::auth::AuthError;
use radix_engine::types::*;
use radix_engine_interface::api::node_modules::auth::AuthAddresses;
use radix_engine_interface::blueprints::epoch_manager::*;
use radix_engine_interface::blueprints::resource::FromPublicKey;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_chacha;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use transaction::model::{Instruction, SystemTransaction, TransactionManifest};

#[test]
fn genesis_epoch_has_correct_initial_validators() {
    // Arrange
    let initial_epoch = 1u64;
    let max_validators = 10u32;

    let mut stake_allocations = Vec::new();
    let mut validators = Vec::new();
    let mut accounts = Vec::new();
    for k in 1usize..=100usize {
        let pub_key = EcdsaSecp256k1PrivateKey::from_u64(k.try_into().unwrap())
            .unwrap()
            .public_key();
        let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
        accounts.push(validator_account_address);
        validators.push(GenesisValidator {
            key: pub_key,
            accept_delegated_stake: true,
            is_registered: true,
            metadata: vec![],
            owner: validator_account_address,
        });

        let stake = Decimal::from(1000000 * ((k + 1) / 2));

        stake_allocations.push((
            pub_key,
            vec![GenesisStakeAllocation {
                account_index: (k - 1) as u32,
                xrd_amount: stake,
            }],
        ));
    }

    let genesis_data_chunks = vec![
        GenesisDataChunk::Validators(validators),
        GenesisDataChunk::Stakes {
            accounts,
            allocations: stake_allocations,
        },
    ];

    let genesis = CustomGenesis {
        genesis_data_chunks,
        initial_epoch,
        initial_configuration: dummy_epoch_manager_configuration()
            .with_max_validators(max_validators),
    };

    // Act
    let (_, validators) = TestRunner::builder()
        .with_custom_genesis(genesis)
        .build_and_get_epoch();

    // Assert
    assert_eq!(validators.len(), max_validators as usize);
    for (_, validator) in validators {
        assert!(
            validator.stake >= Decimal::from(45000000u64)
                && validator.stake <= Decimal::from(50000000u64)
        )
    }
}

#[test]
fn get_epoch_should_succeed() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/epoch_manager");

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "EpochManagerTest",
            "get_epoch",
            manifest_args![],
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    let epoch: u64 = receipt.expect_commit(true).output(1);
    assert_eq!(epoch, 1);
}

#[test]
fn next_round_without_supervisor_auth_fails() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/epoch_manager");

    // Act
    let round = 9876u64;
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .call_function(
            package_address,
            "EpochManagerTest",
            "next_round",
            manifest_args!(EPOCH_MANAGER, round),
        )
        .call_function(
            package_address,
            "EpochManagerTest",
            "get_epoch",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError { .. }))
    });
}

#[test]
fn next_round_with_validator_auth_succeeds() {
    // Arrange
    let initial_epoch = 1u64;
    let rounds_per_epoch = 5u64;
    let genesis = CustomGenesis::default(
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch - 1)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    assert!(result.next_epoch().is_none());
}

#[test]
fn next_epoch_with_validator_auth_succeeds() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let genesis = CustomGenesis::default(
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch").1;
    assert_eq!(next_epoch, initial_epoch + 1);
}

#[test]
fn register_validator_with_auth_succeeds() {
    // Arrange
    let initial_epoch = 5u64;
    let pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let validator_address = test_runner.get_validator_with_key(&pub_key);
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE)
        .lock_fee(test_runner.faucet_component(), 10.into())
        .register_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&pub_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn register_validator_without_auth_fails() {
    // Arrange
    let initial_epoch = 5u64;
    let pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let validator_address = test_runner.get_validator_with_key(&pub_key);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .register_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    });
}

#[test]
fn unregister_validator_with_auth_succeeds() {
    // Arrange
    let initial_epoch = 5u64;
    let pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let validator_address = test_runner.get_validator_with_key(&pub_key);
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE)
        .lock_fee(test_runner.faucet_component(), 10.into())
        .unregister_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&pub_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn unregister_validator_without_auth_fails() {
    // Arrange
    let initial_epoch = 5u64;
    let pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();

    // Act
    let validator_address = test_runner.get_validator_with_key(&pub_key);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .unregister_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError(..)))
    });
}

fn test_disabled_delegated_stake(owner: bool, expect_success: bool) {
    // Arrange
    let initial_epoch = 5u64;
    let pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&pub_key);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE)
        .call_method(
            validator_address,
            "update_accept_delegated_stake",
            manifest_args!(false),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&pub_key)],
    );
    receipt.expect_commit_success();

    // Act
    let mut builder = ManifestBuilder::new();
    builder.lock_fee(test_runner.faucet_component(), 10.into());

    if owner {
        builder.create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE);
    }

    let manifest = builder
        .call_method(test_runner.faucet_component(), "free", manifest_args!())
        .take_all_from_worktop(RADIX_TOKEN, |builder, bucket| {
            builder.call_method(validator_address, "stake", manifest_args!(bucket))
        })
        .call_method(
            validator_account_address,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&pub_key)],
    );

    // Assert
    if expect_success {
        receipt.expect_commit_success();
    } else {
        receipt.expect_specific_failure(|e| {
            matches!(
                e,
                RuntimeError::ModuleError(ModuleError::AuthError(AuthError::Unauthorized { .. }))
            )
        });
    }
}

#[test]
fn not_allowing_delegated_stake_should_still_let_owner_stake() {
    test_disabled_delegated_stake(true, true);
}

#[test]
fn not_allowing_delegated_stake_should_not_let_non_owner_stake() {
    test_disabled_delegated_stake(false, false);
}

#[test]
fn registered_validator_with_no_stake_does_not_become_part_of_validator_set_on_epoch_change() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let genesis = CustomGenesis::default(
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let (pub_key, _, account_address) = test_runner.new_account(false);
    let validator_address = test_runner.new_validator_with_pub_key(pub_key, account_address);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
        .register_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&pub_key)],
    );
    receipt.expect_commit_success();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.1, initial_epoch + 1);
    assert!(!next_epoch.0.contains_key(&validator_address));
}

#[test]
fn validator_set_receives_emissions_proportional_to_stake_on_epoch_change() {
    // Arrange
    let epoch_emissions_xrd = dec!("0.1");
    let a_stake = dec!("2.5");
    let b_stake = dec!("7.5");
    let both_stake = a_stake + b_stake;

    let a_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
    let b_key = EcdsaSecp256k1PrivateKey::from_u64(2).unwrap().public_key();
    let validators = vec![GenesisValidator::from(a_key), GenesisValidator::from(b_key)];
    let allocations = vec![
        (
            a_key,
            vec![GenesisStakeAllocation {
                account_index: 0,
                xrd_amount: a_stake,
            }],
        ),
        (
            b_key,
            vec![GenesisStakeAllocation {
                account_index: 1,
                xrd_amount: b_stake,
            }],
        ),
    ];
    let accounts = validators
        .iter()
        .map(|validator| validator.owner)
        .collect::<Vec<_>>();
    let genesis_data_chunks = vec![
        GenesisDataChunk::Validators(validators),
        GenesisDataChunk::Stakes {
            accounts,
            allocations,
        },
    ];
    let genesis = CustomGenesis {
        genesis_data_chunks,
        initial_epoch: 4,
        initial_configuration: dummy_epoch_manager_configuration()
            .with_rounds_per_epoch(1)
            .with_total_emission_xrd_per_epoch(epoch_emissions_xrd),
    };

    // Act
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&EpochManagerNextRoundInput::successful(1, 0)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let a_substate = test_runner.get_validator_info_by_key(&a_key);
    let a_new_stake = test_runner
        .inspect_vault_balance(a_substate.stake_xrd_vault_id.0)
        .unwrap();
    assert_eq!(
        a_new_stake,
        a_stake + epoch_emissions_xrd * a_stake / both_stake
    );

    let b_substate = test_runner.get_validator_info_by_key(&b_key);
    let b_new_stake = test_runner
        .inspect_vault_balance(b_substate.stake_xrd_vault_id.0)
        .unwrap();
    assert_eq!(
        b_new_stake,
        b_stake + epoch_emissions_xrd * b_stake / both_stake
    );

    // TODO(emissions): we should also be able to verify the same information being returned in the
    // `result.next_epoch()`'s validator set - however, a current bug in "list sorted after write"
    // makes this information incorrect and has to be fixed first.
    let result = receipt.expect_commit_success();
    let next_epoch_validators = result
        .next_epoch()
        .expect("Should have next epoch")
        .0
        .into_values()
        .collect::<Vec<_>>();
    assert_eq!(
        next_epoch_validators,
        vec![
            Validator {
                key: a_key,
                stake: a_stake // exposing a bug: should be `a_new_stake`
            },
            Validator {
                key: b_key,
                stake: b_stake // exposing a bug: should be `b_new_stake`
            },
        ]
    );
}

#[test]
fn validator_receives_emission_penalty_when_some_proposals_missed() {
    // Arrange
    let epoch_emissions_xrd = dec!("10");
    let rounds_per_epoch = 4; // we will simulate 3 gap rounds + 1 successfully made proposal...
    let min_required_reliability = dec!("0.2"); // ...which barely meets the threshold
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
    let validator_stake = dec!("500.0");
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        validator_stake,
        ComponentAddress::virtual_account_from_public_key(&validator_pub_key),
        4,
        dummy_epoch_manager_configuration()
            .with_rounds_per_epoch(rounds_per_epoch)
            .with_total_emission_xrd_per_epoch(epoch_emissions_xrd)
            .with_min_validator_reliability(min_required_reliability),
    );

    // Act
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let validator_substate = test_runner.get_validator_info_by_key(&validator_pub_key);
    let validator_new_stake = test_runner
        .inspect_vault_balance(validator_substate.stake_xrd_vault_id.0)
        .unwrap();
    let actual_reliability = Decimal::one() / Decimal::from(rounds_per_epoch);
    let tolerated_range = Decimal::one() - min_required_reliability;
    let reliability_factor = (actual_reliability - min_required_reliability) / tolerated_range;
    assert_eq!(
        validator_new_stake,
        validator_stake + epoch_emissions_xrd * reliability_factor
    );

    // TODO(emissions): we should also be able to verify the same information being returned in the
    // `result.next_epoch()`'s validator set - however, a current bug in "list sorted after write"
    // makes this information incorrect and has to be fixed first.
    let result = receipt.expect_commit_success();
    let next_epoch_validators = result
        .next_epoch()
        .expect("Should have next epoch")
        .0
        .into_values()
        .collect::<Vec<_>>();
    assert_eq!(
        next_epoch_validators,
        vec![Validator {
            key: validator_pub_key,
            stake: validator_stake // exposing a bug: should be `validator_new_stake`
        },]
    );
}

#[test]
fn validator_receives_no_emission_when_too_many_proposals_missed() {
    // Arrange
    let epoch_emissions_xrd = dec!("10");
    let rounds_per_epoch = 4; // we will simulate 3 gap rounds + 1 successfully made proposal...
    let min_required_reliability = dec!("0.3"); // ...which does NOT meet the threshold
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(1).unwrap().public_key();
    let validator_stake = dec!("500.0");
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        validator_stake,
        ComponentAddress::virtual_account_from_public_key(&validator_pub_key),
        4,
        dummy_epoch_manager_configuration()
            .with_rounds_per_epoch(rounds_per_epoch)
            .with_total_emission_xrd_per_epoch(epoch_emissions_xrd)
            .with_min_validator_reliability(min_required_reliability),
    );

    // Act
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let validator_substate = test_runner.get_validator_info_by_key(&validator_pub_key);
    let validator_new_stake = test_runner
        .inspect_vault_balance(validator_substate.stake_xrd_vault_id.0)
        .unwrap();
    assert_eq!(validator_new_stake, validator_stake);

    let result = receipt.expect_commit_success();
    let next_epoch_validators = result
        .next_epoch()
        .expect("Should have next epoch")
        .0
        .into_values()
        .collect::<Vec<_>>();
    assert_eq!(
        next_epoch_validators,
        vec![Validator {
            key: validator_pub_key,
            stake: validator_stake
        },]
    );
}

fn create_custom_genesis(
    initial_epoch: u64,
    rounds_per_epoch: u64,
    num_initial_validators: usize,
    max_validators: usize,
    initial_stakes: Decimal,
    accounts_xrd_balance: Decimal,
    num_accounts: usize,
) -> (
    CustomGenesis,
    Vec<(EcdsaSecp256k1PublicKey, ComponentAddress)>,
) {
    let mut stake_allocations = Vec::new();
    let mut validators = Vec::new();
    let mut accounts = Vec::new();
    for k in 1usize..=num_initial_validators {
        let pub_key = EcdsaSecp256k1PrivateKey::from_u64(k.try_into().unwrap())
            .unwrap()
            .public_key();
        let validator_account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);

        accounts.push(validator_account_address);
        validators.push(GenesisValidator {
            key: pub_key,
            accept_delegated_stake: true,
            is_registered: true,
            metadata: vec![],
            owner: validator_account_address,
        });

        stake_allocations.push((
            pub_key,
            vec![GenesisStakeAllocation {
                account_index: (k - 1) as u32,
                xrd_amount: initial_stakes,
            }],
        ));
    }

    let validator_account_index = num_initial_validators;

    let mut xrd_balances = Vec::new();
    let mut pub_key_accounts = Vec::new();

    for i in 0..num_accounts {
        let pub_key = EcdsaSecp256k1PrivateKey::from_u64(
            (validator_account_index + 1 + i).try_into().unwrap(),
        )
        .unwrap()
        .public_key();
        let account_address = ComponentAddress::virtual_account_from_public_key(&pub_key);
        pub_key_accounts.push((pub_key, account_address));
        xrd_balances.push((account_address, accounts_xrd_balance));
    }

    let genesis_data_chunks = vec![
        GenesisDataChunk::Validators(validators),
        GenesisDataChunk::Stakes {
            accounts,
            allocations: stake_allocations,
        },
        GenesisDataChunk::XrdBalances(xrd_balances),
    ];

    let genesis = CustomGenesis {
        genesis_data_chunks,
        initial_epoch,
        initial_configuration: dummy_epoch_manager_configuration()
            .with_max_validators(max_validators as u32)
            .with_rounds_per_epoch(rounds_per_epoch),
    };

    (genesis, pub_key_accounts)
}

#[derive(Clone, Copy)]
enum RegisterAndStakeTransactionType {
    SingleManifestRegisterFirst,
    SingleManifestStakeFirst,
    RegisterFirst,
    StakeFirst,
}

impl RegisterAndStakeTransactionType {
    const ALL_TYPES: [RegisterAndStakeTransactionType; 4] = [
        RegisterAndStakeTransactionType::SingleManifestStakeFirst,
        RegisterAndStakeTransactionType::SingleManifestRegisterFirst,
        RegisterAndStakeTransactionType::RegisterFirst,
        RegisterAndStakeTransactionType::StakeFirst,
    ];

    fn manifests(
        &self,
        stake_amount: Decimal,
        account_address: ComponentAddress,
        validator_address: ComponentAddress,
        faucet: ComponentAddress,
    ) -> Vec<TransactionManifest> {
        match self {
            RegisterAndStakeTransactionType::SingleManifestRegisterFirst => {
                let manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .withdraw_from_account(account_address, RADIX_TOKEN, stake_amount)
                    .register_validator(validator_address)
                    .take_all_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.stake_validator(validator_address, bucket_id)
                    })
                    .call_method(
                        account_address,
                        "deposit_batch",
                        manifest_args!(ManifestExpression::EntireWorktop),
                    )
                    .build();
                vec![manifest]
            }
            RegisterAndStakeTransactionType::SingleManifestStakeFirst => {
                let manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .withdraw_from_account(account_address, RADIX_TOKEN, stake_amount)
                    .take_all_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.stake_validator(validator_address, bucket_id)
                    })
                    .register_validator(validator_address)
                    .call_method(
                        account_address,
                        "deposit_batch",
                        manifest_args!(ManifestExpression::EntireWorktop),
                    )
                    .build();
                vec![manifest]
            }
            RegisterAndStakeTransactionType::RegisterFirst => {
                let register_manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .register_validator(validator_address)
                    .build();

                let stake_manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .withdraw_from_account(account_address, RADIX_TOKEN, stake_amount)
                    .take_all_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.stake_validator(validator_address, bucket_id)
                    })
                    .call_method(
                        account_address,
                        "deposit_batch",
                        manifest_args!(ManifestExpression::EntireWorktop),
                    )
                    .build();

                vec![register_manifest, stake_manifest]
            }
            RegisterAndStakeTransactionType::StakeFirst => {
                let register_manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .register_validator(validator_address)
                    .build();

                let stake_manifest = ManifestBuilder::new()
                    .lock_fee(faucet, 10.into())
                    .create_proof_from_account(account_address, VALIDATOR_OWNER_BADGE)
                    .withdraw_from_account(account_address, RADIX_TOKEN, stake_amount)
                    .take_all_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                        builder.stake_validator(validator_address, bucket_id)
                    })
                    .call_method(
                        account_address,
                        "deposit_batch",
                        manifest_args!(ManifestExpression::EntireWorktop),
                    )
                    .build();

                vec![stake_manifest, register_manifest]
            }
        }
    }
}

fn register_and_stake_new_validator(
    register_and_stake_txn_type: RegisterAndStakeTransactionType,
    pub_key: EcdsaSecp256k1PublicKey,
    account_address: ComponentAddress,
    stake_amount: Decimal,
    test_runner: &mut TestRunner,
) -> ComponentAddress {
    let validator_address = test_runner.new_validator_with_pub_key(pub_key, account_address);

    let manifests = register_and_stake_txn_type.manifests(
        stake_amount,
        account_address,
        validator_address,
        test_runner.faucet_component(),
    );

    for manifest in manifests {
        let receipt = test_runner.execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&pub_key)],
        );
        receipt.expect_commit_success();
    }

    validator_address
}

fn registered_validator_test(
    register_and_stake_txn_type: RegisterAndStakeTransactionType,
    num_initial_validators: usize,
    max_validators: usize,
    initial_stakes: Decimal,
    validator_to_stake_amount: Decimal,
    expect_in_next_epoch: bool,
    expected_num_validators_in_next_epoch: usize,
) {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let (genesis, accounts) = create_custom_genesis(
        initial_epoch,
        rounds_per_epoch,
        num_initial_validators,
        max_validators,
        initial_stakes,
        validator_to_stake_amount,
        1,
    );
    let (pub_key, account_address) = accounts[0];
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = register_and_stake_new_validator(
        register_and_stake_txn_type,
        pub_key,
        account_address,
        validator_to_stake_amount,
        &mut test_runner,
    );

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.0.len(), expected_num_validators_in_next_epoch);
    assert_eq!(next_epoch.1, initial_epoch + 1);
    assert_eq!(
        next_epoch.0.contains_key(&validator_address),
        expect_in_next_epoch
    );
}

#[test]
fn registered_validator_with_stake_does_not_become_part_of_validator_on_epoch_change_if_stake_not_enough(
) {
    for register_and_stake_type in RegisterAndStakeTransactionType::ALL_TYPES {
        registered_validator_test(
            register_and_stake_type,
            10,
            10,
            1000000.into(),
            900000.into(),
            false,
            10,
        );
    }
}

#[test]
fn registered_validator_with_stake_does_become_part_of_validator_on_epoch_change_if_there_are_empty_spots(
) {
    for register_and_stake_type in RegisterAndStakeTransactionType::ALL_TYPES {
        registered_validator_test(
            register_and_stake_type,
            9,
            10,
            1000000.into(),
            900000.into(),
            true,
            10,
        );
    }
}

#[test]
fn registered_validator_with_enough_stake_does_become_part_of_validator_on_epoch_change() {
    for register_and_stake_type in RegisterAndStakeTransactionType::ALL_TYPES {
        registered_validator_test(
            register_and_stake_type,
            10,
            10,
            1000000.into(),
            1100000.into(),
            true,
            10,
        );
    }
}

#[test]
fn low_stakes_should_cause_no_problems() {
    for register_and_stake_type in RegisterAndStakeTransactionType::ALL_TYPES {
        registered_validator_test(register_and_stake_type, 1, 10, 1.into(), 1.into(), true, 2);
    }
}

#[test]
fn test_registering_and_staking_many_validators() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let (genesis, accounts) = create_custom_genesis(
        initial_epoch,
        rounds_per_epoch,
        1,
        10,
        1.into(),
        1.into(),
        10,
    );
    let mut rng = ChaCha8Rng::seed_from_u64(1234);

    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let mut all_manifests = Vec::new();
    for (pub_key, account_address) in accounts {
        let validator_address = test_runner.new_validator_with_pub_key(pub_key, account_address);

        let rand = rng.gen_range(0..RegisterAndStakeTransactionType::ALL_TYPES.len());
        let register_and_stake_type = RegisterAndStakeTransactionType::ALL_TYPES[rand];

        let manifests = register_and_stake_type.manifests(
            1.into(),
            account_address,
            validator_address,
            test_runner.faucet_component(),
        );
        all_manifests.push((pub_key, manifests));
    }

    all_manifests.shuffle(&mut rng);

    for (pub_key, manifests) in all_manifests {
        for manifest in manifests {
            let receipt = test_runner.execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(&pub_key)],
            );
            receipt.expect_commit_success();
        }
    }

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.0.len(), 10);
    assert_eq!(next_epoch.1, initial_epoch + 1);
}

#[test]
fn unregistered_validator_gets_removed_on_epoch_change() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(2u64)
        .unwrap()
        .public_key();
    let validator_account_address =
        ComponentAddress::virtual_account_from_public_key(&validator_pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&validator_pub_key);
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE)
        .lock_fee(test_runner.faucet_component(), 10.into())
        .unregister_validator(validator_address)
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&validator_pub_key)],
    );
    receipt.expect_commit_success();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.1, initial_epoch + 1);
    assert!(!next_epoch.0.contains_key(&validator_address));
}

#[test]
fn updated_validator_keys_gets_updated_on_epoch_change() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(2u64)
        .unwrap()
        .public_key();
    let validator_account_address =
        ComponentAddress::virtual_account_from_public_key(&validator_pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        Decimal::one(),
        validator_account_address,
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&validator_pub_key);
    let next_validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(3u64)
        .unwrap()
        .public_key();
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .create_proof_from_account(validator_account_address, VALIDATOR_OWNER_BADGE)
        .call_method(
            validator_address,
            "update_key",
            manifest_args!(next_validator_pub_key),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&validator_pub_key)],
    );
    receipt.expect_commit_success();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.1, initial_epoch + 1);
    assert_eq!(
        next_epoch.0.get(&validator_address).unwrap().key,
        next_validator_pub_key
    );
}

#[test]
fn cannot_claim_unstake_immediately() {
    // Arrange
    let initial_epoch = 5u64;
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(2u64)
        .unwrap()
        .public_key();
    let account_pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let account_with_lp = ComponentAddress::virtual_account_from_public_key(&account_pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        Decimal::from(10),
        account_with_lp,
        initial_epoch,
        dummy_epoch_manager_configuration(),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&validator_pub_key);
    let validator_substate = test_runner.get_validator_info(validator_address);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .withdraw_from_account(
            account_with_lp,
            validator_substate.liquidity_token,
            1.into(),
        )
        .take_all_from_worktop(validator_substate.liquidity_token, |builder, bucket| {
            builder.unstake_validator(validator_address, bucket)
        })
        .take_all_from_worktop(validator_substate.unstake_nft, |builder, bucket| {
            builder.claim_xrd(validator_address, bucket)
        })
        .call_method(
            account_with_lp,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&account_pub_key)],
    );

    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::ValidatorError(
                ValidatorError::EpochUnlockHasNotOccurredYet
            ))
        )
    });
}

#[test]
fn can_claim_unstake_after_epochs() {
    // Arrange
    let initial_epoch = 5u64;
    let num_unstake_epochs = 7u64;
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(2u64)
        .unwrap()
        .public_key();
    let account_pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let account_with_lp = ComponentAddress::virtual_account_from_public_key(&account_pub_key);
    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        Decimal::from(10),
        account_with_lp,
        initial_epoch,
        dummy_epoch_manager_configuration().with_num_unstake_epochs(num_unstake_epochs),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&validator_pub_key);
    let validator_substate = test_runner.get_validator_info(validator_address);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .withdraw_from_account(
            account_with_lp,
            validator_substate.liquidity_token,
            1.into(),
        )
        .take_all_from_worktop(validator_substate.liquidity_token, |builder, bucket| {
            builder.unstake_validator(validator_address, bucket)
        })
        .call_method(
            account_with_lp,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&account_pub_key)],
    );
    receipt.expect_commit_success();
    test_runner.set_current_epoch(initial_epoch + 1 + num_unstake_epochs);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .withdraw_from_account(account_with_lp, validator_substate.unstake_nft, 1.into())
        .take_all_from_worktop(validator_substate.unstake_nft, |builder, bucket| {
            builder.claim_xrd(validator_address, bucket)
        })
        .call_method(
            account_with_lp,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&account_pub_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn unstaked_validator_gets_less_stake_on_epoch_change() {
    // Arrange
    let initial_epoch = 5u64;
    let rounds_per_epoch = 2u64;
    let validator_pub_key = EcdsaSecp256k1PrivateKey::from_u64(2u64)
        .unwrap()
        .public_key();
    let account_pub_key = EcdsaSecp256k1PrivateKey::from_u64(1u64)
        .unwrap()
        .public_key();
    let account_with_lp = ComponentAddress::virtual_account_from_public_key(&account_pub_key);

    let genesis = CustomGenesis::single_validator_and_staker(
        validator_pub_key,
        Decimal::from(10),
        account_with_lp,
        initial_epoch,
        dummy_epoch_manager_configuration().with_rounds_per_epoch(rounds_per_epoch),
    );
    let mut test_runner = TestRunner::builder().with_custom_genesis(genesis).build();
    let validator_address = test_runner.get_validator_with_key(&validator_pub_key);
    let validator_substate = test_runner.get_validator_info(validator_address);
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10.into())
        .withdraw_from_account(
            account_with_lp,
            validator_substate.liquidity_token,
            Decimal::one(),
        )
        .take_all_from_worktop(validator_substate.liquidity_token, |builder, bucket| {
            builder.unstake_validator(validator_address, bucket)
        })
        .call_method(
            account_with_lp,
            "deposit_batch",
            manifest_args!(ManifestExpression::EntireWorktop),
        )
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&account_pub_key)],
    );
    receipt.expect_commit_success();

    // Act
    let instructions = vec![Instruction::CallMethod {
        component_address: EPOCH_MANAGER,
        method_name: EPOCH_MANAGER_NEXT_ROUND_IDENT.to_string(),
        args: to_manifest_value(&next_round_after_gap(rounds_per_epoch)),
    }];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs: vec![],
            nonce: 0,
            pre_allocated_ids: BTreeSet::new(),
        }
        .get_executable(btreeset![AuthAddresses::validator_role()]),
    );

    // Assert
    let result = receipt.expect_commit(true);
    let next_epoch = result.next_epoch().expect("Should have next epoch");
    assert_eq!(next_epoch.1, initial_epoch + 1);
    assert_eq!(
        next_epoch.0.get(&validator_address).unwrap(),
        &Validator {
            key: validator_pub_key,
            stake: Decimal::from(9),
        }
    );
}

#[test]
fn epoch_manager_create_should_fail_with_supervisor_privilege() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let mut pre_allocated_ids = BTreeSet::new();
    pre_allocated_ids.insert(EPOCH_MANAGER.into());
    pre_allocated_ids.insert(VALIDATOR_OWNER_BADGE.into());
    let instructions = vec![Instruction::CallFunction {
        package_address: EPOCH_MANAGER_PACKAGE,
        blueprint_name: EPOCH_MANAGER_BLUEPRINT.to_string(),
        function_name: EPOCH_MANAGER_CREATE_IDENT.to_string(),
        args: manifest_args!(
            Into::<[u8; NodeId::LENGTH]>::into(VALIDATOR_OWNER_BADGE),
            Into::<[u8; NodeId::LENGTH]>::into(EPOCH_MANAGER),
            1u64,
            dummy_epoch_manager_configuration()
        ),
    }];
    let blobs = vec![];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs,
            nonce: 0,
            pre_allocated_ids,
        }
        .get_executable(btreeset![]),
    );

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(e, RuntimeError::ModuleError(ModuleError::AuthError { .. }))
    });
}

#[test]
fn epoch_manager_create_should_succeed_with_system_privilege() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let mut pre_allocated_ids = BTreeSet::new();
    pre_allocated_ids.insert(EPOCH_MANAGER.into());
    pre_allocated_ids.insert(VALIDATOR_OWNER_BADGE.into());

    let instructions = vec![Instruction::CallFunction {
        package_address: EPOCH_MANAGER_PACKAGE,
        blueprint_name: EPOCH_MANAGER_BLUEPRINT.to_string(),
        function_name: EPOCH_MANAGER_CREATE_IDENT.to_string(),
        args: manifest_args!(
            Into::<[u8; NodeId::LENGTH]>::into(VALIDATOR_OWNER_BADGE),
            Into::<[u8; NodeId::LENGTH]>::into(EPOCH_MANAGER),
            1u64,
            dummy_epoch_manager_configuration()
        ),
    }];
    let blobs = vec![];
    let receipt = test_runner.execute_transaction(
        SystemTransaction {
            instructions,
            blobs,
            nonce: 0,
            pre_allocated_ids,
        }
        .get_executable(btreeset![AuthAddresses::system_role()]),
    );

    // Assert
    receipt.expect_commit_success();
}

fn next_round_after_gap(current_round: u64) -> EpochManagerNextRoundInput {
    EpochManagerNextRoundInput {
        round: current_round,
        leader_proposal_history: LeaderProposalHistory {
            gap_round_leaders: (1..current_round).map(|_| 0).collect(),
            current_leader: 0,
            is_fallback: false,
        },
    }
}

fn dummy_epoch_manager_configuration() -> EpochManagerInitialConfiguration {
    EpochManagerInitialConfiguration {
        max_validators: 10,
        rounds_per_epoch: 5,
        num_unstake_epochs: 1,
        total_emission_xrd_per_epoch: Decimal::one(),
        min_validator_reliability: Decimal::one(),
    }
}
