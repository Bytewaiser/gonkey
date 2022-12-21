mod lexer;
pub mod token;

fn main() {
    let input = 
        "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ";
    let mut parser = lexer::Lexer::new(input);
    parser.parse();
}
