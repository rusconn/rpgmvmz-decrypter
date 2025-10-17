mod config;
mod decrypter;
mod system_json;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result};
use phf::phf_map;
use rayon::prelude::*;
use walkdir::WalkDir;

use self::{config::Config, decrypter::Decrypter, system_json::SystemJson};

fn main() {
    let config = Config::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    let dest_root = add_suffix(&config.game_dir, "_decrypted") //
        .context("Something went wrong.")?;
    let system_json = SystemJson::read(&config.game_dir)?;
    let decrypter = Decrypter::new(system_json.encryption_key)?;

    WalkDir::new(&config.game_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .try_for_each(|file| {
            copy_with_decryption(&config.game_dir, &dest_root, &file, &decrypter)
        })?;

    remove_encryption_info(&dest_root)?;

    Ok(())
}

fn add_suffix(path: &Path, suffix: &str) -> Option<PathBuf> {
    let parent = path.parent().unwrap_or(Path::new(""));
    let file_name = path.file_name()?.to_str()?;
    Some(parent.join(format!("{file_name}{suffix}")))
}

fn copy_with_decryption(
    game_dir: &Path,
    dest_root: &Path,
    source: &Path,
    decrypter: &Decrypter,
) -> Result<()> {
    let (dest, do_decrypt) = plan(game_dir, dest_root, source);

    if let Some(dest_parent) = dest.parent() {
        if !fs::exists(dest_parent)? {
            fs::create_dir_all(dest_parent)?;
        }
    }

    if do_decrypt {
        let mut bytes = fs::read(source)?;
        fs::write(dest, decrypter.decrypt(&mut bytes))?;
    } else {
        fs::copy(source, dest)?;
    }

    Ok(())
}

fn plan(game_dir: &Path, dest_root: &Path, source: &Path) -> (PathBuf, bool) {
    let dest_path = dest_root.join(source.strip_prefix(game_dir).unwrap());
    let ext = source.extension().and_then(|s| s.to_str()).unwrap_or("");
    let dest = dest_path.with_extension(EXT_MAP.get(ext).unwrap_or(&ext));
    let do_decrypt = EXT_MAP.contains_key(ext);
    (dest, do_decrypt)
}

static EXT_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "rpgmvo" => "ogg",
    "rpgmvm" => "m4a",
    "rpgmvp" => "png",
    "ogg_" => "ogg",
    "m4a_" => "m4a",
    "png_" => "png",
};

fn remove_encryption_info(dest_root: &Path) -> Result<()> {
    let SystemJson { path, mut content, .. } = SystemJson::read(dest_root)?;

    content.remove("hasEncryptedImages");
    content.remove("hasEncryptedAudio");
    content.remove("encryptionKey");

    fs::write(&path, serde_json::to_string(&content)?)?;

    Ok(())
}
