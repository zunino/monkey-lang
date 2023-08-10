use crate::token::Token;

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
            id @ b'a'..=b'z' => Token::Ident(String::from(id as char)),
            _ => Token::Illegal
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"=;+";
        let mut lexer = Lexer::new(input);

        let expected = vec![Token::Assign, Token::Semicolon, Token::Plus];

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
