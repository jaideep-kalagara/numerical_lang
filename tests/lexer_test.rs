mod common;
use numerical_lang::{
    keyword::Keyword,
    lexer::Lexer,
    token::{Token, TokenType, TokenValue},
};

use crate::common::assert_lexer;

#[test]
fn basic_test() {
    assert_lexer(
        "+-*/,",
        vec![
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Minus, None),
            Token::new(TokenType::Multiply, None),
            Token::new(TokenType::Divide, None),
            Token::new(TokenType::Comma, None),
        ],
    );
}

#[test]
fn parentheses_test() {
    assert_lexer(
        "()",
        vec![
            Token::new(TokenType::LParen, None),
            Token::new(TokenType::RParen, None),
        ],
    );
    assert_lexer(
        "{}",
        vec![
            Token::new(TokenType::LCurly, None),
            Token::new(TokenType::RCurly, None),
        ],
    );
}

#[test]
fn inequality_test() {
    assert_lexer(
        "<>",
        vec![
            Token::new(TokenType::LesserThan, None),
            Token::new(TokenType::GreaterThan, None),
        ],
    );
    assert_lexer(
        "== >= <=",
        vec![
            Token::new(TokenType::DoubleEqual, None),
            Token::new(TokenType::GreaterThanEqualTo, None),
            Token::new(TokenType::LesserThanEqualTo, None),
        ],
    );
}

#[test]
fn identifier_test() {
    assert_lexer(
        "hello",
        vec![Token::new(
            TokenType::Identifier,
            Some(TokenValue::Identifier("hello".to_string())),
        )],
    );
}

#[test]
fn number_test() {
    assert_lexer(
        "10",
        vec![Token::new(
            TokenType::Integer,
            Some(TokenValue::Integer(10)),
        )],
    );
    assert_lexer(
        "10.5",
        vec![Token::new(TokenType::Float, Some(TokenValue::Float(10.5)))],
    )
}

#[test]
fn keyword_test() {
    assert_lexer(
        "if",
        vec![Token::new(
            TokenType::Keyword,
            Some(TokenValue::Keyword(Keyword::If)),
        )],
    );
    assert_lexer(
        "let",
        vec![Token::new(
            TokenType::Keyword,
            Some(TokenValue::Keyword(Keyword::Let)),
        )],
    );
    assert_lexer(
        "func",
        vec![Token::new(
            TokenType::Keyword,
            Some(TokenValue::Keyword(Keyword::Func)),
        )],
    );
    assert_lexer(
        "return",
        vec![Token::new(
            TokenType::Keyword,
            Some(TokenValue::Keyword(Keyword::Return)),
        )],
    );
}

#[test]
fn comment_test() {
    assert_lexer("; Test comment", vec![Token::new(TokenType::Eof, None)]);
}

#[test]
#[should_panic(expected = "Error on line 2")]
fn error_test() {
    let mut lexer = Lexer::new("hi+-\n |".to_string());
    lexer.lex();
}
