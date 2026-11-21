mod common;

use numerical_lang::lexer::{Lexer, keyword::Keyword, token::Token};

use crate::common::assert_lexer;

#[test]
fn basic_test() {
    assert_lexer(
        "+-*/,",
        vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Comma,
        ],
    );
}

#[test]
fn parentheses_test() {
    assert_lexer("()", vec![Token::LParen, Token::RParen]);

    assert_lexer("{}", vec![Token::LCurly, Token::RCurly]);
}

#[test]
fn inequality_test() {
    assert_lexer("<>", vec![Token::LesserThan, Token::GreaterThan]);

    assert_lexer(
        "== >= <=",
        vec![
            Token::DoubleEqual,
            Token::GreaterThanEqualTo,
            Token::LesserThanEqualTo,
        ],
    );
}

#[test]
fn identifier_test() {
    assert_lexer("hello", vec![Token::Identifier("hello".to_string())]);
}

#[test]
fn number_test() {
    assert_lexer("10", vec![Token::Integer(10)]);

    assert_lexer("10.5", vec![Token::Float(10.5)]);
}

#[test]
fn keyword_test() {
    assert_lexer("if", vec![Token::Keyword(Keyword::If)]);

    assert_lexer("let", vec![Token::Keyword(Keyword::Let)]);

    assert_lexer("func", vec![Token::Keyword(Keyword::Func)]);

    assert_lexer("return", vec![Token::Keyword(Keyword::Return)]);
}

#[test]
fn comment_test() {
    assert_lexer(
        "; Test comment\nfunc ; Test comment",
        vec![Token::Keyword(Keyword::Func)],
    );
}

#[test]
#[should_panic(expected = "Invalid character: '|', line: 2")]
fn error_test() {
    let mut lexer = Lexer::new("hi+-\n |".to_string());
    match lexer.lex() {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e)
        }
    };
}
