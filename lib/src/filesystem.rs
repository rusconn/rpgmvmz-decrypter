mod system_json;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use thiserror::Error;
use walkdir::WalkDir;

use crate::decrypter::{self, Decrypter};

use self::system_json::SystemJson;

pub fn decrypt(game_dir: &Path) -> Result<(), DecryptionError> {
    let system_json = SystemJson::read(game_dir)?;
    let decrypter = Decrypter::new(&system_json.encryption_key)?;

    WalkDir::new(game_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .map(Plan::new)
        .filter(|Plan { do_decrypt, .. }| *do_decrypt)
        .try_for_each(|Plan { source, dest, .. }| do_decrypt(&source, &dest, &decrypter))?;

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
    do_decrypt: bool,
}

impl Plan {
    fn new(source: PathBuf) -> Plan {
        let ext = source.extension().and_then(|s| s.to_str()).unwrap_or("");
        let dest = source.with_extension(EXT_MAP.get(ext).unwrap_or(&ext));
        let do_decrypt = EXT_MAP.contains_key(ext);
        Self { source, dest, do_decrypt }
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
