use std::fmt::{Debug, Formatter, Error, Display};
use crate::TokenKind;

#[derive(PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub line: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, line: usize, col: usize) -> Self {
        Self { kind, lexeme, line, col }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.lexeme)
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{{ {:?} {} {} {} }}", self.kind, self.lexeme, self.line, self.col)
    }
}
