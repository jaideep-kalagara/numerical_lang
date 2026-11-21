#[derive(Debug, strum::Display)]
pub enum LexerError {
    #[strum(serialize = "Invalid character: '{char}', line: {line}")]
    InvalidCharacter { line: usize, char: char },
    #[strum(serialize = "Invalid number: \"{number}\", line: {line}")]
    InvalidNumber { line: usize, number: String },
}

impl std::error::Error for LexerError {}
