use radix_engine::errors::{RuntimeError, SystemError, SystemModuleError};
use radix_engine::system::system_modules::auth::AuthError;
use radix_engine::transaction::TransactionReceipt;
use radix_engine::types::*;
use radix_engine_interface::api::node_modules::auth::AuthAddresses;
use radix_engine_interface::api::ModuleId;
use radix_engine_interface::blueprints::resource::FromPublicKey;
use radix_engine_interface::blueprints::transaction_processor::InstructionOutput;
use radix_engine_interface::rule;
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn can_call_public_function() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");

    // Act
    let receipt =
        test_runner.call_function(package_address, "FunctionRules", "public_function", ());

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn cannot_call_protected_function_without_auth() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");

    // Act
    let receipt =
        test_runner.call_function(package_address, "FunctionRules", "protected_function", ());

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(AuthError::Unauthorized(
                ..
            )))
        )
    });
}

#[test]
fn can_call_protected_function_with_auth() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");
    let (key, _priv, account) = test_runner.new_account(true);

    // Act
    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, dec!(1))
        .call_function(
            package_address,
            "FunctionRules",
            "protected_function",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner
        .execute_manifest_ignoring_fee(manifest, [NonFungibleGlobalId::from_public_key(&key)]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn roles_assignment_method_auth_cannot_be_mutated_when_locked() {
    // Arrange
    let mut roles = RoleAssignmentInit::new();
    roles.define_role("deposit_funds_auth_update", rule!(allow_all));
    roles.define_role("borrow_funds_auth", rule!(allow_all));
    roles.define_role("deposit_funds_auth", rule!(require(XRD)));
    let mut test_runner = MutableRolesTestRunner::new(roles);

    // Act
    let receipt = test_runner.set_role_rule(RoleKey::new("deposit_funds_auth"), rule!(allow_all));

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(..))
        )
    });
}

#[test]
fn role_assignment_method_auth_cant_be_mutated_when_required_proofs_are_not_present() {
    // Arrange
    let private_key = Secp256k1PrivateKey::from_u64(709).unwrap();
    let public_key = private_key.public_key();
    let virtual_badge_non_fungible_global_id = NonFungibleGlobalId::from_public_key(&public_key);
    let mut test_runner = MutableRolesTestRunner::new_with_owner(rule!(require(
        virtual_badge_non_fungible_global_id.clone()
    )));

    // Act
    let receipt = test_runner.set_role_rule(RoleKey::new("borrow_funds_auth"), rule!(allow_all));

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(..))
        )
    });
}

#[test]
fn role_assignment_method_auth_can_be_mutated_when_required_proofs_are_present() {
    // Arrange
    let private_key = Secp256k1PrivateKey::from_u64(709).unwrap();
    let public_key = private_key.public_key();
    let virtual_badge_non_fungible_global_id = NonFungibleGlobalId::from_public_key(&public_key);
    let mut test_runner = MutableRolesTestRunner::new_with_owner(rule!(require(
        virtual_badge_non_fungible_global_id.clone()
    )));

    // Act
    test_runner.add_initial_proof(virtual_badge_non_fungible_global_id);
    let receipt = test_runner.set_role_rule(RoleKey::new("borrow_funds_auth"), rule!(allow_all));

    // Assert
    receipt.expect_commit_success();
}

fn component_role_assignment_can_be_mutated_through_manifest(to_rule: Rule) {
    // Arrange
    let private_key = Secp256k1PrivateKey::from_u64(709).unwrap();
    let public_key = private_key.public_key();
    let virtual_badge_non_fungible_global_id = NonFungibleGlobalId::from_public_key(&public_key);
    let mut test_runner = MutableRolesTestRunner::new_with_owner(rule!(require(
        virtual_badge_non_fungible_global_id.clone()
    )));
    test_runner.add_initial_proof(virtual_badge_non_fungible_global_id.clone());

    // Act
    let receipt = test_runner.execute_manifest(
        MutableRolesTestRunner::manifest_builder()
            .set_main_role(test_runner.component_address, "borrow_funds_auth", to_rule)
            .build(),
    );

    // Assert
    receipt.expect_commit_success();
    let receipt = test_runner.borrow_funds();
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(..))
        )
    });
}

#[test]
fn component_role_assignment_can_be_mutated_to_deny_all_through_manifest() {
    component_role_assignment_can_be_mutated_through_manifest(rule!(deny_all));
}

#[test]
fn component_role_assignment_can_be_mutated_to_fungible_resource_through_manifest() {
    component_role_assignment_can_be_mutated_through_manifest(rule!(require(XRD)));
}

#[test]
fn component_role_assignment_can_be_mutated_to_non_fungible_resource_through_manifest() {
    let non_fungible_global_id = AuthAddresses::system_role();
    component_role_assignment_can_be_mutated_through_manifest(rule!(require(
        non_fungible_global_id
    )));
}

#[test]
fn assert_access_rule_through_component_when_not_fulfilled_fails() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");
    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "AssertRule", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(manifest, []);
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    // Act
    let manifest = ManifestBuilder::new()
        .call_method(
            component_address,
            "assert_access_rule",
            manifest_args!(rule!(require(XRD))),
        )
        .build();

    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, []);

    // Assert
    receipt.expect_specific_failure(|error: &RuntimeError| {
        matches!(
            error,
            RuntimeError::SystemError(SystemError::AssertRuleFailed)
        )
    })
}

#[test]
fn assert_access_rule_through_component_when_fulfilled_succeeds() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    let (public_key, _, account) = test_runner.new_account(false);
    let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");

    let component_address = {
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "AssertRule", "new", manifest_args!())
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            [NonFungibleGlobalId::from_public_key(&public_key)],
        );
        receipt.expect_commit_success();

        receipt.expect_commit(true).new_component_addresses()[0]
    };

    let manifest = ManifestBuilder::new()
        .create_proof_from_account_of_amount(account, XRD, dec!(1))
        .call_method(
            component_address,
            "assert_access_rule",
            manifest_args!(rule!(require(XRD))),
        )
        .build();

    // Act
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        [NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn update_rule() {
    // Arrange
    let private_key = Secp256k1PrivateKey::from_u64(709).unwrap();
    let public_key = private_key.public_key();
    let virtual_badge_non_fungible_global_id = NonFungibleGlobalId::from_public_key(&public_key);
    let mut test_runner = MutableRolesTestRunner::new_with_owner(rule!(require(
        virtual_badge_non_fungible_global_id.clone()
    )));

    let receipt = test_runner.get_role(RoleKey::new("borrow_funds_auth"));
    let ret = receipt.expect_commit(true).outcome.expect_success();
    assert_eq!(
        ret[1],
        InstructionOutput::CallReturn(
            scrypto_encode(&Some(Rule::Protected(RuleNode::ProofRule(
                ProofRule::Require(ResourceOrNonFungible::Resource(XRD))
            ))))
            .unwrap()
        )
    );

    // Act, update rule
    test_runner.add_initial_proof(virtual_badge_non_fungible_global_id);
    let receipt = test_runner.set_role_rule(RoleKey::new("borrow_funds_auth"), rule!(allow_all));
    receipt.expect_commit_success();

    // Act
    let receipt = test_runner.get_role(RoleKey::new("borrow_funds_auth"));

    // Assert
    let ret = receipt.expect_commit(true).outcome.expect_success();
    assert_eq!(
        ret[1],
        InstructionOutput::CallReturn(scrypto_encode(&Some(Rule::AllowAll)).unwrap())
    );
}

#[test]
fn change_lock_owner_role_rules() {
    // Arrange
    let mut test_runner =
        MutableRolesTestRunner::new_with_owner_role(OwnerRole::Updatable(rule!(allow_all)));

    // Act: verify if lock owner role is possible
    let receipt = test_runner.lock_owner_role();
    receipt.expect_commit(true).outcome.expect_success();
    let receipt = test_runner.lock_owner_role();

    // Assert
    receipt.expect_specific_failure(|error: &RuntimeError| {
        matches!(
            error,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(..)),
        )
    })
}

struct MutableRolesTestRunner {
    test_runner: DefaultTestRunner,
    component_address: ComponentAddress,
    initial_proofs: BTreeSet<NonFungibleGlobalId>,
}

impl MutableRolesTestRunner {
    const BLUEPRINT_NAME: &'static str = "MutableRulesComponent";

    pub fn create_component(
        roles: RoleAssignmentInit,
        test_runner: &mut DefaultTestRunner,
    ) -> TransactionReceipt {
        let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");

        let manifest = ManifestBuilder::new()
            .call_function(
                package_address,
                Self::BLUEPRINT_NAME,
                "new",
                manifest_args!(roles),
            )
            .build();
        test_runner.execute_manifest_ignoring_fee(manifest, vec![])
    }

    pub fn create_component_with_owner(
        owner_role: OwnerRole,
        test_runner: &mut DefaultTestRunner,
    ) -> TransactionReceipt {
        let package_address = test_runner.compile_and_publish("./tests/blueprints/role_assignment");

        let manifest = ManifestBuilder::new()
            .call_function(
                package_address,
                Self::BLUEPRINT_NAME,
                "new_with_owner",
                manifest_args!(owner_role),
            )
            .build();
        test_runner.execute_manifest_ignoring_fee(manifest, vec![])
    }

    pub fn new_with_owner(update_access_rule: Rule) -> Self {
        let mut test_runner = TestRunnerBuilder::new().build();
        let receipt = Self::create_component_with_owner(
            OwnerRole::Fixed(update_access_rule),
            &mut test_runner,
        );
        let component_address = receipt.expect_commit(true).new_component_addresses()[0];

        Self {
            test_runner,
            component_address,
            initial_proofs: BTreeSet::new(),
        }
    }

    pub fn new_with_owner_role(owner_role: OwnerRole) -> Self {
        let mut test_runner = TestRunnerBuilder::new().build();
        let receipt = Self::create_component_with_owner(owner_role, &mut test_runner);
        let component_address = receipt.expect_commit(true).new_component_addresses()[0];

        Self {
            test_runner,
            component_address,
            initial_proofs: BTreeSet::new(),
        }
    }

    pub fn new(roles: RoleAssignmentInit) -> Self {
        let mut test_runner = TestRunnerBuilder::new().build();
        let receipt = Self::create_component(roles, &mut test_runner);
        let component_address = receipt.expect_commit(true).new_component_addresses()[0];

        Self {
            test_runner,
            component_address,
            initial_proofs: BTreeSet::new(),
        }
    }

    pub fn add_initial_proof(&mut self, initial_proof: NonFungibleGlobalId) {
        self.initial_proofs.insert(initial_proof);
    }

    pub fn set_role_rule(&mut self, role_key: RoleKey, access_rule: Rule) -> TransactionReceipt {
        let manifest = Self::manifest_builder()
            .set_role(
                self.component_address,
                ModuleId::Main,
                role_key,
                access_rule,
            )
            .build();
        self.execute_manifest(manifest)
    }

    pub fn get_role(&mut self, role_key: RoleKey) -> TransactionReceipt {
        let manifest = Self::manifest_builder()
            .get_role(self.component_address, ModuleId::Main, role_key)
            .build();
        self.execute_manifest(manifest)
    }

    pub fn lock_owner_role(&mut self) -> TransactionReceipt {
        let manifest = Self::manifest_builder()
            .lock_owner_role(self.component_address)
            .build();
        self.execute_manifest(manifest)
    }

    pub fn borrow_funds(&mut self) -> TransactionReceipt {
        let manifest = Self::manifest_builder()
            .call_method(self.component_address, "borrow_funds", manifest_args!())
            .build();
        self.execute_manifest(manifest)
    }

    pub fn manifest_builder() -> ManifestBuilder {
        ManifestBuilder::new()
    }

    pub fn execute_manifest(&mut self, manifest: TransactionManifestV1) -> TransactionReceipt {
        self.test_runner
            .execute_manifest_ignoring_fee(manifest, self.initial_proofs.clone())
    }
}
