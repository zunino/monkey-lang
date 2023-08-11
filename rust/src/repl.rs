use std::io::{Stdin, Stdout, Write};
use crate::lexer::Lexer;
use crate::token::Token;

pub fn start(input: Stdin, mut output: Stdout) {
    loop {
        let mut user_input = String::new();
        let _ = output.write("monkey => ".as_bytes());
        let _ = output.flush();
        input.read_line(&mut user_input).expect("error");

        let mut lexer = Lexer::new(&user_input);
        loop {
            let token = lexer.next_token();
            match token {
                Token::Eof => break,
                Token::Illegal => break,
                _ => println!("{:?}", token),
            }
        }
    }
}
