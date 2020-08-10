use std::env;
use std::process;

use learning_rust_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching in {} for {}", config.filename, config.query);

    if let Err(e) = learning_rust_minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

