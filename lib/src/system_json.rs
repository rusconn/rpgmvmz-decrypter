use std::str::FromStr;

use serde_json::{Map, Value};
use thiserror::Error;

use crate::encryption_key::{self, EncryptionKey};

pub struct SystemJson {
    pub encryption_key: EncryptionKey,
    pub content: Map<String, Value>,
}

impl FromStr for SystemJson {
    type Err = ParseError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let Ok(content) = content.parse::<Map<String, Value>>() else {
            return Err(Self::Err::NotAnObject);
        };
        let Some(encryption_key) = content.get("encryptionKey") else {
            return Err(Self::Err::EncryptionKeyNotExists);
        };
        let Value::String(encryption_key) = encryption_key else {
            return Err(Self::Err::EncryptionKeyIsNotAString);
        };

        let encryption_key = encryption_key
            .parse()
            .map_err(
                |e: encryption_key::ParseError| Self::Err::InvalidEncryptionKey {
                    encryption_key: encryption_key.into(),
                    source: e.into(),
                },
            )?;

        Ok(Self { encryption_key, content })
    }
}

impl SystemJson {
    pub fn mark_as_unencrypted(&mut self) {
        self.content.remove("hasEncryptedAudio");
        self.content.remove("hasEncryptedImages");
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("content is not an object")]
    NotAnObject,

    #[error("encryptionKey not exists")]
    EncryptionKeyNotExists,

    #[error("encryptionKey is not a string")]
    EncryptionKeyIsNotAString,

    #[error("invalid encryptionKey({encryption_key}): {source}")]
    InvalidEncryptionKey {
        encryption_key: String,
        #[source]
        source: InvalidEncryptionKeyError,
    },
}

#[derive(Debug, Error)]
pub enum InvalidEncryptionKeyError {
    #[error("invalid character at index {index}: {c:?}")]
    InvalidCharacter { c: char, index: usize },

    #[error("invalid length")]
    InvalidLength,
}

impl From<encryption_key::ParseError> for InvalidEncryptionKeyError {
    fn from(e: encryption_key::ParseError) -> Self {
        match e {
            encryption_key::ParseError::InvalidCharacter { c, index } => {
                Self::InvalidCharacter { c, index }
            }
            encryption_key::ParseError::InvalidLength => Self::InvalidLength,
        }
    }
}
