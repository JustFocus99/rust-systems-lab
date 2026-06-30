use crate::error::HashError;
use crate::hash::{canonical_decode, canonical_encode, hash_canonical_bytes, HashedId};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub previous_hash: [u8; 32],
    pub transaction_commitment: [u8; 32],
    pub state_commitment: [u8; 32],
}

impl BlockHeader {
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

impl Block {
    pub fn hash(&self) -> HashedId {
        self.header.hash_id()
    }

    pub fn transaction_commitment(transactions: &[Transaction]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for transaction in transactions {
            hasher.update(transaction.hash_id().as_bytes());
        }
        hasher.finalize().into()
    }
}
