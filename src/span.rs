use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn line_column(source: &str, position: usize) -> (usize, usize) {
        let preceding = &source[..position];

        let line = preceding.matches('\n').count() + 1;

        let column = preceding.rsplit('\n').next().unwrap().chars().count() + 1;

        (line, column)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, start: usize, end: usize) -> Self {
        Self {
            value,
            span: Span::new(start, end),
        }
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.value, self.span)
    }
}
