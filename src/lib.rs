mod lexer;
mod syntax;
mod token;
mod tokenkind;
mod errors;

pub use lexer::Lexer;
pub use syntax::LexSyntax;
pub use token::Token;
pub use tokenkind::TokenKind;
pub use errors::Catch;

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn gen_syntax() -> LexSyntax {
        LexSyntax {
            symbols: vec! [
                (Regex::new(r#"^ "#).unwrap(),     TokenKind::Space),
                (Regex::new(r#"^\\"#).unwrap(),    TokenKind::Backslash),
                (Regex::new(r#"^\("#).unwrap(),    TokenKind::LParen),
                (Regex::new(r#"^\)"#).unwrap(),    TokenKind::RParen),
                (Regex::new(r#"^\."#).unwrap(),    TokenKind::Dot),
                (Regex::new(r#"^let"#).unwrap(),   TokenKind::Let),
                (Regex::new(r#"^in"#).unwrap(),    TokenKind::In),
                (Regex::new(r#"^[a-z]"#).unwrap(), TokenKind::Identifier)
            ],
            comments: vec! [
                Regex::new(r#"^//.*(\n|\z)"#).unwrap(),
                Regex::new(r#"^/\*.*\*/"#).unwrap()
            ]
        }
    }

    #[test]
    fn lexer() {
        let syntax = gen_syntax();
        let lexer = Lexer::new(r#"\().letin)"#, &syntax);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        assert_eq!(tokens, Ok(vec! [
            Token::new(TokenKind::Backslash, "\\".to_owned(), 1, 0),
            Token::new(TokenKind::LParen, "(".to_owned(), 1, 1),
            Token::new(TokenKind::RParen, ")".to_owned(), 1, 2),
            Token::new(TokenKind::Dot, ".".to_owned(), 1, 3),
            Token::new(TokenKind::Let, "let".to_owned(), 1, 4),
            Token::new(TokenKind::In, "in".to_owned(), 1, 7),
            Token::new(TokenKind::RParen, ")".to_owned(), 1, 9),
        ]));
    }

    #[test]
    fn comments() {
        let syntax = gen_syntax();
        let lexer = Lexer::new(r#"//hello)"#, &syntax);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        assert_eq!(tokens, Ok(vec![]))
    }

    #[test]
    fn whitespace_sensitive() {
        let syntax = gen_syntax();
        let lexer = Lexer::space_sensitive(r#"\x.\y.x y"#, &syntax);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        assert_eq!(tokens, Ok(vec![
            Token::new(TokenKind::Backslash, "\\".to_owned(), 1, 0),
            Token::new(TokenKind::Identifier, "x".to_owned(), 1, 1),
            Token::new(TokenKind::Dot, ".".to_owned(), 1, 2),
            Token::new(TokenKind::Backslash, "\\".to_owned(), 1, 3),
            Token::new(TokenKind::Identifier, "y".to_owned(), 1, 4),
            Token::new(TokenKind::Dot, ".".to_owned(), 1, 5),
            Token::new(TokenKind::Identifier, "x".to_owned(), 1, 6),
            Token::new(TokenKind::Space, " ".to_owned(), 1, 7),
            Token::new(TokenKind::Identifier, "y".to_owned(), 1, 8),
        ]))
    }




}


