mod common;

use numerical_lang::lexer::{Lexer, keyword::Keyword, token::Token};

use crate::common::assert_lexer_unspanned;

#[test]
fn basic_test() {
    assert_lexer_unspanned(
        "+-*/,",
        vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Comma,
            Token::Eof,
        ],
    );
}

#[test]
fn parentheses_test() {
    assert_lexer_unspanned("()", vec![Token::LParen, Token::RParen, Token::Eof]);

    assert_lexer_unspanned("{}", vec![Token::LCurly, Token::RCurly, Token::Eof]);
}

#[test]
fn inequality_test() {
    assert_lexer_unspanned(
        "<>",
        vec![Token::LesserThan, Token::GreaterThan, Token::Eof],
    );

    assert_lexer_unspanned(
        "== >= <=",
        vec![
            Token::DoubleEqual,
            Token::GreaterThanEqualTo,
            Token::LesserThanEqualTo,
            Token::Eof,
        ],
    );
}

#[test]
fn identifier_test() {
    assert_lexer_unspanned(
        "hello",
        vec![Token::Identifier("hello".to_string()), Token::Eof],
    );
}

#[test]
fn number_test() {
    assert_lexer_unspanned("10", vec![Token::Integer(10), Token::Eof]);

    assert_lexer_unspanned("10.5", vec![Token::Float(10.5), Token::Eof]);
}

#[test]
fn keyword_test() {
    assert_lexer_unspanned("if", vec![Token::Keyword(Keyword::If), Token::Eof]);

    assert_lexer_unspanned("let", vec![Token::Keyword(Keyword::Let), Token::Eof]);

    assert_lexer_unspanned("func", vec![Token::Keyword(Keyword::Func), Token::Eof]);

    assert_lexer_unspanned("return", vec![Token::Keyword(Keyword::Return), Token::Eof]);
}

#[test]
fn comment_test() {
    assert_lexer_unspanned(
        "; Test comment\nfunc ; Test comment",
        vec![Token::Keyword(Keyword::Func), Token::Eof],
    );
}

#[test]
fn string_test() {
    assert_lexer_unspanned(
        "\"Test\\n\"",
        vec![Token::String("Test\\n".to_string()), Token::Eof],
    );
}
#[test]
#[should_panic(expected = "Unterminated string literal")]
fn string_error_test() {
    let mut lexer = Lexer::new("\"Test");
    match lexer.lex() {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e)
        }
    };
}

#[test]
#[should_panic(expected = "Invalid character: '|'")]
fn invalid_character_error_test() {
    let mut lexer = Lexer::new("hi+-\n |");
    match lexer.lex() {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e)
        }
    };
}
