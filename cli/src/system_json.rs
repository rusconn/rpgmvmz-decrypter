use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};

pub(crate) struct SystemJson {
    pub(crate) path: PathBuf,
    pub(crate) encryption_key: String,
    pub(crate) content: Map<String, Value>,
}

impl SystemJson {
    pub(crate) fn read(game_dir: &Path) -> Result<SystemJson> {
        if !game_dir.exists() {
            bail!("{} not exists.", game_dir.display());
        }
        if !game_dir.is_dir() {
            bail!("{} is not a directory.", game_dir.display());
        }

        let mv_path = game_dir.join("www").join("data").join("System.json");
        let mz_path = game_dir.join("data").join("System.json");

        let system_json_path = [mv_path, mz_path]
            .into_iter()
            .find(|p| p.exists())
            .context("Can't find the System.json.")?;

        let read = fs::read_to_string(&system_json_path)?;

        let Ok(content) = serde_json::from_str::<Map<String, Value>>(&read) else {
            bail!("Invalid System.json.");
        };
        let Some(encryption_key) = content.get("encryptionKey") else {
            bail!("Can't find the encryptionKey in the System.json.");
        };
        let Value::String(encryption_key) = encryption_key else {
            bail!("Invalid encryptionKey.");
        };

        Ok(Self {
            path: system_json_path,
            encryption_key: encryption_key.to_owned(),
            content,
        })
    }
}
