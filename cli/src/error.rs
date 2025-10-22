use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("invalid encryptionKey: {0:?}")]
    InvalidEncryptionKey(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] io::Error),
}
