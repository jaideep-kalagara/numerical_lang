use std::{fs, path::PathBuf, process};

use pico_args::Arguments;

use crate::{lexer::token::Token, span::Spanned};

pub struct Cli {
    pub output_tokens: bool,
    pub filepath: String,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        let mut args = Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            Cli::help_msg();
        }

        Cli {
            output_tokens: args.contains(["-t", "--output-tokens"]),
            filepath: match args.free_from_str() {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Required argument <FILEPATH> missing");
                    process::exit(1)
                }
            },
        }
    }

    pub fn help_msg() {
        println!("Usage: nlc [-t, --output-tokens] <FILEPATH>");
    }

    pub fn output_tokens_file(&self, tokens: Vec<Spanned<Token>>) {
        let mut tokens_string = String::new();
        for token in tokens {
            tokens_string.push_str(&token.to_string());
            tokens_string.push('\n');
        }

        let tokens_filepath = format!("{}.tokens", PathBuf::from(&self.filepath).to_string_lossy());
        match fs::write(tokens_filepath, tokens_string) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to write file, {}", e);
                process::exit(1);
            }
        };
    }

    pub fn read_input(&self) -> String {
        match fs::read_to_string(&self.filepath) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to read file, {}", e);
                process::exit(1);
            }
        }
    }
}
