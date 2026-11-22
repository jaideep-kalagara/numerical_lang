use std::{
    iter::Peekable,
    str::{CharIndices, FromStr},
};

use strum::VariantNames;

use crate::{
    lexer::{error::LexerError, token::Token},
    span::{Span, Spanned},
};

pub mod error;
pub mod keyword;
pub mod token;

pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
    position: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.char_indices().peekable(),
            position: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Spanned<Token>>, LexerError> {
        let mut tokens: Vec<Spanned<Token>> = Vec::new();

        while let Some(c) = self.peek() {
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

        tokens.push(Spanned::new(Token::Eof, self.position, self.position));

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

    pub fn get_string(&mut self, start: usize) -> Result<String, LexerError> {
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

                // TODO: Decide if this is necessary or not.
                // '\\' => match self.advance() {
                //     Some('n') => buffer.push_str(r"\n"),
                //     Some('t') => buffer.push_str(r"\t"),
                //     Some('\\') => buffer.push_str(r"\"),
                //     Some('"') => buffer.push_str("\""),

                //     None => {
                //         return Err(LexerError::UnterminatedStringLiteral { line: self.line });
                //     }

                //     Some(c) => {
                //         return Err(LexerError::UnknownEscapeCodeCharacter {
                //             line: self.line,
                //             char: c,
                //         });
                //     }
                // },
                '\n' => {
                    return Err(LexerError::UnterminatedStringLiteral {
                        span: Span::new(start, self.position),
                    });
                }

                c => buffer.push(c),
            }
        }

        if !terminated {
            return Err(LexerError::UnterminatedStringLiteral {
                span: Span::new(start, self.position),
            });
        }
        return Ok(buffer);
    }

    pub fn get_number(&mut self, start: usize) -> Result<Spanned<Token>, LexerError> {
        let mut buffer = String::new();

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                buffer.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let span = Span::new(start, self.position);
        if buffer.contains('.') {
            let float: f64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                span,
                number: buffer.clone(),
            })?;

            Ok(Spanned {
                value: Token::Float(float),
                span,
            })
        } else {
            let int: i64 = buffer.parse().map_err(|_| LexerError::InvalidNumber {
                span,
                number: buffer.clone(),
            })?;

            Ok(Spanned {
                value: Token::Integer(int),
                span,
            })
        }
    }

    pub fn token(&mut self) -> Result<Spanned<Token>, LexerError> {
        let start = self.position;
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

            Some('"') => Token::String(self.get_string(start)?),

            Some(c) if c.is_alphabetic() || c == '_' => {
                let string = self.get_ident();

                let span = Span::new(start, self.position);
                if keyword::Keyword::VARIANTS.contains(&string.as_str()) {
                    return Ok(Spanned {
                        value: Token::Keyword(keyword::Keyword::from_str(&string).unwrap()),
                        span,
                    });
                } else {
                    return Ok(Spanned {
                        value: Token::Identifier(string),
                        span,
                    });
                }
            }

            Some(c) if c.is_numeric() => {
                return self.get_number(start);
            }

            Some(c) => {
                self.advance();

                return Err(LexerError::InvalidCharacter {
                    span: Span::new(start, self.position),
                    char: c,
                });
            }

            None => Token::Eof,
        };

        // Consume the token's first character.
        // For two-character operators, the second character
        // was already consumed above.
        self.advance();

        Ok(Spanned::new(token, start, self.position))
    }

    pub fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some((index, c)) => {
                self.position = index + c.len_utf8();

                if c == '\n' {
                    self.line += 1;
                }

                Some(c)
            }
            None => None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input.peek().map(|(_, c)| *c)
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.input.clone().nth(n).map(|(_, c)| c)
    }
}
