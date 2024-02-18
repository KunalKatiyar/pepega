use std::iter::Cloned;
use crate::interpreter::callable::{Callable};
use crate::interpreter::environment::Environment;
use crate::parser::stmt::Stmt;

#[derive(Debug)]
#[derive(Clone)]
pub enum LiteralValue {
    StringVal(String),
    NumberVal(i64),
    FloatVal(f64),
    NullVal,
    BooleanVal(bool),
    IdentifierVal(String),
    FunctionVal(Box<Stmt>),
    CallableVal(Box<Token>)
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::StringVal(s) => s.to_string(),
            LiteralValue::NumberVal(n) => n.to_string(),
            LiteralValue::FloatVal(f) => f.to_string(),
            LiteralValue::NullVal => "nil".to_string(),
            LiteralValue::IdentifierVal(i) => i.to_string(),
            LiteralValue::BooleanVal(b) => b.to_string(),
            LiteralValue::CallableVal(_) => "callable".to_string(),
            LiteralValue::FunctionVal(_) => "function".to_string()
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            LiteralValue::NullVal => false,
            LiteralValue::BooleanVal(b) => *b,
            LiteralValue::FloatVal(f) => *f != 0.0,
            LiteralValue::NumberVal(n) => *n != 0,
            LiteralValue::StringVal(s) => s.len() > 0,
            LiteralValue::IdentifierVal(_) => true,
            LiteralValue::CallableVal(_) => true,
            LiteralValue::FunctionVal(_) => true
        }
    }

    pub fn is_equal(self_val: LiteralValue, other_val: LiteralValue) -> bool {
        match self_val {
            LiteralValue::StringVal(s) => {
                match other_val {
                    LiteralValue::StringVal(o) => s == o,
                    _ => false
                }
            },
            LiteralValue::NumberVal(n) => {
                match other_val {
                    LiteralValue::NumberVal(o) => n == o,
                    _ => false
                }
            },
            LiteralValue::FloatVal(f) => {
                match other_val {
                    LiteralValue::FloatVal(o) => f == o,
                    _ => false
                }
            },
            LiteralValue::NullVal => {
                match other_val {
                    LiteralValue::NullVal => true,
                    _ => false
                }
            },
            LiteralValue::BooleanVal(b) => {
                match other_val {
                    LiteralValue::BooleanVal(o) => b == o,
                    _ => false
                }
            },
            LiteralValue::IdentifierVal(i) => {
                match other_val {
                    LiteralValue::IdentifierVal(o) => i == o,
                    _ => false
                }
            },
            LiteralValue::CallableVal(_) => {
                match other_val {
                    LiteralValue::CallableVal(_) => false,
                    _ => false
                }
            },
            LiteralValue::FunctionVal(_) => {
                match other_val {
                    LiteralValue::FunctionVal(_) => false,
                    _ => false
                }
            }
        }
    }
}

impl Callable for LiteralValue {
    fn arity(&self) -> usize {
        match self {
            LiteralValue::FunctionVal(stmt) => {
                let stmt_non_box = *(stmt.clone());
                match stmt_non_box {
                    Stmt::Function { params, .. } => params.len(),
                    _ => 0
                }
            },
            _ => 0
        }
    }

    fn call(&self, interpreter: &mut crate::interpreter::interpreter::Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, String> {
        match self {
            LiteralValue::CallableVal(name, ..) => {
                match interpreter.environment.get(name) {
                    Ok(v) => {
                        match v {
                            LiteralValue::FunctionVal(stmt) => {
                                match *stmt {
                                    Stmt::Function { params, body, .. } => {
                                        let environment_copy = interpreter.environment.clone();
                                        let mut environment = interpreter.environment.clone();
                                        for (i, param) in params.iter().enumerate() {
                                            environment.define(param.lexeme.clone(), arguments[i].clone()).expect("Error defining function parameter.");
                                        }
                                        interpreter.environment = environment;
                                        let result = interpreter.execute_block(body);
                                        interpreter.environment = environment_copy;
                                        result
                                    },
                                    _ => Err("Cannot call non-function.".to_string())
                                }
                            },
                            _ => Err("Cannot call non-function.".to_string())
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            _ => Err("Cannot call non-function.".to_string())
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