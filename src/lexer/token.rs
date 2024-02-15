#[derive(Debug)]
#[derive(Clone)]
pub enum LiteralValue {
    StringVal(String),
    NumberVal(i64),
    FloatVal(f64),
    NullVal,
    BooleanVal(bool),
    IdentifierVal(String)
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::StringVal(s) => s.to_string(),
            LiteralValue::NumberVal(n) => n.to_string(),
            LiteralValue::FloatVal(f) => f.to_string(),
            LiteralValue::NullVal => "nil".to_string(),
            LiteralValue::IdentifierVal(i) => i.to_string(),
            LiteralValue::BooleanVal(b) => b.to_string()
        }
    }

}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<LiteralValue>
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, line: usize, literal: Option<LiteralValue>) -> Token {
        Token {
            kind,
            lexeme,
            line,
            literal
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.kind, self.lexeme, self.literal.as_ref().unwrap().to_string())
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn display(&self) {
        println!("{}", self.to_string());
    }

    pub fn clone(&self) -> Token {
        Token {
            kind: self.kind.clone(),
            lexeme: self.lexeme.clone(),
            line: self.line,
            literal: self.literal.clone()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}