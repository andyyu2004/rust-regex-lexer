
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind {
    // Symbols
    Backslash, Dot, Equal,
    LParen, RParen,
    Space,
    // Keywords
    In, Let,

    // Classes
    Identifier,
    Typevar
}
