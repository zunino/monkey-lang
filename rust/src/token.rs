use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Assign,
    Plus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,

    Int(i64),
    Ident(String),
}

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("fn", Token::Function);
        m.insert("let", Token::Let);
        m
    };
}

pub fn get_keyword(word: &str) -> Option<&Token> {
    KEYWORD_MAP.get(word)
}

