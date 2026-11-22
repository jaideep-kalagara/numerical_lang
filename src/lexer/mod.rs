use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use strum::VariantNames;

use crate::lexer::{error::LexerError, token::Token};

pub mod error;
pub mod keyword;
pub mod token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.peek() {
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
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }

                    self.advance();
                }

                continue;
            }

            tokens.push(self.token()?);
        }

        tokens.push(Token::Eof);

        Ok(tokens)
    }

    pub fn get_ident(&mut self) -> String {
        let mut buffer = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }

        buffer
    }

    pub fn get_string(&mut self) -> Result<String, LexerError> {
        let mut buffer = String::new();

        // Consume opening quote
        self.advance();

        let mut terminated = false;

        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    terminated = true;
                    break;
                }

                '\\' => match self.advance() {
                    Some('n') => buffer.push('\n'),
                    Some('t') => buffer.push('\t'),
                    Some('\\') => buffer.push('\\'),
                    Some('"') => buffer.push('"'),

                    None => {
                        return Err(LexerError::UnterminatedStringLiteral { line: self.line });
                    }

                    Some(c) => {
                        return Err(LexerError::UnknownEscapeCodeCharacter {
                            line: self.line,
                            char: c,
                        });
                    }
                },

                '\n' => {
                    return Err(LexerError::UnterminatedStringLiteral { line: self.line });
                }

                c => buffer.push(c),
            }
        }

        if !terminated {
            return Err(LexerError::UnterminatedStringLiteral { line: self.line });
        }

        return Ok(buffer);
    }

    pub fn get_number(&mut self) -> Result<Token, LexerError> {
        let mut buffer = String::new();

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if buffer.contains('.') {
            let float: f64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                line: self.line,
                number: buffer.clone(),
            })?;

            Ok(Token::Float(float))
        } else {
            let int: i64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                line: self.line,
                number: buffer.clone(),
            })?;

            Ok(Token::Integer(int))
        }
    }

    pub fn token(&mut self) -> Result<Token, LexerError> {
        let token = match self.peek() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Divide,
            Some('*') => Token::Multiply,

            Some('=') => {
                if self.peek_nth(1) == Some('=') {
                    self.advance();
                    Token::DoubleEqual
                } else {
                    Token::Equal
                }
            }

            Some('>') => {
                if self.peek_nth(1) == Some('=') {
                    self.advance();
                    Token::GreaterThanEqualTo
                } else {
                    Token::GreaterThan
                }
            }

            Some('<') => {
                if self.peek_nth(1) == Some('=') {
                    self.advance();
                    Token::LesserThanEqualTo
                } else {
                    Token::LesserThan
                }
            }

            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LCurly,
            Some('}') => Token::RCurly,
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,

            Some('"') => Token::String(self.get_string()?),

            Some(c) if c.is_alphabetic() || c == '_' => {
                let string = self.get_ident();

                if keyword::Keyword::VARIANTS.contains(&string.as_str()) {
                    return Ok(Token::Keyword(keyword::Keyword::from_str(&string).unwrap()));
                } else {
                    return Ok(Token::Identifier(string));
                }
            }

            Some(c) if c.is_numeric() => {
                return self.get_number();
            }

            Some(c) => {
                return Err(LexerError::InvalidCharacter {
                    line: self.line,
                    char: c,
                });
            }

            None => return Ok(Token::Eof),
        };

        // Consume the token's first character.
        // For two-character operators, the second character
        // was already consumed above.
        self.advance();

        Ok(token)
    }

    pub fn advance(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.input.clone().nth(n)
    }
}
