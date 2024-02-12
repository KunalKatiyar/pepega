#[derive(Debug)]
pub enum LiteralValue {
    StringVal(String),
    NumberVal(i64),
    FloatVal(f64),
    NullVal,
    IdentifierVal(String)
}

#[derive(Debug)]
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
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(Clone)]
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