use marlang::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Could not parse arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = marlang::run(config) {
        eprintln!("Compiler error: {error}");
        process::exit(1);
    }
}
