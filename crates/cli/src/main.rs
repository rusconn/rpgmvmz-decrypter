use std::{env, process};

use rpgmvmz_decrypter::decrypt_game;

use rpgmvmz_decrypter_cli::{Args, AsDisplay};

fn main() {
    let args = Args::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = decrypt_game(&args.game_dir) {
        eprintln!("{}", e.as_display());
        process::exit(1);
    }
}
