use std::str::FromStr;

use serde_json::{Map, Value};
use thiserror::Error;

use crate::encryption_key::{self, EncryptionKey};

pub use encryption_key::ParseError as InvalidEncryptionKeyError;

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

        let encryption_key =
            encryption_key
                .parse()
                .map_err(|source| Self::Err::InvalidEncryptionKey {
                    encryption_key: encryption_key.into(),
                    source,
                })?;

        Ok(Self { encryption_key, content })
    }
}

impl SystemJson {
    pub fn mark_as_unencrypted(&mut self) {
        self.content["hasEncryptedAudio"] = Value::Bool(false);
        self.content["hasEncryptedImages"] = Value::Bool(false);
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
