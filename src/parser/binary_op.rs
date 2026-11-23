use crate::{lexer::token::Token, parser::error::ParserError, span::Spanned};

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
}

impl TryFrom<&Spanned<Token>> for BinaryOp {
    fn try_from(token: &Spanned<Token>) -> Result<Self, Self::Error> {
        match token.value {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Sub),
            Token::Multiply => Ok(BinaryOp::Mul),
            Token::Divide => Ok(BinaryOp::Div),
            _ => {
                return Err(ParserError::InvalidBinaryOp {
                    token: token.value.clone(),
                    span: token.span,
                });
            }
        }
    }

    type Error = ParserError;
}
