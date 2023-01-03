use std::ptr::null;

use crate::ast;
use crate::lexer;
use crate::token;

struct Parser {
    l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    fn new(l: lexer::Lexer) -> Parser {
        let mut l = l.clone();
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Parser{l, cur_token, peek_token}
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_let_statement(&self) -> ast::LetStatement {
        let let_statement = ast::LetStatement::new();

        let_statement
    }

    fn parse_statement(&self) -> dyn ast::Statement {
        let default_statement = token::Token::new(token::EOF.to_string(), '\0'.to_string());
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement(),
            _ => default_statement
            
        }
    }

    fn parse_program(&self) -> ast::Program {
        let program = ast::Program::new();

        while self.cur_token.token_type != token::EOF {
        }

        program

    }

}
