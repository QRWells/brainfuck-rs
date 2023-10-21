use std::fmt;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CompileErrorKind {
    #[error("Unexpected character")]
    UnclosedCharacter,
    #[error("Unclosed left bracket")]
    UnclosedLeftBracket,
    #[error("Unexpected right bracket")]
    UnexpectedRightBracket,
}

#[derive(Debug)]
pub struct CompileError {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) kind: CompileErrorKind,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}:{}", self.kind, self.line, self.col)
    }
}
