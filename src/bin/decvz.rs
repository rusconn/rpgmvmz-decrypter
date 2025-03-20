use std::{env, process};

use rpgmvmz_decrypter::{config::Config, run};

fn main() {
    let config = Config::new(&mut env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
