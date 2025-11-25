use hex::FromHexError;
use thiserror::Error;

pub struct Decrypter {
    masks: Vec<u8>,
}

impl Decrypter {
    pub fn new(encryption_key: &str) -> Result<Self, InitError> {
        let masks = hex::decode(encryption_key)?;
        Ok(Self { masks })
    }

    pub fn decrypt<'a>(&self, bytes: &'a mut [u8]) -> &'a [u8] {
        let body = &mut bytes[16..]; // first 16 bytes are rpg maker's header
        for i in 0..(usize::min(body.len(), self.masks.len())) {
            body[i] ^= self.masks[i];
        }
        body
    }
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("invalid character at index {index}: {c:?}")]
    InvalidCharacter { c: char, index: usize },

    #[error("invalid length")]
    InvalidLength,
}

impl From<FromHexError> for InitError {
    fn from(e: FromHexError) -> Self {
        match e {
            FromHexError::InvalidHexCharacter { c, index } => {
                InitError::InvalidCharacter { c, index }
            }
            FromHexError::OddLength => InitError::InvalidLength,
            FromHexError::InvalidStringLength => InitError::InvalidLength,
        }
    }
}
