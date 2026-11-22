use crate::span::Span;

#[derive(Debug, strum::Display)]
pub enum LexerError {
    #[strum(serialize = "Invalid character: '{char}'")]
    InvalidCharacter { span: Span, char: char },
    #[strum(serialize = "Invalid character: none")]
    InvalidCharacterWithUnknownInfo { span: Span },
    #[strum(serialize = "Unterminated string literal")]
    UnterminatedStringLiteral { span: Span },
    #[strum(serialize = "Unknown escape code character: '{char}'")]
    UnknownEscapeCodeCharacter { span: Span, char: char },
    #[strum(serialize = "Invalid number: \"{number}\"")]
    InvalidNumber { span: Span, number: String },
}

impl LexerError {
    pub fn span(&self) -> &Span {
        match self {
            Self::InvalidCharacter { span, .. } => span,
            Self::InvalidCharacterWithUnknownInfo { span } => span,
            Self::UnterminatedStringLiteral { span } => span,
            Self::UnknownEscapeCodeCharacter { span, .. } => span,
            Self::InvalidNumber { span, .. } => span,
        }
    }
}

impl std::error::Error for LexerError {}
