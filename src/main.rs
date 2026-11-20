use std::{error::Error, fs};

use numerical_lang::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let to_be_lexed = fs::read_to_string("examples/lexer_test.nl")?;

    let mut lexer = Lexer::new(to_be_lexed);
    let tokens = lexer.lex();
    for token in tokens {
        println!("{}", token)
    }
    Ok(())
}
