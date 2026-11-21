use numerical_lang::lexer::Lexer;
use pico_args::Arguments;
use std::{fs, path::PathBuf, process};

pub struct ProgramArgs {
    output_tokens: bool,
    filepath: String,
}

fn main() {
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        println!("Usage: nlc [-t, --output-tokens] <FILEPATH>");
    }

    let p_args = ProgramArgs {
        output_tokens: args.contains(["-t", "--output-tokens"]),
        filepath: match args.free_from_str() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Required argument <FILEPATH> missing");
                process::exit(1);
            }
        },
    };

    let file = match fs::read_to_string(&p_args.filepath) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to read file, {}", e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(file.clone());

    // Output tokens file
    if p_args.output_tokens {
        let tokens = match lexer.lex() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("The lexing process encountered an error!\n{}", e);
                process::exit(1);
            }
        };
        let mut tokens_string = String::new();
        for token in tokens {
            tokens_string.push_str(&token.to_string());
            tokens_string.push('\n');
        }

        let tokens_filepath = format!(
            "{}.tokens",
            PathBuf::from(p_args.filepath).to_string_lossy()
        );
        match fs::write(tokens_filepath, tokens_string) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to write file, {}", e);
                process::exit(1);
            }
        };
    }

    let remaining = args.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }
}
