mod args;
mod error;

use std::{env, path::Path, process};

use rpgmvmz_decrypter::filesystem;

use self::{args::Args, error::AppError};

fn main() {
    let args = Args::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = run(&args.game_dir) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(game_dir: &Path) -> Result<(), AppError> {
    filesystem::decrypt(game_dir)?;
    Ok(())
}
