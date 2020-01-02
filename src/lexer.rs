use crate::{LexSyntax, Token, Catch, TokenKind};

pub struct Lexer<'a> {
    syntax: &'a LexSyntax,
    src: &'a str,
    tokens: Vec<Token<'a>>,
    line: usize,
    col: usize,
    ss: bool, // Space sensitivity
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, syntax: &'a LexSyntax) -> Self {
        Self { syntax, src, tokens: Vec::new(), line: 1, col: 0, ss: false }
    }

    pub fn space_sensitive(src: &'a str, syntax: &'a LexSyntax) -> Self {
        Self { syntax, src, tokens: Vec::new(), line: 1, col: 0, ss: true }
    }

    pub fn lex(mut self) -> Result<Vec<Token<'a>>, Vec<String>> {
        let mut errors = Vec::new();
        while !self.src.is_empty() {
            // Only trim whitespace if not sensitive
            // Rerun the loop if whitespace was trimmed so it can both remove further whitespace/comments or terminate if src is now empty
            if !self.ss { if self.remove_whitespace() { continue; } }
            self.find().catch(|err| errors.push(err));
        }

        if errors.is_empty() {
            self.tokens.push(Token::new(TokenKind::EOF, "<eof>", self.line, self.col));
            Ok(self.tokens)
        }
        else { Err(errors) }
    }

    /// Assume every regex is prepended with a caret, so start === 0
    /// Searches left to right through the symbol vector
    fn find(&mut self) -> Result<(), String> {
        for (regex, kind) in &self.syntax.symbols {
            if let Some(regex_match) = regex.find(self.src) {
                return Ok(self.emit(regex_match.as_str(), *kind))
            }
        }

        let err = Err(format!("Failed to find a symbol to match `{}`", self.src));
        // Push the lexer forward on failure and find the remaining errors
        if !self.src.is_empty() { self.inc(1) }
        err
    }

    /// Trims left whitespace and comments
    /// Returns whether something has been modified
    fn remove_whitespace(&mut self) -> bool {
        for regex in &self.syntax.comments {
            if let Some(m) = regex.find(self.src) {
                for c in m.as_str().chars() {
                    self.inc(1);
                    if c == '\n' { self.line += 1 }
                }
            }
        }

        if self.src.is_empty() { return true }

        match &self.src[0..1] {
            " "  => self.inc(1),
            "\n" => { self.inc(1); self.line += 1; self.col = 0 }
            "\t" => { self.inc(1); self.col += 3 }
            "\r" => { self.inc(1); self.col = 0 }
            _    => { return false } // Comments have been trimmed and there is no more whitespace
        };

        return true;
    }

    fn inc(&mut self, n: usize) { self.src = &self.src[n..]; self.col += n }

    fn emit(&mut self, lexeme: &'a str, kind: TokenKind) {
        let token = Token::new(kind, lexeme, self.line, self.col);
        self.inc(lexeme.len());
        self.tokens.push(token)
    }

}



