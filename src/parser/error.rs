use crate::{lexer::token::Token, span::Span};

#[derive(Debug, strum::Display)]
pub enum ParserError {
    #[strum(serialize = "Invalid binary operator: {token}")]
    InvalidBinaryOp { token: Token, span: Span },
    #[strum(serialize = "Invalid token: {token}")]
    InvalidToken { token: Token, span: Span },
    #[strum(serialize = "Unexpected end of file")]
    UnexpectedEof { span: Span },
}

impl ParserError {
    pub fn span(&self) -> &Span {
        match self {
            Self::InvalidBinaryOp { span, .. } => span,
            Self::InvalidToken { span, .. } => span,
            Self::UnexpectedEof { span, .. } => span,
        }
    }
}

impl std::error::Error for ParserError {}
