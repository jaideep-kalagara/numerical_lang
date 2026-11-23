use crate::{lexer::token::Token, parser::binary_op::BinaryOp, span::Span};

#[derive(Debug, strum::Display)]
pub enum ParserError {
    #[strum(serialize = "Invalid binary operator: {token}")]
    InvalidBinaryOp { token: Token, span: Span },
    #[strum(serialize = "Invalid unary operator: {op}")]
    InvalidUnaryOp { op: BinaryOp, span: Span },
    #[strum(serialize = "Invalid token: {token}")]
    InvalidToken { token: Token, span: Span },
    #[strum(serialize = "Expected token: {expected}, got token: {got}")]
    ExpectedToken {
        expected: Token,
        got: Token,
        span: Span,
    },
    #[strum(serialize = "Unexpected end of file")]
    UnexpectedEof { span: Span },
}

impl ParserError {
    pub fn span(&self) -> &Span {
        match self {
            Self::InvalidBinaryOp { span, .. } => span,
            Self::InvalidToken { span, .. } => span,
            Self::InvalidUnaryOp { span, .. } => span,
            Self::ExpectedToken { span, .. } => span,
            Self::UnexpectedEof { span, .. } => span,
        }
    }
}

impl std::error::Error for ParserError {}
