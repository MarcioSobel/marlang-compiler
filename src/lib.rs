use lexer::Lexer;
use std::{error::Error, fs};

pub mod lexer;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("No file provided");
        }

        let file_path = &args[1];
        Ok(Config {
            file_path: file_path.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut lexer = Lexer::new(&contents);
    lexer.tokenize();

    Ok(())
}
