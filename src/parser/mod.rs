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

    /// Consumes and gets an expression.
    pub fn expr(&mut self) -> Result<Expression, ParserError> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expression, ParserError> {
        // Parse the left-hand side.
        let mut lhs = match self.input.next() {
            Some(Spanned {
                value: Token::Integer(v),
                ..
            }) => Expression::IntValue(v),

            Some(Spanned {
                value: Token::Float(v),
                ..
            }) => Expression::FloatValue(v),

            Some(Spanned {
                value: Token::Eof,
                span,
            }) => {
                return Err(ParserError::UnexpectedEof { span });
            }

            Some(token) => {
                return Err(ParserError::InvalidToken {
                    token: token.value,
                    span: token.span,
                });
            }

            None => unreachable!("Lexer should always emit EOF"),
        };

        // Parse infix expressions.
        loop {
            let op = match self.input.peek() {
                Some(Spanned {
                    value: Token::Eof, ..
                }) => break,

                Some(token) => BinaryOp::try_from(token)?,

                None => unreachable!("Lexer should always emit EOF"),
            };

            let (l_bp, r_bp) = op.infix_binding_power();

            // Don't consume operators belonging to the caller.
            if l_bp < min_bp {
                break;
            }

            // Consume the operator.
            self.input.next();

            let rhs = self.expr_bp(r_bp)?;

            lhs = Expression::Binary {
                lhs: Box::new(lhs),
                operator: op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }
}
