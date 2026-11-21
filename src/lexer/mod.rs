use std::str::FromStr;

use strum::VariantNames;

use crate::lexer::{error::LexerError, token::Token};

pub mod error;
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

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
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

                continue;
            }

            tokens.push(self.token()?);
        }
        Ok(tokens)
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

    pub fn get_string(&mut self) -> Result<String, LexerError> {
        // advance characters until is isn't a alphabetic char or an underscore
        let mut buffer = String::new();

        // consume the first quote
        self.advance();
        let mut terminated = false;

        while let Some(c) = self.current_char {
            if c == '"' {
                terminated = true;
                break;
            }

            if c == '\n' {
                return Err(LexerError::UnterminatedStringLiteral { line: self.line });
            }

            if c == '\\' {
                // TODO: Handle escape sequences in strings
                unimplemented!()
            }

            buffer.push(c);
            self.advance();
        }

        if !terminated {
            return Err(LexerError::UnterminatedStringLiteral { line: self.line });
        }
        Ok(buffer)
    }

    pub fn get_number(&mut self) -> Result<Token, LexerError> {
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
            let float: f64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                line: self.line,
                number: buffer,
            })?;
            Ok(Token::Float(float))
        } else {
            let int: i64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                line: self.line,
                number: buffer,
            })?;
            Ok(Token::Integer(int))
        }
    }

    /// gets the next token
    pub fn token(&mut self) -> Result<Token, LexerError> {
        let token = match self.current_char {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Divide,
            Some('*') => Token::Multiply,
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::DoubleEqual
                } else {
                    Token::Equal
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GreaterThanEqualTo
                } else {
                    Token::GreaterThan
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
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
            Some('\"') => Token::String(self.get_string()?),
            None => Token::Eof,

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

            _ => {
                if self.current_char.is_some() {
                    return Err(LexerError::InvalidCharacter {
                        line: self.line,
                        char: self.current_char.unwrap(),
                    });
                } else {
                    return Err(LexerError::InvalidCharacterWithUnknownInfo { line: self.line });
                }
            }
        };

        self.advance();
        Ok(token)
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
