use clap::Parser;
use numerical_lang::lexer::Lexer;
use std::{error::Error, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filepath: String,
    #[arg(short = 't', long, action=clap::ArgAction::SetTrue)]
    output_tokens: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file = fs::read_to_string(&args.filepath)?;

    let mut lexer = Lexer::new(file.clone());

    if args.output_tokens {
        let tokens = lexer.lex();
        let mut tokens_string = String::new();
        for token in tokens {
            tokens_string.push_str(&token.to_string());
            tokens_string.push('\n');
        }

        let tokens_filepath = format!(
            "{}.tokens",
            PathBuf::from(args.filepath)
                .with_extension("")
                .to_string_lossy()
        );
        fs::write(tokens_filepath, tokens_string)?;
    }

    Ok(())
}
