use crate::token::{Token, get_keyword};
use std::str;

struct Lexer {
    input: Vec<u8>,
    pos: usize,
    rpos: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.bytes().collect(),
            pos: 0,
            rpos: 0,
            ch: 0,
        };
        lexer.consume_char();
        lexer
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            n @ b'0'..=b'9' => Token::Int((n - 0x30) as i64),
            b'a'..=b'z' => self.read_word(),
            _ => Token::Illegal,
        };
        self.consume_char();
        println!("----> {:?}", token);
        token
    }

    fn consume_char(&mut self) {
        if self.rpos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.rpos];
        }
        self.pos = self.rpos;
        self.rpos += 1;
    }

    fn read_word(&mut self) -> Token {
        let position = self.pos;

        while self.ch.is_ascii_alphabetic() {
            self.consume_char();
        }

        let word = str::from_utf8(&self.input[position..self.pos])
                .unwrap();

        if let Some(keyword_token) = get_keyword(word) {
            return keyword_token.clone();
        }
        Token::Ident(word.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
        ];

        for t in 0..expected.len() {
            let tok = lexer.next_token();
            assert_eq!(tok, expected[t]);
        }
    }

    #[test]
    fn test_read_char_should_read_all_chars_from_given_input() {
        let input = r"BAR";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.ch, 'B' as u8);
        lexer.consume_char();
        assert_eq!(lexer.ch, 'A' as u8);
        lexer.consume_char();
        assert_eq!(lexer.ch, 'R' as u8);
        lexer.consume_char();
        assert_eq!(lexer.ch, 0 as u8);
    }
}
