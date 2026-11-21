use numerical_lang::lexer::{Lexer, token::Token};

pub fn assert_lexer(input: &str, expected: Vec<Token>) {
    let mut lexer = Lexer::new(input);
    let actual = match lexer.lex() {
        Ok(a) => a,
        Err(e) => panic!("assert_lexer failed, the lexing failed with error: {}", e),
    };

    assert_eq!(actual, expected, "Lexer failed on input: {}", input);
}
