use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};
use thiserror::Error;

pub(super) struct SystemJson {
    pub(super) path: PathBuf,
    pub(super) encryption_key: String,
    pub(super) content: Map<String, Value>,
}

impl SystemJson {
    pub(super) fn read(game_dir: &Path) -> Result<SystemJson, ReadError> {
        if !game_dir.exists() {
            return Err(ReadError::GameDirNotExists(game_dir.into()));
        }
        if !game_dir.is_dir() {
            return Err(ReadError::GameDirIsNotADirectory(game_dir.into()));
        }

        let mv_path = game_dir.join("www").join("data").join("System.json");
        let mz_path = game_dir.join("data").join("System.json");

        let system_json_path = [mv_path, mz_path]
            .into_iter()
            .find(|p| p.exists())
            .ok_or(ReadError::SystemJsonNotExists)?;

        let read = fs::read_to_string(&system_json_path)?;

        let Ok(content) = read.parse::<Map<String, Value>>() else {
            return Err(ReadError::InvalidSystemJsonContent);
        };
        let Some(encryption_key) = content.get("encryptionKey") else {
            return Err(ReadError::EncryptionKeyNotExists);
        };
        let Value::String(encryption_key) = encryption_key else {
            return Err(ReadError::EncryptionKeyIsNotAString);
        };

        Ok(Self {
            path: system_json_path,
            encryption_key: encryption_key.into(),
            content,
        })
    }
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("{0} not exists")]
    GameDirNotExists(PathBuf),

    #[error("{0} is not a directory")]
    GameDirIsNotADirectory(PathBuf),

    #[error("System.json not exists")]
    SystemJsonNotExists,

    #[error("invalid System.json content")]
    InvalidSystemJsonContent,

    #[error("encryptionKey not exists")]
    EncryptionKeyNotExists,

    #[error("encryptionKey is not a string")]
    EncryptionKeyIsNotAString,

    #[error(transparent)]
    Io(#[from] io::Error),
}
