use crate::account::Account;
use crate::error::LedgerError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ledger {
    accounts: HashMap<String, Account>,
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

    pub fn apply_transfer(&mut self, transfer: &Transfer) -> Result<(), LedgerError> {
        // 1. Validate (immutable borrows die at end of this block)
        {
            // amount is zero
            if transfer.amount == 0 {
                return Err(LedgerError::ZeroAmount);
            }

            // sender and receiver are the same
            if transfer.from == transfer.to {
                return Err(LedgerError::SelfTransfer);
            }

            // sender does not exist
            let Some(sender) = self.account(transfer.from.clone()) else {
                return Err(LedgerError::SenderNotFound(transfer.from.clone()));
            };

            // receiver does not exist
            let Some(receiver) = self.account(transfer.to.clone()) else {
                return Err(LedgerError::ReceiverNotFound(transfer.to.clone()));
            };

            // transfer nonce does not match sender nonce
            if sender.nonce != transfer.nonce {
                return Err(LedgerError::IncorrectNonce {
                    expected: sender.nonce,
                    received: transfer.nonce,
                });
            }

            // sender does not have enough balance
            if sender.balance < transfer.amount {
                return Err(LedgerError::InsufficientBalance {
                    available: sender.balance,
                    requested: transfer.amount,
                });
            }

            // receiver balance would overflow
            if receiver.balance > u64::MAX - transfer.amount {
                return Err(LedgerError::BalanceOverflow);
            }
        } // sender & receiver borrows end here

        // 2. Mutate — one account at a time
        // For a valid transfer:
        // sender balance decreases by amount
        let sender = self.accounts.get_mut(&transfer.from).unwrap();
        sender.balance -= transfer.amount;
        // sender nonce increases by 1
        sender.nonce += 1;

        // receiver balance increases by amount
        let receiver = self.accounts.get_mut(&transfer.to).unwrap();
        receiver.balance += transfer.amount;
        // Receiver nonce should not change.

        Ok(())
    }

    pub fn total_supply(&self) -> u64 {
        self.accounts.values().map(|acc| acc.balance).sum()
    }
}
