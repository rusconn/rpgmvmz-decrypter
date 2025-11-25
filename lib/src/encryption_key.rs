use std::str::FromStr;

use hex::FromHexError;
use thiserror::Error;

pub struct EncryptionKey(Vec<u8>);

impl FromStr for EncryptionKey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let masks = hex::decode(s)?;
        Ok(Self(masks))
    }
}

impl EncryptionKey {
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.0.iter().copied()
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid character at index {index}: {c:?}")]
    InvalidCharacter { c: char, index: usize },

    #[error("invalid length")]
    InvalidLength,
}

impl From<FromHexError> for ParseError {
    fn from(e: FromHexError) -> Self {
        match e {
            FromHexError::InvalidHexCharacter { c, index } => Self::InvalidCharacter { c, index },
            FromHexError::OddLength | FromHexError::InvalidStringLength => Self::InvalidLength,
        }
    }
}
