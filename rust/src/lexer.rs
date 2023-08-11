use crate::token::{get_keyword, Token};
use std::str;

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    rpos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.bytes().collect(),
            pos: 0,
            rpos: 0,
            ch: 0,
        };
        lexer.consume_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::Assign,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            _ if self.ch.is_ascii_digit() => self.read_number(),
            _ if self.ch.is_ascii_alphabetic() => self.read_word(),
            0u8 => Token::Eof,
            _ => Token::Illegal,
        };
        self.consume_char();
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

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.consume_char();
        }
    }

    fn rewind(&mut self) {
        self.pos -= 1;
        self.rpos = self.pos + 1;
    }

    fn read_word(&mut self) -> Token {
        let position = self.pos;

        while self.ch.is_ascii_alphabetic() {
            self.consume_char();
        }

        let word = str::from_utf8(&self.input[position..self.pos])
            .unwrap()
            .to_owned();

        self.rewind();

        if let Some(keyword_token) = get_keyword(&word) {
            return keyword_token.clone();
        }

        Token::Ident(word.to_string())
    }

    fn read_number(&mut self) -> Token {
        let position = self.pos;

        while self.ch.is_ascii_digit() {
            self.consume_char();
        }

        let number = str::from_utf8(&self.input[position..self.pos])
            .unwrap()
            .parse::<i64>()
            .unwrap();

        self.rewind();

        Token::Int(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"let   five  =   5;
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
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
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
