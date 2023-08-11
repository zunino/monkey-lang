mod token;
mod lexer;
mod repl;

use std::io::{stdin, stdout};

fn main() {
    repl::start(stdin(), stdout());
}
