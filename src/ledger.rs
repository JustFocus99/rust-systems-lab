use crate::account::Account;
use crate::error::LedgerError;
use crate::transaction::Transaction;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ledger {
    pub accounts: HashMap<String, Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(
        &mut self,
        _id: impl Into<String>,
        balance: u64,
    ) -> Result<(), LedgerError> {
        let id = _id.into();
        // check if account id already exists
        match self.accounts.get(&id) {
            Some(_) => Err(LedgerError::AccountAlreadyExists(id.clone())),
            None => {
                self.accounts.insert(id.clone(), Account::new(balance));
                Ok(())
            }
        }
    }

    pub fn account(&self, _id: impl Into<String>) -> Option<&Account> {
        let id = _id.into();
        self.accounts.get(&id)
    }

    pub(crate) fn account_mut(&mut self, _id: impl Into<String>) -> Option<&mut Account> {
        let id = _id.into();
        self.accounts.get_mut(&id)
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) -> Result<(), LedgerError> {
        crate::traits::StateTransition::apply(self, transaction)
    }

    pub fn total_supply(&self) -> u64 {
        self.accounts.values().map(|acc| acc.balance).sum()
    }
}
