use std::io;

use crate::{lexer::Lexer, token};

const PROMPT: &str = ">> ";

pub fn start() {
    let scanner = io::stdin();

    let mut buffer = String::new();
    print!("{}", PROMPT);
    loop {
        match scanner.read_line(&mut buffer) {
            Ok(_) => {
                let mut l = Lexer::new(&buffer.trim());
                loop {
                    let tok = l.next_token();
                    if tok.token_type == token::EOF { break }
                    println!("{:?}", tok);
                }
                buffer.clear();
            }
            Err(_) => return,
        }
    }
}
