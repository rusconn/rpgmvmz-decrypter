mod plan;
mod system_json;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use thiserror::Error;
use walkdir::WalkDir;

use crate::{Encrypted, encrypted, encryption_key::EncryptionKey, system_json as mem_system_json};

use {plan::Plan, system_json::SystemJson};

pub fn decrypt(game_dir: &Path) -> Result<(), DecryptionError> {
    let mut system_json = SystemJson::new(game_dir)?;

    WalkDir::new(game_dir)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DecryptionError::ScanDirectory {
            path: e.path().map(Path::to_path_buf),
            source: e
                .into_io_error()
                .expect("success since loops won't occur because of follow_links=false"),
        })?
        .into_par_iter()
        .filter_map(Plan::new)
        .try_for_each(|plan| do_decrypt(&plan, system_json.get_encryption_key()))?;

    system_json.save_as_unencrypted()?;

    Ok(())
}

fn do_decrypt(plan: &Plan, encryption_key: &EncryptionKey) -> Result<(), DecryptionError> {
    let bytes = fs::read(&plan.source) //
        .map_err(|source| DecryptionError::ReadEncryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    let encrypted = Encrypted::new(bytes) //
        .map_err(|source| DecryptionError::InvalidEncryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    let decrypted_view = encrypted.into_decrypted_view(encryption_key);

    fs::write(&plan.dest, decrypted_view.as_bytes()) //
        .map_err(|source| DecryptionError::WriteDecryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    fs::remove_file(&plan.source) //
        .map_err(|source| DecryptionError::RemoveEncryptedFile {
            path: plan.source.clone(),
            source,
        })
}

#[derive(Debug, Error)]
pub enum DecryptionError {
    #[error("{0} not exists")]
    PathNotExists(PathBuf),

    #[error("{0} is not a directory")]
    PathIsNotADirectory(PathBuf),

    #[error("System.json not found")]
    SystemJsonNotFound,

    #[error("failed to read System.json({path})")]
    ReadSystemJson {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse System.json({path})")]
    ParseSystemJson {
        path: PathBuf,
        #[source]
        source: mem_system_json::ParseError,
    },

    #[error("failed to scan({path:?})")]
    ScanDirectory {
        path: Option<PathBuf>,
        #[source]
        source: io::Error,
    },

    #[error("failed to read encrypted file({path})")]
    ReadEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to decrypt file({path})")]
    InvalidEncryptedFile {
        path: PathBuf,
        #[source]
        source: encrypted::InvalidEncryptedBytesError,
    },

    #[error("failed to write decrypted file({path})")]
    WriteDecryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to remove encrypted file({path})")]
    RemoveEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to mark System.json as unencrypted({path})")]
    MarkSystemJsonAsUnencrypted {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl From<system_json::NewError> for DecryptionError {
    fn from(value: system_json::NewError) -> Self {
        match value {
            system_json::NewError::PathNotExists(path_buf) => {
                DecryptionError::PathNotExists(path_buf)
            }
            system_json::NewError::PathIsNotADirectory(path_buf) => {
                DecryptionError::PathIsNotADirectory(path_buf)
            }
            system_json::NewError::SystemJsonNotFound => DecryptionError::SystemJsonNotFound,
            system_json::NewError::ReadSystemJson { path, source } => {
                DecryptionError::ReadSystemJson { path, source }
            }
            system_json::NewError::ParseSystemJson { path, source } => {
                DecryptionError::ParseSystemJson { path, source }
            }
        }
    }
}

impl From<system_json::SaveError> for DecryptionError {
    fn from(value: system_json::SaveError) -> Self {
        match value {
            system_json::SaveError::Io { path, source } => {
                DecryptionError::MarkSystemJsonAsUnencrypted { path, source }
            }
        }
    }
}
