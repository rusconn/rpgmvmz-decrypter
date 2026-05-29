use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{encryption_key::EncryptionKey, system_json::SystemJson as MemSystemJson};

use super::DecryptionError;

pub(super) struct SystemJson {
    path: PathBuf,
    mem_system_json: MemSystemJson,
}

impl SystemJson {
    pub(super) fn new(game_dir: &Path) -> Result<Self, DecryptionError> {
        let (path, content) = Self::read_system_json(game_dir)?;
        let mem_system_json = content
            .parse::<MemSystemJson>()
            .map_err(|source| DecryptionError::ParseSystemJson { path: path.clone(), source })?;
        Ok(Self { path, mem_system_json })
    }

    fn read_system_json(game_dir: &Path) -> Result<(PathBuf, String), DecryptionError> {
        if !game_dir.exists() {
            return Err(DecryptionError::NotExists(game_dir.into()));
        }
        if !game_dir.is_dir() {
            return Err(DecryptionError::NotADirectory(game_dir.into()));
        }

        let mv_path = game_dir.join("www").join("data").join("System.json");
        let mz_path = game_dir.join("data").join("System.json");

        let system_json_path = [mv_path, mz_path]
            .into_iter()
            .find(|p| p.exists())
            .ok_or(DecryptionError::SystemJsonNotFound)?;

        fs::read_to_string(&system_json_path) //
            .map_err(|source| DecryptionError::ReadSystemJson {
                path: system_json_path.clone(),
                source,
            })
            .map(|s| (system_json_path, s))
    }

    pub(super) fn get_encryption_key(&self) -> &EncryptionKey {
        self.mem_system_json.get_encryption_key()
    }

    pub(super) fn save_as_unencrypted(&mut self) -> Result<(), DecryptionError> {
        self.mem_system_json.mark_as_unencrypted();

        fs::write(&self.path, self.mem_system_json.to_string()) //
            .map_err(|source| DecryptionError::MarkSystemJsonAsUnencrypted {
                path: self.path.clone(),
                source,
            })
    }
}
