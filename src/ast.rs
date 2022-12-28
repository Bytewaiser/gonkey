use crate::token;

trait Node {
    fn token_literal(&self) -> String;
}

struct Statement {}
impl Node for Statement {
    fn token_literal(&self) -> String {todo!()} 
}

struct Expression {}
impl Node for Expression {
    fn token_literal(&self) -> String {todo!()} 
}

struct Program {
    statements: Vec<Statement>,
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

struct Identifier {
    token: token::Token,
    value: String,
}

struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: Expression,
}
