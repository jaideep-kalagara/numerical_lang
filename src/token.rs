use std::fmt::Display;

use crate::keyword::{self};

#[derive(Debug, Clone, Copy, PartialEq, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TokenType {
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
    Integer,
    Float,
    Keyword,
    Identifier,
}

#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumIs)]
pub enum TokenValue {
    #[strum(to_string = "{0}")]
    Integer(i64),
    #[strum(to_string = "{0}")]
    Float(f64),
    #[strum(to_string = "{0}")]
    Keyword(keyword::Keyword),
    #[strum(to_string = "{0}")]
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<TokenValue>,
    // TODO: add more error deducing fields
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.is_some() {
            write!(
                f,
                "Token(Type: {}, Value: {})",
                self.token_type,
                self.value.clone().unwrap()
            )?;
        } else {
            write!(f, "Token(Type: {})", self.token_type)?;
        }
        Ok(())
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<TokenValue>) -> Self {
        Self { token_type, value }
    }
}
