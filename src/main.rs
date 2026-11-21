use clap::Parser;
use numerical_lang::lexer::Lexer;
use std::{fs, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filepath: String,
    #[arg(short = 't', long, action=clap::ArgAction::SetTrue)]
    output_tokens: bool,
}

fn main() {
    let args = Args::parse();

    let file = match fs::read_to_string(&args.filepath) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to read file, {}", e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(file.clone());

    if args.output_tokens {
        let tokens = match lexer.lex() {
            Ok(t) => t,
            Err(e) => {
                println!("The lexing process encountered an error! {}", e);
                process::exit(1);
            }
        };
        let mut tokens_string = String::new();
        for token in tokens {
            tokens_string.push_str(&token.to_string());
            tokens_string.push('\n');
        }

        let tokens_filepath = format!("{}.tokens", PathBuf::from(args.filepath).to_string_lossy());
        match fs::write(tokens_filepath, tokens_string) {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to write file, {}", e);
                process::exit(1);
            }
        };
    }
}
