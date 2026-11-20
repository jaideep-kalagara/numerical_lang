mod common;
use numerical_lang::{
    keyword::Keyword,
    token::{Token, TokenType, TokenValue},
};

use crate::common::assert_lexer;

#[test]
fn basic_lexer_test() {
    assert_lexer(
        "+-*/",
        vec![
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Minus, None),
            Token::new(TokenType::Multiply, None),
            Token::new(TokenType::Divide, None),
        ],
    );
}

#[test]
fn parentheses_lexer_test() {
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
fn identifier_lexer_test() {
    assert_lexer(
        "hello",
        vec![Token::new(
            TokenType::Identifier,
            Some(TokenValue::Identifier("hello".to_string())),
        )],
    );
}

#[test]
fn keyword_lexer_test() {
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
}
