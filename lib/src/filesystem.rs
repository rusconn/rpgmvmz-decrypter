mod system_json;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use phf::{Map, phf_map};
use rayon::prelude::*;
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

use crate::decrypter::{self, Decrypter};

use self::system_json::SystemJson;

pub fn decrypt(game_dir: &Path) -> Result<(), DecryptionError> {
    let mut system_json = SystemJson::read(game_dir)?;
    let decrypter = Decrypter::new(&system_json.encryption_key)?;

    WalkDir::new(game_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .filter_map(Plan::new)
        .try_for_each(|plan| do_decrypt(&plan, &decrypter))?;

    system_json.mark_as_unencrypted()?;

    Ok(())
}

fn do_decrypt(plan: &Plan, decrypter: &Decrypter) -> Result<(), DecryptionError> {
    let mut bytes = fs::read(&plan.source) //
        .map_err(|source| DecryptionError::ReadEncryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    fs::write(&plan.dest, decrypter.decrypt(&mut bytes)) //
        .map_err(|error| DecryptionError::WriteDecryptedFile {
            path: plan.source.clone(),
            source: error,
        })?;

    fs::remove_file(&plan.source) //
        .map_err(|source| DecryptionError::RemoveEncryptedFile {
            path: plan.source.clone(),
            source,
        })?;

    Ok(())
}

struct Plan {
    source: PathBuf,
    dest: PathBuf,
}

impl Plan {
    fn new(entry: DirEntry) -> Option<Plan> {
        let source = entry.into_path();
        if !source.is_file() {
            return None;
        }

        let ext = source.extension()?.to_str()?;
        let ext = EXT_MAP.get(ext)?;
        let dest = source.with_extension(ext);

        Some(Self { source, dest })
    }
}

static EXT_MAP: Map<&'static str, &'static str> = phf_map! {
    "rpgmvo" => "ogg",
    "rpgmvm" => "m4a",
    "rpgmvp" => "png",
    "ogg_" => "ogg",
    "m4a_" => "m4a",
    "png_" => "png",
};

#[derive(Debug, Error)]
pub enum DecryptionError {
    #[error(transparent)]
    ReadSystemJson(#[from] system_json::ReadError),

    #[error("invalid encryptionKey: {0}")]
    InvalidEncryptionKey(#[from] decrypter::InitError),

    #[error("failed to read encrypted file {path}: {source}")]
    ReadEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to write decrypted file {path}: {source}")]
    WriteDecryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to remove encrypted file {path}: {source}")]
    RemoveEncryptedFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error(transparent)]
    MarkSystemJsonAsUnencrypted(#[from] system_json::MarkError),
}
