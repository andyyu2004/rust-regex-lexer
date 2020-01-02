
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    // Symbols
    Backslash, Dot, Equal,
    LParen, RParen,
    Space,
    // Keywords
    In, Let,
    Lambda,
    Plus, Minus, Slash, Times,
    Tilde, Bang,
    EOF,
    
    // Classes
    Identifier,
    Typevar,
    Metavar,
    Integral,
    Float,
    Unknown,
}
