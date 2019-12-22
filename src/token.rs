use std::fmt::{Debug, Formatter, Error};
use crate::TokenKind;

#[derive(PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: usize,
    col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, col: usize) -> Self {
        Self { kind, lexeme, line, col }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{{ {:?} {} {} }}", self.kind, self.line, self.col)
    }
}
