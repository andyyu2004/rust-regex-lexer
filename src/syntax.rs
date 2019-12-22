use regex::Regex;
use crate::TokenKind;

/// T is the TokenType
pub struct LexSyntax {
    pub symbols: Vec<(Regex, TokenKind)>,
    pub comments: Vec<Regex>
}
