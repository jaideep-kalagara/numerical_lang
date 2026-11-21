use std::str::FromStr;

use strum::VariantNames;

use crate::lexer::token::{Token, TokenType, TokenValue};

pub mod keyword;
pub mod token;

pub struct Lexer {
    input: String,
    position: usize,
    lookahead_position: usize,
    line: usize,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.clone(),
            position: 0,
            lookahead_position: 0,
            line: 1,
            current_char: input.chars().nth(0),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.advance();
                continue;
            }

            if c.is_whitespace() {
                self.advance();
                continue;
            }

            // Handle comments
            if c == ';' {
                while let Some(c) = self.current_char {
                    if c == '\n' {
                        self.line += 1;
                        self.advance();
                        break;
                    }
                    self.advance();
                }
            }

            tokens.push(self.token());
        }
        tokens
    }

    pub fn get_ident(&mut self) -> String {
        // advance characters until is isn't a alphabetic char or an underscore
        let mut buffer = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphabetic() || c == '_' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }

        buffer
    }

    pub fn get_number(&mut self) -> TokenValue {
        let mut buffer = String::new();

        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if buffer.contains(".") {
            let float: f64 = buffer
                .parse()
                .unwrap_or_else(|_| panic!("Failed to lex number, {}", buffer));
            TokenValue::Float(float)
        } else {
            let int: i64 = buffer
                .parse()
                .unwrap_or_else(|_| panic!("Failed to lex number, {}", buffer));
            TokenValue::Integer(int)
        }
    }

    /// gets the next token
    pub fn token(&mut self) -> Token {
        let mut value: Option<TokenValue> = None;

        let token_type = match self.current_char {
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some('/') => TokenType::Divide,
            Some('*') => TokenType::Multiply,
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::DoubleEqual
                } else {
                    TokenType::Equal
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::GreaterThanEqualTo
                } else {
                    TokenType::GreaterThan
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenType::LesserThanEqualTo
                } else {
                    TokenType::LesserThan
                }
            }
            Some('(') => TokenType::LParen,
            Some(')') => TokenType::RParen,
            Some('{') => TokenType::LCurly,
            Some('}') => TokenType::RCurly,
            Some(':') => TokenType::Colon,
            Some(',') => TokenType::Comma,
            None => TokenType::Eof,

            Some(c) if c.is_alphabetic() || c == '_' => {
                let string = self.get_ident();

                if keyword::Keyword::VARIANTS.contains(&string.as_str()) {
                    value = Some(TokenValue::Keyword(
                        keyword::Keyword::from_str(&string).unwrap(),
                    ));

                    return Token::new(TokenType::Keyword, value); // avoid the second advance
                } else {
                    value = Some(TokenValue::Identifier(string));

                    return Token::new(TokenType::Identifier, value); // avoid the second advance
                }
            }

            // TODO: decide if numbers can start with decimal point
            Some(c) if c.is_numeric() => {
                value = Some(self.get_number());
                if value.clone().unwrap().is_float() {
                    // TODO: no unwarp
                    return Token::new(TokenType::Float, value);
                } else {
                    return Token::new(TokenType::Integer, value);
                }
            }

            _ => {
                if self.current_char.is_some() {
                    panic!(
                        "Error on line {}, with character: {}",
                        self.line,
                        self.current_char.unwrap()
                    );
                } else {
                    panic!("Error on line {}", self.line);
                }
            }
        };

        self.advance();
        Token::new(token_type, value)
    }

    pub fn advance(&mut self) {
        self.position += 1;
        self.lookahead_position = self.position + 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input.chars().nth(self.lookahead_position)
    }
}
