use crate::error::HashError;
use bincode::config;
use bincode::serde::{decode_from_slice, encode_to_vec};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashedId([u8; 32]);

impl HashedId {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Display for HashedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

pub(crate) fn hash_canonical_bytes(bytes: &[u8]) -> HashedId {
    let digest = Sha256::digest(bytes);
    HashedId(digest.into())
}

pub(crate) fn canonical_encode<T: Serialize>(value: &T) -> Vec<u8> {
    let config = config::standard();
    encode_to_vec(value, config).expect("canonical encoding should not fail")
}

pub(crate) fn canonical_decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, HashError> {
    let config = config::standard();
    decode_from_slice(bytes, config)
        .map(|(value, _)| value)
        .map_err(|_| HashError::CannotDecode)
}
