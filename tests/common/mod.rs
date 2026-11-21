use numerical_lang::lexer::{Lexer, token::Token};

pub fn assert_lexer(input: &str, expected: Vec<Token>) {
    let mut lexer = Lexer::new(input.to_string());
    let actual = lexer.lex();

    assert_eq!(actual, expected, "Lexer failed on input: {}", input);
}
