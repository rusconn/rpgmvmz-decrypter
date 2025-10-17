use std::{env, path::PathBuf};

use anyhow::{Context, Result};

pub(crate) struct Config {
    pub(crate) game_dir: PathBuf,
}

impl Config {
    pub(crate) fn parse(mut args: env::Args) -> Result<Self> {
        args.next();

        let game_dir = args
            .next()
            .map(PathBuf::from)
            .context("USAGE: decvz <game_dir>")?;

        Ok(Self { game_dir })
    }
}
