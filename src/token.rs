pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const ASSING: &str = "=";
pub const PLUS: &str = "+";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

pub struct Token {
    pub token_type: String,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: String, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn from_identifier(literal: &str) -> Token {
        match literal {
            "fn" => Token {
                token_type: FUNCTION.to_string(),
                literal: literal.to_string(),
            },
            "let" => Token {
                token_type: LET.to_string(),
                literal: literal.to_string(),
            },
            _ => Token {
                token_type: IDENT.to_string(),
                literal: literal.to_string(),
            },
        }
    }
}
