use regex::Regex;
use crate::TokenKind;
use std::collections::HashMap;

/// T is the TokenType
pub struct LexSyntax {
    pub symbols: Vec<(Regex, TokenKind)>,
    pub keywords: HashMap<&'static str, TokenKind>,
    pub comments: Vec<Regex>,    
}
