use radix_engine::{
    errors::{KernelError, ModuleError, RuntimeError},
    system::kernel_modules::transaction_limits::TransactionLimitsError,
    types::*,
};
use radix_engine_constants::{
    DEFAULT_MAX_CALL_DEPTH, DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME,
    DEFAULT_MAX_WASM_MEM_PER_TRANSACTION,
};
use radix_engine_interface::blueprints::resource::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

const WASM_MEMORY_PAGE_SIZE: usize = 64 * 1024;

#[test]
fn test_loop() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let code = wat2wasm(&include_str!("wasm/loop.wat").replace("${n}", "1000"));
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest_with_cost_unit_limit(manifest, vec![], 15_000_000);

    // Assert
    receipt.expect_commit_success();
}

// TODO: investigate the case where cost_unit_limit < system_loan and transaction runs out of cost units.

#[test]
fn test_loop_out_of_cost_unit() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let code = wat2wasm(&include_str!("wasm/loop.wat").replace("${n}", "2000000"));
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 450.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest_with_cost_unit_limit(manifest, vec![], 15_000_000);

    // Assert
    receipt.expect_specific_failure(is_costing_error)
}

#[test]
fn test_recursion() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    // In this test case, each call frame costs 4 stack units
    let code = wat2wasm(&include_str!("wasm/recursion.wat").replace("${n}", "256"));
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn test_recursion_stack_overflow() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let code = wat2wasm(&include_str!("wasm/recursion.wat").replace("${n}", "257"));
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(is_wasm_error)
}

#[test]
fn test_grow_memory() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    let max_mem_pages: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME / WASM_MEMORY_PAGE_SIZE;

    // Calculate how much we can grow the memory (by wasm pages), subtract 1 to be below limit.
    let grow_value: usize = max_mem_pages - 1;

    // Act
    let code = wat2wasm(
        &include_str!("wasm/memory.wat")
            .replace("${n}", &grow_value.to_string())
            .replace("${m}", &max_mem_pages.to_string()),
    );
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn test_grow_memory_out_of_cost_unit() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    let max_mem_pages: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME / WASM_MEMORY_PAGE_SIZE;

    // Act
    let code = wat2wasm(
        &include_str!("wasm/memory.wat")
            .replace("${n}", "100000")
            .replace("${m}", &max_mem_pages.to_string()),
    );
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(is_costing_error)
}

#[test]
fn test_max_call_frame_memory_exceeded() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    let max_mem_pages: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME / WASM_MEMORY_PAGE_SIZE;
    // Grow memory (wasm pages) to exceed default max wasm memory per instance.
    let grow_value: usize = max_mem_pages;

    // Act
    let code = wat2wasm(
        &include_str!("wasm/memory.wat")
            .replace("${n}", &grow_value.to_string())
            .replace("${m}", &max_mem_pages.to_string()),
    );
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::KernelError(KernelError::WasmRuntimeError(_))
        )
    })
}

#[test]
fn test_max_transaction_memory_exceeded() {
    // Test recursive instantiation of WASM with memory allocation of 1 MiB
    // for each call (+additional memory for transaction execution).

    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("tests/blueprints/recursion");

    // Calculate value of additional bytes to allocate per call to exceed
    // max wasm memory per transaction limit.
    let grow_value: usize = DEFAULT_MAX_WASM_MEM_PER_TRANSACTION / DEFAULT_MAX_CALL_DEPTH;

    // Ensure calculated wasm instance grow value is lower than instance limit.
    assert!(grow_value <= DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME);

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "Caller",
            "recursive_with_memory",
            args!(DEFAULT_MAX_CALL_DEPTH as u32, grow_value),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ModuleError(ModuleError::TransactionLimitsError(
                TransactionLimitsError::MaxWasmMemoryExceeded(_)
            ))
        )
    })
}

#[test]
fn test_max_call_frame_memory_exceeded_blueprint() {
    // Test recursive instantiation of WASM with memory allocation of 1 MiB
    // for each call (+additional memory for transaction execution).

    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("tests/blueprints/recursion");

    // Calculate value of additional bytes to allocate per call to exceed
    // max wasm memory per transaction limit.
    let grow_value: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME;

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "Caller",
            "recursive_with_memory",
            args!(1u32, grow_value),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::KernelError(KernelError::WasmRuntimeError(_))
        )
    })
}
