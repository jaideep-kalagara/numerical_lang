use crate::{
    lexer::token::Token,
    parser::error::ParserError,
    span::{Span, Spanned},
};

#[derive(Debug, Clone, PartialEq, strum::Display)]
pub enum BinaryOp {
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "*")]
    Mul,
    #[strum(serialize = "/")]
    Div,
}

impl BinaryOp {
    pub fn infix_binding_power(&self) -> (u8, u8) {
        match &self {
            BinaryOp::Add | BinaryOp::Sub => (1, 2),
            BinaryOp::Mul | BinaryOp::Div => (3, 4),
        }
    }

    pub fn prefix_binding_power(&self, span: Span) -> Result<((), u8), ParserError> {
        match self {
            BinaryOp::Add | BinaryOp::Sub => Ok(((), 6)),
            op => Err(ParserError::InvalidUnaryOp {
                op: op.clone(),
                span,
            }),
        }
    }
}

impl BinaryOp {
    pub fn from_token(token: &Spanned<Token>) -> Option<BinaryOp> {
        match token.value {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Multiply => Some(BinaryOp::Mul),
            Token::Divide => Some(BinaryOp::Div),
            _ => {
                return None;
            }
        }
    }
}
