pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
// Identifiers + Literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
// Operators
pub const ASSING: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const BANG: &str = "!";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
// Delimeters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

#[derive(Debug, Clone)]
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
            "true" => Token {
                token_type: TRUE.to_string(),
                literal: literal.to_string(),
            },
            "false" => Token {
                token_type: FALSE.to_string(),
                literal: literal.to_string(),
            },
            "if" => Token {
                token_type: IF.to_string(),
                literal: literal.to_string(),
            },
            "else" => Token {
                token_type: ELSE.to_string(),
                literal: literal.to_string(),
            },
            "return" => Token {
                token_type: RETURN.to_string(),
                literal: literal.to_string(),
            },
            _ => Token {
                token_type: IDENT.to_string(),
                literal: literal.to_string(),
            },
        }
    }
}
