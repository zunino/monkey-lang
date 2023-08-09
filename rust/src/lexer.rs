use crate::token::{ETokenType, Token, TokenType};

struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.bytes().collect(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn next_token(&mut self) -> Token {
        let mut tok: Token;
        match self.ch {
            b'=' => { tok = Token::new(ETokenType::Assign, "=") },
            b';' => { tok = Token::new(ETokenType::Semicolon, ";") },
            _ => { tok = Token::new(ETokenType::Illegal, "") },
        }
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"=;+";
        let mut lexer = Lexer::new(input);

        let expected = vec![ETokenType::Assign, ETokenType::Semicolon, ETokenType::Illegal];

        for t in 0..expected.len() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected[t]);
        }
    }

    #[test]
    fn test_read_char_should_read_all_chars_from_given_input() {
        let input = r"BAR";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.ch, 'B' as u8);
        lexer.read_char();
        assert_eq!(lexer.ch, 'A' as u8);
        lexer.read_char();
        assert_eq!(lexer.ch, 'R' as u8);
        lexer.read_char();
        assert_eq!(lexer.ch, 0 as u8);
    }
}
