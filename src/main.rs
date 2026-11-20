use std::{error::Error, fs};

use numerical_lang::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let to_be_lexed = fs::read_to_string("examples/lexer_test.nl")?;

    let mut lexer = Lexer::new(to_be_lexed);
    let tokens = lexer.lex();
    let mut tokens_string = String::new();
    for token in tokens {
        tokens_string.push_str(&token.to_string());
        tokens_string.push_str("\n");
    }

    fs::write("tokens", tokens_string)?;
    Ok(())
}
