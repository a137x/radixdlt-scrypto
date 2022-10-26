use scrypto::core::NetworkDefinition;
use transaction::errors::TransactionValidationError;
use transaction::model::PreviewIntent;
use transaction::validation::IntentHashManager;
use transaction::validation::NotarizedTransactionValidator;
use transaction::validation::ValidationConfig;

use crate::constants::PREVIEW_CREDIT;
use crate::engine::ScryptoInterpreter;
use crate::fee::SystemLoanFeeReserve;
use crate::ledger::*;
use crate::transaction::TransactionReceipt;
use crate::transaction::*;
use crate::types::*;
use crate::wasm::{WasmEngine, WasmInstance};

#[derive(Debug)]
pub struct PreviewResult {
    pub intent: PreviewIntent,
    pub receipt: TransactionReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewError {
    TransactionValidationError(TransactionValidationError),
}

pub struct PreviewExecutor<'s, 'w, 'n, S, W, I, IHM>
where
    S: ReadableSubstateStore,
    W: WasmEngine<I>,
    I: WasmInstance,
    IHM: IntentHashManager,
{
    substate_store: &'s mut S,
    scrypto_interpreter: &'w mut ScryptoInterpreter<I, W>,
    intent_hash_manager: &'w IHM,
    network: &'n NetworkDefinition,
    phantom1: PhantomData<I>,
}

impl<'s, 'w, 'n, S, W, I, IHM> PreviewExecutor<'s, 'w, 'n, S, W, I, IHM>
where
    S: ReadableSubstateStore,
    W: WasmEngine<I>,
    I: WasmInstance,
    IHM: IntentHashManager,
{
    pub fn new(
        substate_store: &'s mut S,
        scrypto_interpreter: &'w mut ScryptoInterpreter<I, W>,
        intent_hash_manager: &'w IHM,
        network: &'n NetworkDefinition,
    ) -> Self {
        PreviewExecutor {
            substate_store,
            scrypto_interpreter,
            intent_hash_manager,
            network,
            phantom1: PhantomData,
        }
    }

    pub fn execute(
        &mut self,
        preview_intent: PreviewIntent,
    ) -> Result<PreviewResult, PreviewError> {
        let validation_config = ValidationConfig::default(self.network.id);

        let validator = NotarizedTransactionValidator::new(validation_config);

        let validated_preview_transaction = validator
            .validate_preview_intent(preview_intent.clone(), self.intent_hash_manager)
            .map_err(PreviewError::TransactionValidationError)?;

        let mut transaction_executor =
            TransactionExecutor::new(self.substate_store, self.scrypto_interpreter);

        let mut fee_reserve = SystemLoanFeeReserve::default();
        if preview_intent.flags.unlimited_loan {
            fee_reserve.credit(PREVIEW_CREDIT);
        }
        let receipt = transaction_executor.execute_with_fee_reserve(
            &validated_preview_transaction,
            &ExecutionConfig::default(),
            SystemLoanFeeReserve::default(),
        );

        Ok(PreviewResult {
            intent: preview_intent,
            receipt,
        })
    }
}
