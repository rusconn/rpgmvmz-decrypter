use std::{env, process};

use rpgmvmz_decrypter::{Config, run};

#[tokio::main]
async fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = run(config).await {
        eprintln!("{e}");
        process::exit(1);
    }
}
