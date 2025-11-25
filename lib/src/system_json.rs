use std::str::FromStr;

use serde_json::{Map, Value};
use thiserror::Error;

pub struct SystemJson {
    pub encryption_key: String,
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

        Ok(Self {
            encryption_key: encryption_key.into(),
            content,
        })
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
    #[error("System.json is not an object")]
    NotAnObject,

    #[error("encryptionKey not exists")]
    EncryptionKeyNotExists,

    #[error("encryptionKey is not a string")]
    EncryptionKeyIsNotAString,
}
