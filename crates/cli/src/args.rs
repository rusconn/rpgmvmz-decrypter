use std::{env, path::PathBuf};

pub struct Args {
    pub game_dir: PathBuf,
}

impl Args {
    pub fn parse(mut args: env::Args) -> Result<Self, String> {
        args.next();

        let game_dir = args
            .next()
            .map(PathBuf::from)
            .ok_or("USAGE: decvz <game_dir>")?;

        Ok(Self { game_dir })
    }
}
