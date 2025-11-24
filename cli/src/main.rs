mod args;

use std::{env, process};

use rpgmvmz_decrypter::filesystem;

use self::args::Args;

fn main() {
    let args = Args::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = filesystem::decrypt(&args.game_dir) {
        eprintln!("{e}");
        process::exit(1);
    }
}
