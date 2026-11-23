use numerical_lang::{
    lexer::{Lexer, token::Token},
    parser::{Parser, expr::Expression},
};

pub fn assert_lexer_unspanned(input: &str, expected: Vec<Token>) {
    let mut lexer = Lexer::new(input);
    let actual = match lexer.lex() {
        Ok(a) => a.iter().map(|t| t.value.clone()).collect::<Vec<Token>>(),
        Err(e) => panic!(
            "assert_lexer failed, the lexing failed with errors: {:#?}",
            e
        ),
    };

    assert_eq!(actual, expected, "Lexer failed on input: {}", input);
}

pub fn assert_pratt_parser(input: &str, expected: Expression) {
    // TODO: change to statement rather than just expression
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.lex() {
        Ok(a) => a,
        Err(e) => panic!(
            "assert_lexer failed, the lexing failed with errors: {:#?}",
            e
        ),
    };

    let mut parser = Parser::new(tokens);
    let expr = match parser.expr() {
        Ok(e) => e,
        Err(e) => panic!(
            "assert_pratt_parser failed, the lexing failed with errors: {:#?}",
            e
        ),
    };

    assert_eq!(expected, expr)
}
