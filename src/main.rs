pub mod ast;
pub mod lexer;
mod parser;
mod repl;
pub mod token;

fn main() {
    let _input = "
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
