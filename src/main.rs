use std::env;
use std::process;

use learning_rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching in {} for {}", config.filename, config.query);

    if let Err(e) = learning_rust_minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

