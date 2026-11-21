use crate::lexer::keyword;

#[derive(Debug, Clone, PartialEq, strum::EnumIs)]
#[strum(serialize_all = "snake_case")]
pub enum Token {
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    DoubleEqual,
    GreaterThan,
    GreaterThanEqualTo,
    LesserThan,
    LesserThanEqualTo,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Colon,
    Comma,
    Eof,

    // Valued
    Integer(i64),
    Float(f64),
    Keyword(keyword::Keyword),
    Identifier(String),
}

use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(value) => write!(f, "Token(Integer, {value})"),
            Token::Float(value) => write!(f, "Token(Float, {value})"),
            Token::Keyword(value) => write!(f, "Token(Keyword, {value})"),
            Token::Identifier(value) => write!(f, "Token(Identifier, {value})"),
            token => write!(f, "Token({token:?})"),
        }
    }
}
