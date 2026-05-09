mod decrypted_view;

use thiserror::Error;

use crate::EncryptionKey;

use decrypted_view::DecryptedView;

pub struct Encrypted(Vec<u8>);

static ENCRYPTION_HEADER: [u8; 16] = [
    0x52, 0x50, 0x47, 0x4d, 0x56, 0x00, 0x00, 0x00, 0x00, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
];

impl Encrypted {
    pub fn new(bytes: Vec<u8>) -> Result<Self, InvalidEncryptedBytesError> {
        if !bytes.starts_with(&ENCRYPTION_HEADER) {
            return Err(InvalidEncryptedBytesError::InvalidEncryptionHeader);
        }

        Ok(Self(bytes))
    }

    pub fn into_decrypted_view(mut self, encryption_key: &EncryptionKey) -> DecryptedView {
        let body = &mut self.0[ENCRYPTION_HEADER.len()..];
        for (bit, mask) in body.iter_mut().zip(encryption_key.iter()) {
            *bit ^= mask;
        }
        DecryptedView(self.0)
    }
}

#[derive(Debug, Error)]
pub enum InvalidEncryptedBytesError {
    #[error("invalid encryption header")]
    InvalidEncryptionHeader,
}
