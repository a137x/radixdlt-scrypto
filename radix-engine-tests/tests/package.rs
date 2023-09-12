mod package_loader;

use package_loader::PackageLoader;
use radix_engine::blueprints::package::PackageError;
use radix_engine::errors::{ApplicationError, RuntimeError, SystemModuleError, VmError};
use radix_engine::system::system_modules::auth::AuthError;
use radix_engine::types::*;
use radix_engine::vm::wasm::PrepareError;
use radix_engine::vm::wasm::*;
use radix_engine_interface::blueprints::package::{
    AuthConfig, BlueprintDefinitionInit, BlueprintType, PackageDefinition,
    PackagePublishNativeManifestInput, PACKAGE_BLUEPRINT,
};
use radix_engine_interface::metadata_init;
use radix_engine_interface::schema::{
    BlueprintEventSchemaInit, BlueprintFunctionsSchemaInit, BlueprintSchemaInit,
    BlueprintStateSchemaInit, FieldSchema, FunctionSchemaInit, TypeRef,
};
use sbor::basic_well_known_types::{ANY_TYPE, UNIT_TYPE};
use scrypto_unit::*;
use transaction::prelude::*;

#[test]
fn missing_memory_should_cause_error() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Act
    let code = wat2wasm(
        r#"
            (module
                (func (export "test") (result i32)
                    i32.const 1337
                )
            )
            "#,
    );
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            PackageDefinition::default(),
            BTreeMap::new(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            &RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidWasm(PrepareError::InvalidMemory(
                    InvalidMemory::MissingMemorySection
                ))
            ))
        )
    });
}

#[test]
fn large_return_len_should_cause_memory_access_error() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(package, "LargeReturnSize", "f", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        if let RuntimeError::VmError(VmError::Wasm(b)) = e {
            matches!(*b, WasmRuntimeError::MemoryAccessError)
        } else {
            false
        }
    });
}

#[test]
fn overflow_return_len_should_cause_memory_access_error() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(package, "MaxReturnSize", "f", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        if let RuntimeError::VmError(VmError::Wasm(b)) = e {
            matches!(*b, WasmRuntimeError::MemoryAccessError)
        } else {
            false
        }
    });
}

#[test]
fn zero_return_len_should_cause_data_validation_error() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(package, "ZeroReturnSize", "f", manifest_args!())
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| matches!(e, RuntimeError::SystemUpstreamError(_)));
}

#[test]
fn test_basic_package() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Act
    let code = wat2wasm(include_str!("wasm/basic_package.wat"));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            single_function_package_definition("Test", "f"),
            BTreeMap::new(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn test_basic_package_missing_export() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let mut blueprints = index_map_new();
    blueprints.insert(
        "Test".to_string(),
        BlueprintDefinitionInit {
            blueprint_type: BlueprintType::default(),
            is_transient: false,
            feature_set: indexset!(),
            dependencies: indexset!(),

            schema: BlueprintSchemaInit {
                generics: vec![],
                schema: VersionedScryptoSchema::V1(SchemaV1 {
                    type_kinds: vec![],
                    type_metadata: vec![],
                    type_validations: vec![],
                }),
                state: BlueprintStateSchemaInit {
                    fields: vec![FieldSchema::static_field(LocalTypeId::WellKnown(UNIT_TYPE))],
                    collections: vec![],
                },
                events: BlueprintEventSchemaInit::default(),
                types: BlueprintTypeSchemaInit::default(),
                functions: BlueprintFunctionsSchemaInit {
                    functions: indexmap!(
                        "f".to_string() => FunctionSchemaInit {
                            receiver: Option::None,
                            input: TypeRef::Static(LocalTypeId::WellKnown(ANY_TYPE)),
                            output: TypeRef::Static(LocalTypeId::WellKnown(ANY_TYPE)),
                            export: "not_exist".to_string(),
                        }
                    ),
                },
                hooks: BlueprintHooksInit::default(),
            },

            royalty_config: PackageRoyaltyConfig::default(),
            auth_config: AuthConfig::default(),
        },
    );
    // Act
    let code = wat2wasm(include_str!("wasm/basic_package.wat"));
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            PackageDefinition { blueprints },
            BTreeMap::new(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidWasm(PrepareError::MissingExport { .. })
            ))
        )
    });
}

#[test]
fn bad_function_schema_should_fail() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Act
    let (code, definition) = PackageLoader::get("package_invalid");
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(None, code, definition, BTreeMap::new(), OwnerRole::None)
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidLocalTypeId(_)
            ))
        )
    });
}

#[test]
fn should_not_be_able_to_publish_wasm_package_outside_of_transaction_processor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("publish_package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package,
            "PublishPackage",
            "publish_package",
            manifest_args!(),
        )
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

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
fn should_not_be_able_to_publish_advanced_wasm_package_outside_of_transaction_processor() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("publish_package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package,
            "PublishPackage",
            "publish_package_advanced",
            manifest_args!(),
        )
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

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
fn should_not_be_able_to_publish_native_packages() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            PACKAGE_PACKAGE,
            PACKAGE_BLUEPRINT,
            "publish_native",
            PackagePublishNativeManifestInput {
                package_address: None,
                native_package_code_id: 0u64,
                definition: PackageDefinition::default(),
                metadata: metadata_init!(),
            },
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

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
fn should_not_be_able_to_publish_native_packages_in_scrypto() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let package = test_runner.publish_package_simple(PackageLoader::get("publish_package"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package,
            "PublishPackage",
            "publish_native",
            manifest_args!(),
        )
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

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
fn name_validation_blueprint() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("publish_package");

    definition.blueprints = indexmap![
       String::from("wrong_bluepint_name_*") =>
            definition
                .blueprints
                .values_mut()
                .next()
                .unwrap()
                .to_owned(),
    ];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(None, code, definition, BTreeMap::new(), OwnerRole::None)
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidName { .. }
            ))
        )
    });
}

#[test]
fn name_validation_feature_set() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("publish_package");

    definition
        .blueprints
        .values_mut()
        .next()
        .unwrap()
        .feature_set
        .insert(String::from("wrong-feature"));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(None, code, definition, BTreeMap::new(), OwnerRole::None)
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidName { .. }
            ))
        )
    });
}

#[test]
fn well_known_types_in_schema_are_validated() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (code, mut definition) = PackageLoader::get("publish_package");

    let method_definition = definition
        .blueprints
        .values_mut()
        .next()
        .unwrap()
        .schema
        .functions
        .functions
        .get_mut("some_method".into())
        .unwrap();

    // Invalid well known type
    method_definition.input = TypeRef::Static(LocalTypeId::WellKnown(WellKnownTypeId::of(0)));

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(None, code, definition, BTreeMap::new(), OwnerRole::None)
        .build();

    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidLocalTypeId(..)
            ))
        )
    });
}

#[test]
fn publishing_of_package_with_blueprint_name_exceeding_length_limit_fails() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("address");

    let (_, value) = definition.blueprints.pop().unwrap();
    definition
        .blueprints
        .insert(name(MAX_BLUEPRINT_NAME_LEN + 1, 'A'), value);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            definition,
            MetadataInit::default(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::ExceededMaxBlueprintNameLen {
                    limit: 100,
                    actual: 101
                }
            ))
        )
    })
}

#[test]
fn publishing_of_package_where_outer_blueprint_is_inner_fails() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("address");

    let (bp_name1, mut bp_definition1) = definition.blueprints.pop().unwrap();
    let (bp_name2, mut bp_definition2) = definition.blueprints.pop().unwrap();

    bp_definition1.blueprint_type = BlueprintType::Inner {
        outer_blueprint: "NoneExistent".to_owned(),
    };
    bp_definition2.blueprint_type = BlueprintType::Inner {
        outer_blueprint: bp_name1.clone(),
    };

    definition.blueprints.insert(bp_name2, bp_definition2);
    definition.blueprints.insert(bp_name1, bp_definition1);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            definition,
            MetadataInit::default(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::OuterBlueprintCantBeAnInnerBlueprint { .. }
            ))
        )
    })
}

#[test]
fn publishing_of_package_where_outer_blueprint_is_self_fails() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("address");

    let (bp_name, mut bp_definition) = definition.blueprints.pop().unwrap();
    bp_definition.blueprint_type = BlueprintType::Inner {
        outer_blueprint: bp_name.clone(),
    };
    definition.blueprints.insert(bp_name, bp_definition);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            definition,
            MetadataInit::default(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::MissingOuterBlueprint
            ))
        )
    })
}

#[test]
fn publishing_of_package_with_transient_blueprints_fails() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("address");

    definition
        .blueprints
        .values_mut()
        .for_each(|def| def.is_transient = true);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            definition,
            MetadataInit::default(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::WasmUnsupported(..)
            ))
        )
    })
}

#[test]
fn publishing_of_package_with_whitespace_in_blueprint_name_fails() {
    test_publishing_of_packages_with_invalid_names("\nHelloWorld")
}

#[test]
fn publishing_of_package_with_number_at_start_of_blueprint_name_fails() {
    test_publishing_of_packages_with_invalid_names("1000HelloWorld")
}

#[test]
fn publishing_of_package_with_a_hidden_ascii_character_fails() {
    test_publishing_of_packages_with_invalid_names("World")
}

#[test]
fn publishing_of_package_with_a_lookalike_character_fails() {
    test_publishing_of_packages_with_invalid_names("depοsit")
}

fn test_publishing_of_packages_with_invalid_names(name: &str) {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();
    let (code, mut definition) = PackageLoader::get("address");

    let (_, value) = definition.blueprints.pop().unwrap();
    definition.blueprints.insert(name.to_owned(), value);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .publish_package_advanced(
            None,
            code,
            definition,
            MetadataInit::default(),
            OwnerRole::None,
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|error| {
        matches!(
            error,
            RuntimeError::ApplicationError(ApplicationError::PackageError(
                PackageError::InvalidName { .. }
            ))
        )
    })
}

fn name(len: usize, chr: char) -> String {
    (0..len).map(|_| chr).collect()
}
