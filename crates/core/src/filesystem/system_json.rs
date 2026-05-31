use std::{
    fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{encryption_key::EncryptionKey, system_json as mem_system_json};

pub(super) struct SystemJson {
    path: PathBuf,
    mem_system_json: mem_system_json::SystemJson,
}

impl SystemJson {
    pub(super) fn new(game_dir: &Path) -> Result<Self, NewError> {
        let (path, content) = Self::read_system_json(game_dir)?;
        let mem_system_json = content
            .parse::<mem_system_json::SystemJson>()
            .map_err(|source| NewError::ParseSystemJson { path: path.clone(), source })?;
        Ok(Self { path, mem_system_json })
    }

    fn read_system_json(game_dir: &Path) -> Result<(PathBuf, String), NewError> {
        if !game_dir.exists() {
            return Err(NewError::PathNotExists(game_dir.into()));
        }
        if !game_dir.is_dir() {
            return Err(NewError::PathIsNotADirectory(game_dir.into()));
        }

        let mv_path = game_dir.join("www").join("data").join("System.json");
        let mz_path = game_dir.join("data").join("System.json");

        let system_json_path = [mv_path, mz_path]
            .into_iter()
            .find(|p| p.exists())
            .ok_or(NewError::SystemJsonNotFound)?;

        fs::read_to_string(&system_json_path) //
            .map_err(|source| NewError::ReadSystemJson {
                path: system_json_path.clone(),
                source,
            })
            .map(|s| (system_json_path, s))
    }

    pub(super) fn get_encryption_key(&self) -> &EncryptionKey {
        self.mem_system_json.get_encryption_key()
    }

    pub(super) fn save_as_unencrypted(&mut self) -> Result<(), SaveError> {
        self.mem_system_json.mark_as_unencrypted();

        fs::write(&self.path, self.mem_system_json.to_string()) //
            .map_err(|source| SaveError::Io {
                path: self.path.clone(),
                source,
            })
    }
}

#[derive(Debug, Error)]
pub enum NewError {
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
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("failed to save System.json as unencrypted({path})")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
