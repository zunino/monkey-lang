#[derive(Debug, PartialEq)]
pub struct TokenType(String);

impl TokenType {
    fn new(s: &str) -> Self {
        TokenType(String::from(s))
    }
}

pub struct Token {
    pub token_type: ETokenType,
    pub text: String,
}

impl Token {
    pub fn new(token_type: ETokenType, text: &str) -> Self {
        Token {
            token_type,
            text: String::from(text),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ETokenType {
    Illegal,
    Eof,

    Ident,
    Int,

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
}

fn gimme_a_token_type() -> TokenType {
    TokenType::new("I am a token type")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type() {
        let t = gimme_a_token_type();
        assert_eq!(t.0, "I am a token type".to_string());
    }
}
