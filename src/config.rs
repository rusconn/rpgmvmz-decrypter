use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

use crate::system_json::SystemJson;

#[derive(Debug)]
pub struct Config {
    pub(crate) game_dir: PathBuf,
    pub(crate) dest_root: PathBuf,
    pub(crate) masks: Vec<u8>,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self> {
        args.next();

        let game_dir = args
            .next()
            .map(PathBuf::from)
            .context("USAGE: rpgmvmz-decrypter <game_dir>")?;

        let system_json = SystemJson::new(&game_dir)?;

        let masks = hex::decode(&system_json.encryption_key)
            .map_err(|e| anyhow!("Invalid encryptionKey: {e}"))?;

        let dest_root = Self::add_suffix(&game_dir, "_decrypted") //
            .context("Something went wrong.")?;

        Ok(Self { game_dir, dest_root, masks })
    }

    fn add_suffix(path: &Path, suffix: &str) -> Option<PathBuf> {
        let parent = path.parent().unwrap_or(Path::new(""));
        let file_name = path.file_name()?.to_str()?;
        Some(parent.join(format!("{file_name}{suffix}")))
    }
}
