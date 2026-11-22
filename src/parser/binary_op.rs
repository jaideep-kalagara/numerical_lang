use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq, strum::Display)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<Token> for BinaryOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Sub,
            Token::Multiply => BinaryOp::Mul,
            Token::Divide => BinaryOp::Div,
            _ => panic!("Invalid token!"),
        }
    }
}
