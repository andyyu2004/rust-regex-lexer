use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    // Symbols
    Backslash, Dot, Equal,
    LParen, RParen,
    Space,
    Plus, Minus, Slash, Star, 
    Tilde, Bang, Caret, DStar,
    LT, LTE, GT, GTE, DEqual, BangEqual,
    True, False,
    Colon, SemiColon, LBrace, RBrace,
    EOF, 

    // Keywords
    In, Let,
    Lambda,
    Fn,
    
    // Classes
    Identifier,
    Typevar,
    Metavar,
    Integral,
    Float,
    Str,
    Unknown,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Backslash  => write!(f, "\\"),
            TokenKind::Dot        => write!(f, "."),
            TokenKind::Equal      => write!(f, "="),
            TokenKind::LParen     => write!(f, "("),
            TokenKind::RParen     => write!(f, ")"),
            TokenKind::Space      => write!(f, " "),
            TokenKind::In         => write!(f, "in"),
            TokenKind::Let        => write!(f, "let"),
            TokenKind::Lambda     => write!(f, "λ"),
            TokenKind::Plus       => write!(f, "+"),
            TokenKind::Minus      => write!(f, "-"),
            TokenKind::Slash      => write!(f, "/"),
            TokenKind::Star       => write!(f, "*"),
            TokenKind::DStar      => write!(f, "**"),
            TokenKind::Tilde      => write!(f, "~"),
            TokenKind::Bang       => write!(f, "!"),
            TokenKind::Caret      => write!(f, "^"),
            TokenKind::EOF        => write!(f, "<eof>"),
            TokenKind::False      => write!(f, "false"),
            TokenKind::True       => write!(f, "true"),
            TokenKind::Identifier => write!(f, "<identifier>"),
            TokenKind::Typevar    => write!(f, "<tvar>"),
            TokenKind::Metavar    => write!(f, "unknown"),
            TokenKind::Integral   => write!(f, "<integral>"),
            TokenKind::Float      => write!(f, "<float>"),
            TokenKind::Str        => write!(f, "<str>"),
            TokenKind::Unknown    => write!(f, "<unknown>"),
            TokenKind::LT         => write!(f, "<"),
            TokenKind::LTE        => write!(f, "<="),
            TokenKind::GT         => write!(f, ">"),
            TokenKind::GTE        => write!(f, ">="),
            TokenKind::DEqual     => write!(f, "=="),
            TokenKind::BangEqual  => write!(f, "!="),
            TokenKind::Colon      => write!(f, ":"),
            TokenKind::SemiColon  => write!(f, ";"),
            TokenKind::LBrace     => write!(f, "{{"),
            TokenKind::RBrace     => write!(f, "}}"),
            TokenKind::Fn         => write!(f, "fn"),
        }
    }
}