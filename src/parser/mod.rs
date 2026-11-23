use std::iter::Peekable;
use std::vec;

use crate::{
    lexer::token::Token,
    parser::{binary_op::BinaryOp, error::ParserError, expr::Expression},
    span::Spanned,
};

pub mod binary_op;
pub mod error;
pub mod expr;

pub struct Parser {
    input: Peekable<vec::IntoIter<Spanned<Token>>>,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Self {
            input: tokens.into_iter().peekable(),
        }
    }

    pub fn expr(&mut self) -> Result<Expression, ParserError> {
        let expr = self.expr_bp(0)?;

        match self.input.peek() {
            Some(Spanned {
                value: Token::Eof, ..
            }) => Ok(expr),

            Some(token) => Err(ParserError::InvalidToken {
                token: token.value.clone(),
                span: token.span,
            }),

            None => unreachable!("Lexer should always emit EOF"),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.advance() {
            Some(Spanned {
                value: Token::Integer(v),
                ..
            }) => Ok(Expression::IntValue(v)),

            Some(Spanned {
                value: Token::Float(v),
                ..
            }) => Ok(Expression::FloatValue(v)),

            // Parenthesized expression.
            Some(Spanned {
                value: Token::LParen,
                ..
            }) => {
                let expr = self.expr_bp(0)?;

                self.expect(Token::RParen)?;

                Ok(expr)
            }

            // Unary expressions.
            Some(token) if matches!(token.value, Token::Plus | Token::Minus) => {
                let op =
                    BinaryOp::from_token(&token).expect("Plus and Minus must be binary operators");

                let ((), r_bp) = op.prefix_binding_power(token.span)?;

                let rhs = self.expr_bp(r_bp)?;

                Ok(Expression::Unary {
                    operator: op,
                    rhs: Box::new(rhs),
                })
            }

            Some(Spanned {
                value: Token::Eof,
                span,
            }) => Err(ParserError::UnexpectedEof { span }),

            Some(token) => Err(ParserError::InvalidToken {
                token: token.value,
                span: token.span,
            }),

            None => unreachable!("Lexer should always emit EOF"),
        }
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.input.peek() {
                // Expression boundaries.
                Some(Spanned {
                    value: Token::Eof | Token::RParen,
                    ..
                }) => break,

                // Check whether the next token is an infix operator.
                Some(token) => match BinaryOp::from_token(token) {
                    Some(op) => op,
                    None => break,
                },

                None => unreachable!("Lexer should always emit EOF"),
            };

            let (l_bp, r_bp) = op.infix_binding_power();

            // Don't consume operators belonging to the caller.
            if l_bp < min_bp {
                break;
            }

            // Consume the operator.
            self.advance();

            let rhs = self.expr_bp(r_bp)?;

            lhs = Expression::Binary {
                lhs: Box::new(lhs),
                operator: op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    pub fn advance(&mut self) -> Option<Spanned<Token>> {
        self.input.next()
    }

    pub fn expect(&mut self, expected: Token) -> Result<Spanned<Token>, ParserError> {
        match self.advance() {
            Some(Spanned {
                value: Token::Eof,
                span,
            }) => Err(ParserError::UnexpectedEof { span }),

            Some(token) if token.value == expected => Ok(token),

            Some(token) => Err(ParserError::ExpectedToken {
                expected,
                got: token.value,
                span: token.span,
            }),

            None => unreachable!("Lexer should always emit EOF"),
        }
    }

    pub fn check(&mut self, expected: &Token) -> bool {
        matches!(
            self.input.peek(),
            Some(token) if &token.value == expected
        )
    }
}
