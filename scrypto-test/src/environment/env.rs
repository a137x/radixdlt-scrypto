//! This module defines and implements the [`TestEnvironment`] struct.

use super::*;
use crate::prelude::*;
use std::ops::AddAssign;

/// The environment that the tests are run against.
///
/// Each test environment has it's own instance of a [`SelfContainedRadixEngine`] which is exposed
/// through the [`ClientApi`] and which tests run against.
pub struct TestEnvironment(pub(super) SelfContainedRadixEngine);

impl TestEnvironment {
    //================
    // Initialization
    //================

    pub fn new() -> Self {
        let mut env = Self(SelfContainedRadixEngine::standard());

        // Adding references to all of the well-known global nodes.
        env.0.with_kernel_mut(|kernel| {
            let current_frame = kernel.kernel_current_frame_mut();
            for node_id in GLOBAL_VISIBLE_NODES {
                let Ok(global_address) = GlobalAddress::try_from(node_id.0) else {
                    continue;
                };
                current_frame.add_global_reference(global_address)
            }
        });

        // Publishing the test-environment package.
        let test_environment_package = {
            let code = include_bytes!("../../../assets/test_environment.wasm");
            let package_definition = manifest_decode::<PackageDefinition>(include_bytes!(
                "../../../assets/test_environment.rpd"
            ))
            .expect("Must succeed");

            env.with_auth_module_disabled(|env| {
                Package::publish_advanced(
                    OwnerRole::None,
                    package_definition,
                    code.to_vec(),
                    Default::default(),
                    None,
                    env,
                )
                .expect("Must succeed")
            })
        };

        // Creating the call-frame of the test environment & making it the current call frame
        {
            // Creating the auth zone of the next call-frame
            let auth_zone = env.0.with_kernel_mut(|kernel| {
                let mut system_service = SystemService {
                    api: kernel,
                    phantom: PhantomData,
                };
                AuthModule::create_mock(
                    &mut system_service,
                    Some((TRANSACTION_PROCESSOR_PACKAGE.as_node_id(), false)),
                    Default::default(),
                    Default::default(),
                )
                .expect("Must succeed")
            });

            // Define the actor of the next call-frame. This would be a function actor of the test
            // environment package.
            let actor = Actor::Function(FunctionActor {
                blueprint_id: BlueprintId {
                    package_address: test_environment_package,
                    blueprint_name: "TestEnvironment".to_owned(),
                },
                ident: "run".to_owned(),
                auth_zone,
            });

            // Creating the message, call-frame, and doing the replacement.
            let message = {
                let mut message =
                    CallFrameMessage::from_input(&IndexedScryptoValue::from_typed(&()), &actor);
                for node_id in GLOBAL_VISIBLE_NODES {
                    message.copy_global_references.push(node_id);
                }
                message
            };
            env.0.with_kernel_mut(|kernel| {
                let current_frame = kernel.kernel_current_frame_mut();
                let new_frame = CallFrame::new_child_from_parent(current_frame, actor, message)
                    .expect("Must succeed.");
                let previous_frame = core::mem::replace(current_frame, new_frame);
                kernel.kernel_prev_frame_stack_mut().push(previous_frame)
            });
        }

        env
    }

    //=============
    // Invocations
    //=============

    /// Invokes a function on the provided blueprint and package with the given arguments.
    ///
    /// This method is a typed version of the [`ClientBlueprintApi::call_function`] which Scrypto
    /// encodes the arguments and Scrypto decodes the returns on behalf of the caller. This method
    /// assumes that the caller is correct about the argument and return types and panics if the
    /// encoding or decoding fails.
    ///
    /// # Arguments
    ///
    /// * `package_address`: [`PackageAddress`] - The address of the package that contains the
    /// blueprint.
    /// * `blueprint_name`: [`&str`] - The name of the blueprint.
    /// * `function_name`: [`&str`] - The nae of the function.
    /// * `args`: `&I` - The arguments to invoke the method with. This is a generic arguments that
    /// is fulfilled by any type that implements [`ScryptoEncode`].
    ///
    /// # Returns
    ///
    /// * [`Result<O, RuntimeError>`] - The returns from the method invocation. If the invocation
    /// was successful a [`Result::Ok`] is returned, otherwise a [`Result::Err`] is returned. The
    /// [`Result::Ok`] variant is a generic that's fulfilled by any type that implements
    /// [`ScryptoDecode`].
    ///
    /// # Panics
    ///
    /// This method panics in the following two cases:
    ///
    /// * Through an unwrap when calling [`scrypto_encode`] on the method arguments. Please consult
    /// the SBOR documentation on more information on why SBOR encoding may fail.
    /// * Through an unwrap when calling [`scrypto_decode`] on the returns. This panics if the type
    /// could be decoded as the desired output type.
    pub fn call_function_typed<I, O>(
        &mut self,
        package_address: PackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: &I,
    ) -> Result<O, RuntimeError>
    where
        I: ScryptoEncode,
        O: ScryptoDecode,
    {
        let args = scrypto_encode(args).expect("Scrypto encoding of args failed");
        self.call_function(package_address, blueprint_name, function_name, args)
            .map(|rtn| scrypto_decode(&rtn).expect("Scrypto decoding of returns failed"))
    }

    /// Invokes a method on the main module of a node with the provided typed arguments.
    ///
    /// This method is a typed version of the [`ClientObjectApi::call_method`] which Scrypto encodes
    /// the arguments and Scrypto decodes the returns on behalf of the caller. This method assumes
    /// that the caller is correct about the argument and return types and panics if the encoding or
    /// decoding fails.
    ///
    /// # Arguments
    ///
    /// * `node_id`: `T` - The node to invoke the method on. This is a generic argument that's
    /// fulfilled by any type that implements [`Into<NodeId>`], thus, any address type can be used.
    /// * `method_name`: [`&str`] - The name of the method to invoke.
    /// * `args`: `&I` - The arguments to invoke the method with. This is a generic arguments that
    /// is fulfilled by any type that implements [`ScryptoEncode`].
    ///
    /// # Returns
    ///
    /// * [`Result<O, RuntimeError>`] - The returns from the method invocation. If the invocation
    /// was successful a [`Result::Ok`] is returned, otherwise a [`Result::Err`] is returned. The
    /// [`Result::Ok`] variant is a generic that's fulfilled by any type that implements
    /// [`ScryptoDecode`].
    ///
    /// # Panics
    ///
    /// This method panics in the following two cases:
    ///
    /// * Through an unwrap when calling [`scrypto_encode`] on the method arguments. Please consult
    /// the SBOR documentation on more information on why SBOR encoding may fail.
    /// * Through an unwrap when calling [`scrypto_decode`] on the returns. This panics if the type
    /// could be decoded as the desired output type.
    pub fn call_method_typed<N, I, O>(
        &mut self,
        node_id: N,
        method_name: &str,
        args: &I,
    ) -> Result<O, RuntimeError>
    where
        N: Into<NodeId>,
        I: ScryptoEncode,
        O: ScryptoDecode,
    {
        let args = scrypto_encode(args).expect("Scrypto encoding of args failed");
        self.call_method(&node_id.into(), method_name, args)
            .map(|rtn| scrypto_decode(&rtn).expect("Scrypto decoding of returns failed"))
    }

    /// Invokes a method on the main module of a node with the provided typed arguments.
    ///
    /// This method is a typed version of the [`ClientObjectApi::call_method`] which Scrypto encodes
    /// the arguments and Scrypto decodes the returns on behalf of the caller. This method assumes
    /// that the caller is correct about the argument and return types and panics if the encoding or
    /// decoding fails.
    ///
    /// # Arguments
    ///
    /// * `node_id`: `T` - The node to invoke the method on. This is a generic argument that's
    /// fulfilled by any type that implements [`Into<NodeId>`], thus, any address type can be used.
    /// * `method_name`: [`&str`] - The name of the method to invoke.
    /// * `args`: `&I` - The arguments to invoke the method with. This is a generic arguments that
    /// is fulfilled by any type that implements [`ScryptoEncode`].
    ///
    /// # Returns
    ///
    /// * [`Result<O, RuntimeError>`] - The returns from the method invocation. If the invocation
    /// was successful a [`Result::Ok`] is returned, otherwise a [`Result::Err`] is returned. The
    /// [`Result::Ok`] variant is a generic that's fulfilled by any type that implements
    /// [`ScryptoDecode`].
    ///
    /// # Panics
    ///
    /// This method panics in the following two cases:
    ///
    /// * Through an unwrap when calling [`scrypto_encode`] on the method arguments. Please consult
    /// the SBOR documentation on more information on why SBOR encoding may fail.
    /// * Through an unwrap when calling [`scrypto_decode`] on the returns. This panics if the type
    /// could be decoded as the desired output type.
    pub fn call_direct_access_method_typed<N, I, O>(
        &mut self,
        node_id: N,
        method_name: &str,
        args: &I,
    ) -> Result<O, RuntimeError>
    where
        N: Into<NodeId>,
        I: ScryptoEncode,
        O: ScryptoDecode,
    {
        let args = scrypto_encode(args).expect("Scrypto encoding of args failed");
        self.call_direct_access_method(&node_id.into(), method_name, args)
            .map(|rtn| scrypto_decode(&rtn).expect("Scrypto decoding of returns failed"))
    }

    /// Invokes a method on a module of a node with the provided typed arguments.
    ///
    /// This method is a typed version of the [`ClientObjectApi::call_method`] which Scrypto encodes
    /// the arguments and Scrypto decodes the returns on behalf of the caller. This method assumes
    /// that the caller is correct about the argument and return types and panics if the encoding or
    /// decoding fails.
    ///
    /// # Arguments
    ///
    /// * `node_id`: `T` - The node to invoke the method on. This is a generic argument that's
    /// fulfilled by any type that implements [`Into<NodeId>`], thus, any address type can be used.
    /// * `module`: [`ModuleId`] - The module id.
    /// * `method_name`: [`&str`] - The name of the method to invoke.
    /// * `args`: `&I` - The arguments to invoke the method with. This is a generic arguments that
    /// is fulfilled by any type that implements [`ScryptoEncode`].
    ///
    /// # Returns
    ///
    /// * [`Result<O, RuntimeError>`] - The returns from the method invocation. If the invocation
    /// was successful a [`Result::Ok`] is returned, otherwise a [`Result::Err`] is returned. The
    /// [`Result::Ok`] variant is a generic that's fulfilled by any type that implements
    /// [`ScryptoDecode`].
    ///
    /// # Panics
    ///
    /// This method panics in the following two cases:
    ///
    /// * Through an unwrap when calling [`scrypto_encode`] on the method arguments. Please consult
    /// the SBOR documentation on more information on why SBOR encoding may fail.
    /// * Through an unwrap when calling [`scrypto_decode`] on the returns. This panics if the type
    /// could be decoded as the desired output type.
    pub fn call_module_method_typed<N, I, O>(
        &mut self,
        node_id: N,
        module: ModuleId,
        method_name: &str,
        args: &I,
    ) -> Result<O, RuntimeError>
    where
        N: Into<NodeId>,
        I: ScryptoEncode,
        O: ScryptoDecode,
    {
        let args = scrypto_encode(args).expect("Scrypto encoding of args failed");
        self.call_module_method(&node_id.into(), module, method_name, args)
            .map(|rtn| scrypto_decode(&rtn).expect("Scrypto decoding of returns failed"))
    }

    //====================================
    // Manipulation of the Kernel Modules
    //====================================

    /// Enables the kernel trace kernel module of the Radix Engine.
    pub fn enable_kernel_trace_module(&mut self) {
        self.enable_module(EnabledModules::KERNEL_TRACE)
    }

    /// Enables the limits kernel module of the Radix Engine.
    pub fn enable_limits_module(&mut self) {
        self.enable_module(EnabledModules::LIMITS)
    }

    /// Enables the costing kernel module of the Radix Engine.
    pub fn enable_costing_module(&mut self) {
        self.enable_module(EnabledModules::COSTING)
    }

    /// Enables the auth kernel module of the Radix Engine.
    pub fn enable_auth_module(&mut self) {
        self.enable_module(EnabledModules::AUTH)
    }

    /// Enables the transaction env kernel module of the Radix Engine.
    pub fn enable_transaction_runtime_module(&mut self) {
        self.enable_module(EnabledModules::TRANSACTION_RUNTIME)
    }

    /// Enables the execution trace kernel module of the Radix Engine.
    pub fn enable_execution_trace_module(&mut self) {
        self.enable_module(EnabledModules::EXECUTION_TRACE)
    }

    /// Disables the kernel trace kernel module of the Radix Engine.
    pub fn disable_kernel_trace_module(&mut self) {
        self.disable_module(EnabledModules::KERNEL_TRACE)
    }

    /// Disables the limits kernel module of the Radix Engine.
    pub fn disable_limits_module(&mut self) {
        self.disable_module(EnabledModules::LIMITS)
    }

    /// Disables the costing kernel module of the Radix Engine.
    pub fn disable_costing_module(&mut self) {
        self.disable_module(EnabledModules::COSTING)
    }

    /// Disables the auth kernel module of the Radix Engine.
    pub fn disable_auth_module(&mut self) {
        self.disable_module(EnabledModules::AUTH)
    }

    /// Disables the transaction env kernel module of the Radix Engine.
    pub fn disable_transaction_runtime_module(&mut self) {
        self.disable_module(EnabledModules::TRANSACTION_RUNTIME)
    }

    /// Disables the execution trace kernel module of the Radix Engine.
    pub fn disable_execution_trace_module(&mut self) {
        self.disable_module(EnabledModules::EXECUTION_TRACE)
    }

    /// Calls the passed `callback` with the kernel trace kernel module enabled and then resets the
    /// state of the kernel modules.
    pub fn with_kernel_trace_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_kernel_trace_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the limits kernel module enabled and then resets the state
    /// of the kernel modules.
    pub fn with_limits_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_limits_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the costing kernel module enabled and then resets the state
    /// of the kernel modules.
    pub fn with_costing_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_costing_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the auth kernel module enabled and then resets the state of
    /// the kernel modules.
    pub fn with_auth_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_auth_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the transaction env kernel module enabled and then
    /// resets the state of the kernel modules.
    pub fn with_transaction_runtime_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_transaction_runtime_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the execution trace kernel module enabled and then resets
    /// the state of the kernel modules.
    pub fn with_execution_trace_module_enabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.enable_execution_trace_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the kernel trace kernel module disabled and then resets the
    /// state of the kernel modules.
    pub fn with_kernel_trace_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_kernel_trace_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the limits kernel module disabled and then resets the state
    /// of the kernel modules.
    pub fn with_limits_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_limits_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the costing kernel module disabled and then resets the
    /// state of the kernel modules.
    pub fn with_costing_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_costing_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the auth kernel module disabled and then resets the state
    /// of the kernel modules.
    pub fn with_auth_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_auth_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the transaction env kernel module disabled and then
    /// resets the state of the kernel modules.
    pub fn with_transaction_runtime_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_transaction_runtime_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Calls the passed `callback` with the execution trace kernel module disabled and then resets
    /// the state of the kernel modules.
    pub fn with_execution_trace_module_disabled<F, O>(&mut self, callback: F) -> O
    where
        F: FnOnce(&mut Self) -> O,
    {
        let enabled_modules = self.enabled_modules();
        self.disable_execution_trace_module();
        let rtn = callback(self);
        self.set_enabled_modules(enabled_modules);
        rtn
    }

    /// Returns the bit flags representing the currently enabled kernel modules.
    pub fn enabled_modules(&self) -> EnabledModules {
        self.0
            .with_kernel(|kernel| kernel.kernel_callback().modules.enabled_modules)
    }

    /// Sets the bit flags representing the enabled kernel modules.
    pub fn set_enabled_modules(&mut self, enabled_modules: EnabledModules) {
        self.0.with_kernel_mut(|kernel| {
            kernel.kernel_callback_mut().modules.enabled_modules = enabled_modules
        })
    }

    /// Enables specific kernel module(s).
    pub fn enable_module(&mut self, module: EnabledModules) {
        self.0.with_kernel_mut(|kernel| {
            kernel.kernel_callback_mut().modules.enabled_modules |= module
        })
    }

    /// Disables specific kernel module(s).
    pub fn disable_module(&mut self, module: EnabledModules) {
        self.0.with_kernel_mut(|kernel| {
            kernel.kernel_callback_mut().modules.enabled_modules &= !module
        })
    }

    //=======
    // State
    //=======

    /// Reads the state of a component and SBOR decodes it to the specified generic.
    ///
    /// This method reads the state of a component and returns it as an instance of [`S`]. Owned
    /// nodes encountered in the component state are added as transient references to the test call
    /// frame and references are added as references to the test call frame. This means that all
    /// nodes in the component state become visible to the tests.
    ///
    /// # Arguments
    ///
    /// * `node_id`: [`N`] - The address of the component to read the state of. This is a generic
    /// type parameter that's satisfied by any type that implements [`Into<NodeId>`].
    ///
    /// # Returns
    ///
    /// * [`Result<S, RuntimeError>`] - If the component state could be read successfully then an
    /// [`Ok`] is returned, otherwise an [`Err`] is returned.
    ///
    /// # Panics
    ///
    /// This method panics if the component state can not be decoded as the generic type parameter
    /// [`S`].
    pub fn read_component_state<S, N>(&mut self, node_id: N) -> Result<S, RuntimeError>
    where
        S: ScryptoDecode,
        N: Into<NodeId>,
    {
        self.0.with_kernel_mut(|kernel| {
            let handle = kernel.kernel_open_substate(
                &node_id.into(),
                MAIN_BASE_PARTITION,
                &SubstateKey::Field(ComponentField::State0.into()),
                LockFlags::read_only(),
                SystemLockData::Field(FieldLockData::Read),
            )?;
            let state = kernel.kernel_read_substate(handle).map(|v| {
                let FieldSubstate::<ScryptoValue>::V1(FieldSubstateV1 { payload, .. }) =
                    v.as_typed().unwrap();
                scrypto_encode(&payload).unwrap()
            })?;
            kernel.kernel_close_substate(handle)?;

            // Add all of the references in the state to the current call-frame
            let indexed_state = IndexedScryptoValue::from_slice(&state).unwrap();
            for reference in indexed_state.references() {
                kernel
                    .kernel_current_frame_mut()
                    .add_global_reference(GlobalAddress::new_or_panic(reference.0))
            }

            // Add all of the owned nodes from the state as transient nodes to the test call frame
            for own in indexed_state.owned_nodes() {
                let device = Self::get_node_device(kernel, own);
                kernel
                    .kernel_current_frame_mut()
                    .transient_references_mut()
                    .entry(*own)
                    .or_insert(TransientReference {
                        ref_count: 0,
                        ref_origin: ReferenceOrigin::SubstateNonGlobalReference(device),
                    })
                    .ref_count
                    .add_assign(1);
            }

            // Decode and return
            Ok(scrypto_decode::<S>(&state).unwrap())
        })
    }

    //===================
    // Epoch & Timestamp
    //===================

    /// Gets the current epoch from the Consensus Manager.
    pub fn get_current_epoch(&mut self) -> Epoch {
        Runtime::current_epoch(self).unwrap()
    }

    /// Sets the current epoch.
    pub fn set_current_epoch(&mut self, epoch: Epoch) {
        self.as_method_actor(
            CONSENSUS_MANAGER,
            ObjectModuleId::Main,
            CONSENSUS_MANAGER_NEXT_ROUND_IDENT,
            |env: &mut TestEnvironment| -> Result<(), RuntimeError> {
                let manager_handle = env
                    .actor_open_field(
                        ACTOR_STATE_SELF,
                        ConsensusManagerField::State.into(),
                        LockFlags::MUTABLE,
                    )
                    .unwrap();
                let mut manager_substate =
                    env.field_read_typed::<VersionedConsensusManagerState>(manager_handle)?;
                match &mut manager_substate {
                    VersionedConsensusManagerState::V1(ConsensusManagerSubstate {
                        epoch: state_epoch,
                        ..
                    }) => *state_epoch = epoch,
                }
                env.field_write_typed(manager_handle, &manager_substate)?;
                env.field_close(manager_handle)?;
                Ok(())
            },
        )
        .unwrap()
        .unwrap();
    }

    /// Gets the current time stamp from the Consensus Manager.
    pub fn get_current_time(&mut self) -> Instant {
        Runtime::current_time(self, TimePrecision::Minute).unwrap()
    }

    pub fn set_current_time(&mut self, instant: Instant) {
        self.as_method_actor(
            CONSENSUS_MANAGER,
            ObjectModuleId::Main,
            CONSENSUS_MANAGER_NEXT_ROUND_IDENT,
            |env: &mut TestEnvironment| -> Result<(), RuntimeError> {
                let handle = env.actor_open_field(
                    ACTOR_STATE_SELF,
                    ConsensusManagerField::ProposerMinuteTimestamp.into(),
                    LockFlags::MUTABLE,
                )?;
                let mut proposer_minute_timestamp =
                    env.field_read_typed::<ConsensusManagerProposerMinuteTimestampFieldPayload>(
                        handle,
                    )?;
                match &mut proposer_minute_timestamp {
                    ConsensusManagerProposerMinuteTimestampFieldPayload {
                        content: VersionedConsensusManagerProposerMinuteTimestamp::V1(timestamp),
                    } => timestamp.epoch_minute = (instant.seconds_since_unix_epoch / 60) as i32,
                };
                env.field_write_typed(handle, &proposer_minute_timestamp)?;
                env.field_close(handle)?;
                Ok(())
            },
        )
        .unwrap()
        .unwrap();
    }

    //=========
    // Helpers
    //=========

    // TODO: Feels hacky, something better is needed.
    /// Determines the device that the node is in based on whether it's in the heap or not.
    fn get_node_device(kernel: &TestKernel<'_>, node_id: &NodeId) -> SubstateDevice {
        let substate_io = kernel.kernel_substate_io();
        if substate_io
            .heap
            .scan_keys(node_id, MAIN_BASE_PARTITION, 1)
            .len()
            == 1
        {
            SubstateDevice::Heap
        } else {
            SubstateDevice::Store
        }
    }

    /// Allows us to perform some action as another actor.
    ///
    /// This function pushes a new call-frame onto the stack with the actor information we desire,
    /// performs the call-back, and then pops the call-frame off.
    pub(crate) fn as_method_actor<N, F, O>(
        &mut self,
        node_id: N,
        module_id: ObjectModuleId,
        method_ident: &str,
        callback: F,
    ) -> Result<O, RuntimeError>
    where
        N: Into<NodeId> + Copy,
        F: FnOnce(&mut Self) -> O,
        O: ScryptoEncode,
    {
        let object_info = self.0.with_kernel_mut(|kernel: &mut TestKernel<'_>| {
            SystemService {
                api: kernel,
                phantom: PhantomData,
            }
            .get_object_info(&node_id.into())
        })?;
        let auth_zone = self.0.with_kernel_mut(|kernel| {
            let mut system_service = SystemService {
                api: kernel,
                phantom: PhantomData,
            };
            AuthModule::create_mock(
                &mut system_service,
                Some((&node_id.into(), false)),
                Default::default(),
                Default::default(),
            )
        })?;
        let actor = Actor::Method(MethodActor {
            method_type: match module_id {
                ObjectModuleId::Main => MethodType::Main,
                ObjectModuleId::Royalty => MethodType::Module(ModuleId::Royalty),
                ObjectModuleId::Metadata => MethodType::Module(ModuleId::Metadata),
                ObjectModuleId::RoleAssignment => MethodType::Module(ModuleId::RoleAssignment),
            },
            ident: method_ident.to_owned(),
            node_id: node_id.into(),
            auth_zone,
            object_info,
        });
        self.as_actor(actor, callback)
    }

    /// Allows us to perform some action as another actor.
    ///
    /// This function pushes a new call-frame onto the stack with the actor information we desire,
    /// performs the call-back, and then pops the call-frame off.
    pub(crate) fn as_actor<F, O>(&mut self, actor: Actor, callback: F) -> Result<O, RuntimeError>
    where
        F: FnOnce(&mut Self) -> O,
        O: ScryptoEncode,
    {
        // Creating the next frame.
        let message = CallFrameMessage::from_input(&IndexedScryptoValue::from_typed(&()), &actor);
        self.0.with_kernel_mut(|kernel| {
            let current_frame = kernel.kernel_current_frame_mut();
            let mut new_frame =
                CallFrame::new_child_from_parent(current_frame, actor, message).unwrap();
            *new_frame.stable_references_mut() = current_frame.stable_references_mut().clone();
            let old = core::mem::replace(current_frame, new_frame);
            kernel.kernel_prev_frame_stack_mut().push(old);
        });

        // Executing the callback
        let rtn = callback(self);

        // Constructing the message from the returns
        let message = {
            let indexed_scrypto_value = IndexedScryptoValue::from_typed(&rtn);
            CallFrameMessage {
                move_nodes: indexed_scrypto_value.owned_nodes().clone(),
                copy_global_references: indexed_scrypto_value.references().clone(),
                ..Default::default()
            }
        };

        // Popping the last frame & Passing message
        self.0
            .with_kernel_mut(|kernel| -> Result<(), RuntimeError> {
                let mut previous_frame = kernel.kernel_prev_frame_stack_mut().pop().unwrap();

                CallFrame::pass_message(
                    kernel.kernel_current_frame_mut(),
                    &mut previous_frame,
                    message.clone(),
                )
                .map_err(CallFrameError::PassMessageError)
                .map_err(KernelError::CallFrameError)?;

                *kernel.kernel_current_frame_mut() = previous_frame;
                Ok(())
            })?;

        Ok(rtn)
    }
}

#[cfg(test)]
mod tests {
    use super::TestEnvironment;

    #[test]
    pub fn test_env_can_be_created() {
        let _ = TestEnvironment::new();
    }
}
