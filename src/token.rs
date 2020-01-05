use std::fmt::{Debug, Formatter, Error, Display};
use crate::TokenKind;

#[derive(Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub index: usize, // Index in src
    pub line: usize,
    pub col: usize,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.lexeme == other.lexeme
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, index: usize, line: usize, col: usize) -> Self {
        Self { kind, lexeme, index, line, col }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.lexeme)
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{{ {:?} `{}` {}:{}:{} }}", self.kind, self.lexeme, self.index, self.line, self.col)
    }
}
