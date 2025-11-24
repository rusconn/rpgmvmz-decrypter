mod system_json;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

use crate::decrypter::{self, Decrypter};

use self::system_json::SystemJson;

pub fn decrypt(game_dir: &Path) -> Result<(), DecryptionError> {
    let system_json = SystemJson::read(game_dir)?;
    let decrypter = Decrypter::new(&system_json.encryption_key)?;

    WalkDir::new(game_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .filter_map(Plan::new)
        .try_for_each(|Plan { source, dest }| do_decrypt(&source, &dest, &decrypter))?;

    mark_as_unencrypted(system_json)?;

    Ok(())
}

fn do_decrypt(source: &Path, dest: &Path, decrypter: &Decrypter) -> Result<(), DecryptionError> {
    let mut bytes = fs::read(source)?;

    fs::write(dest, decrypter.decrypt(&mut bytes))?;
    fs::remove_file(source)?;

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

static EXT_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "rpgmvo" => "ogg",
    "rpgmvm" => "m4a",
    "rpgmvp" => "png",
    "ogg_" => "ogg",
    "m4a_" => "m4a",
    "png_" => "png",
};

fn mark_as_unencrypted(
    SystemJson { mut content, path, .. }: SystemJson,
) -> Result<(), DecryptionError> {
    content.remove("hasEncryptedImages");
    content.remove("hasEncryptedAudio");

    fs::write(&path, serde_json::to_string(&content).expect("success"))?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum DecryptionError {
    #[error(transparent)]
    ReadSystemJson(#[from] system_json::ReadError),

    #[error("invalid encryptionKey: {0}")]
    InvalidEncryptionKey(#[from] decrypter::InitError),

    #[error(transparent)]
    Io(#[from] io::Error),
}
