use std::io;

pub mod lexer;
mod repl;
pub mod token;

fn main() {
    let input = "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 6;

        if (5 < 10) {
            return true;
        }
        else {
            return false;

        10 == 10;
        9 != 10;
        }
    ";

    repl::start();

    // let mut parser = lexer::Lexer::new(input);
    // parser.parse();
}
