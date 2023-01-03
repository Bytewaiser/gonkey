use crate::token;

trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
}

pub trait Expression: Node {
}

pub struct Program {
    pub statements: Vec<dyn Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            '\0'.to_string()
        }
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

struct Identifier {
    token: token::Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal
    }
}

impl Identifier {
    pub fn new(tok: token::Token, value: String) -> Identifier {
        Identifier { token: tok, value }
    }
}

pub struct LetStatement {
    token: token::Token,
    name: *mut Identifier,
    value: dyn Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl LetStatement {
    pub fn new() -> LetStatement {
        let tok = token::Token::new(token::LET.to_string(), "let".to_string());
        LetStatement {
            token: tok,
            name: &mut Identifier::new(tok, "let".to_string()),
            value: Expression::new()
        }
    }
}
