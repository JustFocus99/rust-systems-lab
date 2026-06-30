use crate::error::HashError;
use crate::hash::{canonical_decode, canonical_encode, hash_canonical_bytes, HashedId};
use crate::TransactionValidationError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
}

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for Transaction {
    type Error = TransactionValidationError;

    fn validate(&self) -> Result<(), TransactionValidationError> {
        if self.amount == 0 {
            return Err(TransactionValidationError::ZeroAmount);
        }
        if self.sender == self.receiver {
            return Err(TransactionValidationError::SelfTransfer);
        }
        Ok(())
    }
}

impl Transaction {
    pub fn new(
        sender: impl Into<String>,
        receiver: impl Into<String>,
        amount: u64,
        nonce: u64,
    ) -> Self {
        Self {
            sender: sender.into(),
            receiver: receiver.into(),
            amount,
            nonce,
        }
    }

    pub fn hash_id(&self) -> HashedId {
        hash_canonical_bytes(&self.canonical_bytes())
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        canonical_encode(self)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, HashError> {
        canonical_decode(bytes)
    }
}
