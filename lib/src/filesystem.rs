mod plan;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use thiserror::Error;
use walkdir::WalkDir;

use crate::{
    decrypter,
    encryption_key::EncryptionKey,
    system_json::{self, SystemJson},
};

use self::plan::Plan;

pub use system_json::{InvalidEncryptionKeyError, ParseError as ParseSystemJsonError};

pub fn decrypt(game_dir: &Path) -> Result<(), DecryptionError> {
    let (path, content) = read_system_json(game_dir)?;

    let mut system_json = content
        .parse::<SystemJson>()
        .map_err(|source| DecryptionError::ParseSystemJson { path: path.clone(), source })?;

    WalkDir::new(game_dir)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        // since follow_links=false, loops won't occur.
        .map_err(|e| DecryptionError::Scan {
            path: e.path().map(Path::to_path_buf),
            source: e.into_io_error().unwrap(),
        })?
        .into_iter()
        .filter_map(Plan::new)
        .par_bridge()
        .try_for_each(|plan| do_decrypt(&plan, &system_json.encryption_key))?;

    system_json.mark_as_unencrypted();

    write_system_json(&path, &system_json)
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

fn do_decrypt(plan: &Plan, encryption_key: &EncryptionKey) -> Result<(), DecryptionError> {
    let mut bytes = fs::read(&plan.source) //
        .map_err(|source| DecryptionError::ReadEncryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    fs::write(&plan.dest, decrypter::decrypt(&mut bytes, encryption_key)) //
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

fn write_system_json(path: &Path, system_json: &SystemJson) -> Result<(), DecryptionError> {
    fs::write(
        path,
        serde_json::to_string(&system_json.content).expect("success"),
    )
    .map_err(|source| DecryptionError::MarkSystemJsonAsUnencrypted {
        path: path.to_path_buf(),
        source,
    })
}

#[derive(Debug, Error)]
pub enum DecryptionError {
    #[error("{0} not exists")]
    NotExists(PathBuf),

    #[error("{0} is not a directory")]
    NotADirectory(PathBuf),

    #[error("System.json not found")]
    SystemJsonNotFound,

    #[error("failed to read System.json({path}): {source}")]
    ReadSystemJson {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse System.json({path}): {source}")]
    ParseSystemJson {
        path: PathBuf,
        #[source]
        source: ParseSystemJsonError,
    },

    #[error("failed to scan({path:?}): {source}")]
    Scan {
        path: Option<PathBuf>,
        #[source]
        source: io::Error,
    },

    #[error("failed to read encrypted file({path}): {source}")]
    ReadEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to write decrypted file({path}): {source}")]
    WriteDecryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to remove encrypted file({path}): {source}")]
    RemoveEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to mark System.json as unencrypted({path}): {source}")]
    MarkSystemJsonAsUnencrypted {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
