mod config;
mod decrypter;
mod system_json;

use std::{env, fs, path::Path, process};

use anyhow::Result;
use rayon::prelude::*;
use walkdir::WalkDir;

use self::{config::Config, decrypter::copy_with_decryption, system_json::SystemJson};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    WalkDir::new(&config.game_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .try_for_each(|file| copy_with_decryption(&config, &file))?;

    remove_encryption_info(&config.dest_root)?;

    Ok(())
}

fn remove_encryption_info(dest_root: &Path) -> Result<()> {
    let SystemJson { path, mut content, .. } = SystemJson::new(dest_root)?;

    content.remove("hasEncryptedImages");
    content.remove("hasEncryptedAudio");
    content.remove("encryptionKey");

    fs::write(&path, serde_json::to_string(&content)?)?;

    Ok(())
}
