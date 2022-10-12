use crate::constants::{DEFAULT_COST_UNIT_LIMIT, DEFAULT_COST_UNIT_PRICE, DEFAULT_SYSTEM_LOAN};
use crate::fee::FeeSummary;
use crate::model::Resource;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum FeeReserveError {
    OutOfCostUnit,
    Overflow,
    LimitExceeded,
    SystemLoanNotCleared,
}

pub trait FeeReserve {
    fn consume<T: ToString>(
        &mut self,
        n: u32,
        reason: T,
        deferred: bool,
    ) -> Result<(), FeeReserveError>;

    fn repay(
        &mut self,
        vault_id: VaultId,
        fee: Resource,
        contingent: bool,
    ) -> Result<Resource, FeeReserveError>;

    fn finalize(self) -> FeeSummary;

    fn limit(&self) -> u32;

    fn consumed_instant(&self) -> u32;

    fn consumed_deferred(&self) -> u32;

    fn balance(&self) -> u32;

    fn owed(&self) -> u32;
}

#[derive(Debug)]
pub struct SystemLoanFeeReserve {
    /// The price of cost unit
    cost_unit_price: Decimal,
    /// The tip percentage
    tip_percentage: u32,
    /// Payments made during the execution of a transaction.
    payments: Vec<(VaultId, Resource, bool)>,
    /// The balance cost units
    balance: u32,
    /// The number of cost units owed to the system
    owed: u32,
    /// The total cost units consumed so far, instant
    consumed_instant: u32,
    /// The total cost units consumed so far, deferred
    consumed_deferred: u32,
    /// The max number of cost units that can be consumed
    limit: u32,
    /// At which point the system loan repayment is checked
    check_point: u32,
    /// Cost breakdown
    cost_breakdown: HashMap<String, u32>,
}

impl SystemLoanFeeReserve {
    pub fn new(
        cost_unit_limit: u32,
        tip_percentage: u32,
        cost_unit_price: Decimal,
        system_loan: u32,
    ) -> Self {
        Self {
            cost_unit_price,
            tip_percentage,
            payments: Vec::new(),
            balance: system_loan,
            owed: system_loan,
            consumed_instant: 0,
            consumed_deferred: 0,
            limit: cost_unit_limit,
            check_point: system_loan,
            cost_breakdown: HashMap::new(),
        }
    }

    /// Adds free credit.
    ///
    /// Note that overflow is not checked.
    pub fn credit(&mut self, n: u32) {
        self.balance += n;
        self.repay_with_balance();
    }

    fn repay_with_balance(&mut self) {
        let n = u32::min(self.owed, self.balance);
        self.owed -= n;
        self.balance -= n;
    }
}

impl FeeReserve for SystemLoanFeeReserve {
    fn consume<T: ToString>(
        &mut self,
        n: u32,
        reason: T,
        deferred: bool,
    ) -> Result<(), FeeReserveError> {
        // update consumed
        if !deferred {
            self.consumed_instant = self
                .consumed_instant
                .checked_add(n)
                .ok_or(FeeReserveError::Overflow)?;
        } else {
            self.consumed_deferred = self
                .consumed_deferred
                .checked_add(n)
                .ok_or(FeeReserveError::Overflow)?;
        }
        if self.consumed_instant + self.consumed_deferred > self.limit {
            return Err(FeeReserveError::LimitExceeded);
        }

        // update cost breakdown
        self.cost_breakdown
            .entry(reason.to_string())
            .or_default()
            .add_assign(n);

        // update balance or owed
        if !deferred {
            // println!("Trace: {}, {}, {}, {}", self.balance, self.owed, reason.to_string(), n);
            self.balance = self
                .balance
                .checked_sub(n)
                .ok_or(FeeReserveError::OutOfCostUnit)?;
        } else {
            assert!(self.consumed_instant < self.check_point);
            self.owed = self.owed.checked_add(n).ok_or(FeeReserveError::Overflow)?;
        }

        // check system loan
        if self.consumed_instant >= self.check_point && self.owed > 0 {
            self.repay_with_balance();
            if self.owed > 0 {
                return Err(FeeReserveError::SystemLoanNotCleared);
            }
        }
        Ok(())
    }

    fn repay(
        &mut self,
        vault_id: VaultId,
        mut fee: Resource,
        contingent: bool,
    ) -> Result<Resource, FeeReserveError> {
        let effective_cost_unit_price =
            self.cost_unit_price + self.cost_unit_price * self.tip_percentage / 100;

        // TODO: Add `TryInto` implementation once the new decimal types are in place
        let n = u32::from_str(
            (fee.amount() / effective_cost_unit_price)
                .round(0, RoundingMode::TowardsZero)
                .to_string()
                .as_str(),
        )
        .map_err(|_| FeeReserveError::Overflow)?;

        if !contingent {
            if n >= self.owed {
                self.balance = self
                    .balance
                    .checked_add(n - self.owed)
                    .ok_or(FeeReserveError::Overflow)?;
                self.owed = 0;
            } else {
                self.owed -= n;
            }
        }

        let actual_amount = effective_cost_unit_price * n;
        self.payments.push((
            vault_id,
            fee.take_by_amount(actual_amount)
                .expect("Failed to take from fee resource"),
            contingent,
        ));

        Ok(fee)
    }

    fn finalize(mut self) -> FeeSummary {
        self.repay_with_balance();

        let consumed = self.consumed_instant + self.consumed_deferred;
        FeeSummary {
            loan_fully_repaid: self.owed == 0,
            cost_unit_limit: self.limit,
            cost_unit_consumed: consumed,
            cost_unit_price: self.cost_unit_price,
            tip_percentage: self.tip_percentage,
            burned: self.cost_unit_price * consumed,
            tipped: self.cost_unit_price * self.tip_percentage / 100 * consumed,
            payments: self.payments,
            cost_breakdown: self.cost_breakdown,
        }
    }

    fn limit(&self) -> u32 {
        self.limit
    }

    fn consumed_instant(&self) -> u32 {
        self.consumed_instant
    }

    fn consumed_deferred(&self) -> u32 {
        self.consumed_deferred
    }

    fn balance(&self) -> u32 {
        self.balance
    }

    fn owed(&self) -> u32 {
        self.owed
    }
}

impl Default for SystemLoanFeeReserve {
    fn default() -> Self {
        Self::new(
            DEFAULT_COST_UNIT_LIMIT,
            0,
            DEFAULT_COST_UNIT_PRICE
                .parse()
                .expect("Invalid DEFAULT_COST_UNIT_PRICE"),
            DEFAULT_SYSTEM_LOAN,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scrypto::{constants::RADIX_TOKEN, crypto::Hash};

    const TEST_VAULT_ID: VaultId = (Hash([0u8; 32]), 1);

    fn xrd<T: Into<Decimal>>(amount: T) -> Resource {
        Resource::new_fungible(RADIX_TOKEN, 18, amount.into())
    }

    #[test]
    fn test_consume_and_repay() {
        let mut fee_reserve = SystemLoanFeeReserve::new(100, 0, 1.into(), 5);
        fee_reserve.consume(2, "test", false).unwrap();
        fee_reserve.repay(TEST_VAULT_ID, xrd(3), false).unwrap();
        assert_eq!(3, fee_reserve.balance());
        assert_eq!(2, fee_reserve.consumed_instant());
        assert_eq!(2, fee_reserve.owed());
    }

    #[test]
    fn test_out_of_cost_unit() {
        let mut fee_reserve = SystemLoanFeeReserve::new(100, 0, 1.into(), 5);
        assert_eq!(
            Err(FeeReserveError::OutOfCostUnit),
            fee_reserve.consume(6, "test", false)
        );
    }

    #[test]
    fn test_overflow() {
        let mut fee_reserve = SystemLoanFeeReserve::new(100, 0, 1.into(), 0);
        assert_eq!(
            Ok(xrd(0)),
            fee_reserve.repay(TEST_VAULT_ID, xrd(u32::max_value()), false)
        );
        assert_eq!(
            Err(FeeReserveError::Overflow),
            fee_reserve.repay(TEST_VAULT_ID, xrd(1), false)
        );
    }

    #[test]
    fn test_repay() {
        let mut fee_reserve = SystemLoanFeeReserve::new(100, 0, 1.into(), 500);
        fee_reserve.repay(TEST_VAULT_ID, xrd(100), false).unwrap();
        assert_eq!(500, fee_reserve.balance());
        assert_eq!(400, fee_reserve.owed());
    }

    #[test]
    fn test_xrd_cost_unit_conversion() {
        let mut fee_reserve = SystemLoanFeeReserve::new(100, 0, 5.into(), 500);
        fee_reserve.repay(TEST_VAULT_ID, xrd(100), false).unwrap();
        assert_eq!(500, fee_reserve.balance());
        assert_eq!(500 - 100 / 5, fee_reserve.owed());
        assert_eq!(
            vec![(TEST_VAULT_ID, xrd(100), false)],
            fee_reserve.finalize().payments
        )
    }
}
