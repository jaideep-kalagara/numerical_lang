#[derive(Debug, strum::Display)]
pub enum LexerError {
    #[strum(serialize = "Invalid character: '{char}', line: {line}")]
    InvalidCharacter { line: usize, char: char },
    #[strum(serialize = "Invalid character: none, line: {line}")]
    InvalidCharacterWithUnknownInfo { line: usize },
    #[strum(serialize = "Unterminated string literal line: {line}")]
    UnterminatedStringLiteral { line: usize },
    #[strum(serialize = "Invalid number: \"{number}\", line: {line}")]
    InvalidNumber { line: usize, number: String },
}

impl std::error::Error for LexerError {}
