#[derive(Debug, PartialEq)]
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
