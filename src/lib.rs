mod config;
mod decrypter;
mod system_json;

use std::{fs, path::Path};

use anyhow::Result;
use rayon::prelude::*;
use walkdir::WalkDir;

pub use config::Config;
use decrypter::copy_with_decryption;
use system_json::SystemJson;

pub fn run(config: Config) -> Result<()> {
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
