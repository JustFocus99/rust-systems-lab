use crate::error::LedgerError;
use crate::ledger::Ledger;
use crate::transaction::{Transaction, Validate};

pub trait StateTransition<T> {
    type Error;
    fn apply(&mut self, payload: T) -> Result<(), Self::Error>;
}

impl StateTransition<Transaction> for Ledger {
    type Error = LedgerError;

    fn apply(&mut self, transaction: Transaction) -> Result<(), Self::Error> {
        // 1. Validate (immutable borrows die at end of this block)
        {
            // check if `amount is zero` or `sender and receiver are the same`
            transaction.validate()?;

            // sender does not exist
            let Some(sender) = self.account(transaction.sender.clone()) else {
                return Err(LedgerError::SenderNotFound(transaction.sender.clone()));
            };

            // receiver does not exist
            let Some(receiver) = self.account(transaction.receiver.clone()) else {
                return Err(LedgerError::ReceiverNotFound(transaction.receiver.clone()));
            };

            // transaction nonce does not match sender nonce
            if sender.nonce != transaction.nonce {
                return Err(LedgerError::IncorrectNonce {
                    expected: sender.nonce,
                    received: transaction.nonce,
                });
            }

            // sender does not have enough balance
            if sender.balance < transaction.amount {
                return Err(LedgerError::InsufficientBalance {
                    available: sender.balance,
                    requested: transaction.amount,
                });
            }

            // receiver balance would overflow
            if receiver.balance > u64::MAX - transaction.amount {
                return Err(LedgerError::BalanceOverflow);
            }
        } // sender & receiver borrows end here

        // 2. Mutate — one account at a time
        // For a valid transaction:

        // sender balance decreases by amount
        let sender = self.account_mut(&transaction.sender).unwrap();
        sender.balance -= transaction.amount;

        // sender nonce increases by 1
        sender.nonce += 1;

        // receiver balance increases by amount
        let receiver = self.account_mut(&transaction.receiver).unwrap();
        receiver.balance += transaction.amount;
        // Receiver nonce should not change.

        Ok(())
    }
}
