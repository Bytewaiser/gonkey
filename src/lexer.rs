use crate::token;
use crate::token::Token;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '-'
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        let literal = self.input.get(position..self.position).unwrap();
        Token::from_identifier(literal)
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char()
        }
        let literal = self.input.get(position..self.position).unwrap();
        Token::new(token::INT.to_string(), literal.to_string())
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char()
        }
    }

    fn next_token(&mut self) -> Token {
        if self.ch.is_whitespace() {
            self.skip_whitespace()
        }
        let mut skip = false;
        let tok: Token = match self.ch {
            '=' => Token::new(token::ASSING.to_string(), self.ch.to_string()),
            '+' => Token::new(token::PLUS.to_string(), self.ch.to_string()),
            '(' => Token::new(token::LPAREN.to_string(), self.ch.to_string()),
            ')' => Token::new(token::RPAREN.to_string(), self.ch.to_string()),
            '{' => Token::new(token::LBRACE.to_string(), self.ch.to_string()),
            '}' => Token::new(token::RBRACE.to_string(), self.ch.to_string()),
            ',' => Token::new(token::COMMA.to_string(), self.ch.to_string()),
            ';' => Token::new(token::SEMICOLON.to_string(), self.ch.to_string()),
            '\0' => Token::new(token::EOF.to_string(), self.ch.to_string()),
            x => {
                if is_letter(x) {
                    skip = true;
                    self.read_identifier()
                } else if x.is_digit(10) {
                    skip = true;
                    self.read_number()
                } else {
                    Token::new(token::ILLEGAL.to_string(), self.ch.to_string())
                }
            }
        };
        if !skip {
            self.read_char();
        }
        tok
    }

    pub fn parse(&mut self) {
        while self.ch != '\0' {
            let tok = self.next_token();
            println!("{} - {}", tok.token_type, tok.literal);
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            ";
        struct ExpectedToken<'a> {
            expected_type: &'a str,
            expected_literal: &'a str,
        }

        impl ExpectedToken<'_> {
            fn new<'a>(expected_type: &'a str, expected_literal: &'a str) -> ExpectedToken<'a> {
                ExpectedToken {
                    expected_type,
                    expected_literal,
                }
            }
        }

        let tests = [
            ExpectedToken::new(token::LET, "let"),
            ExpectedToken::new(token::IDENT, "five"),
            ExpectedToken::new(token::ASSING, "="),
            ExpectedToken::new(token::INT, "5"),
            ExpectedToken::new(token::SEMICOLON, ";"),
            ExpectedToken::new(token::LET, "let"),
            ExpectedToken::new(token::IDENT, "ten"),
            ExpectedToken::new(token::ASSING, "="),
            ExpectedToken::new(token::INT, "10"),
            ExpectedToken::new(token::SEMICOLON, ";"),
            ExpectedToken::new(token::LET, "let"),
            ExpectedToken::new(token::IDENT, "add"),
            ExpectedToken::new(token::ASSING, "="),
            ExpectedToken::new(token::FUNCTION, "fn"),
            ExpectedToken::new(token::LPAREN, "("),
            ExpectedToken::new(token::IDENT, "x"),
            ExpectedToken::new(token::COMMA, ","),
            ExpectedToken::new(token::IDENT, "y"),
            ExpectedToken::new(token::RPAREN, ")"),
            ExpectedToken::new(token::LBRACE, "{"),
            ExpectedToken::new(token::IDENT, "x"),
            ExpectedToken::new(token::PLUS, "+"),
            ExpectedToken::new(token::IDENT, "y"),
            ExpectedToken::new(token::SEMICOLON, ";"),
            ExpectedToken::new(token::RBRACE, "}"),
            ExpectedToken::new(token::SEMICOLON, ";"),
            ExpectedToken::new(token::LET, "let"),
            ExpectedToken::new(token::IDENT, "result"),
            ExpectedToken::new(token::ASSING, "="),
            ExpectedToken::new(token::IDENT, "add"),
            ExpectedToken::new(token::LPAREN, "("),
            ExpectedToken::new(token::IDENT, "five"),
            ExpectedToken::new(token::COMMA, ","),
            ExpectedToken::new(token::IDENT, "ten"),
            ExpectedToken::new(token::RPAREN, ")"),
            ExpectedToken::new(token::SEMICOLON, ";"),
            ExpectedToken::new(token::EOF, "\0"),
        ];

        let mut l = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert!(
                tok.token_type == tt.expected_type,
                "tests[{}] - tokentype wrong expected={} got={}",
                i,
                tt.expected_type,
                tok.token_type
            );
            assert!(
                tok.literal == tt.expected_literal,
                "tests[{}] - literal wrong expected={} got={}",
                i,
                tt.expected_literal,
                tok.literal
            );
        }
    }
}
