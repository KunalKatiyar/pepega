use std::fmt::Display;
use crate::lexer::token::Token;
use crate::parser::expr::Expr;
#[derive(Clone)]
#[derive(Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>
    },
    Expression {
        expression: Expr
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Print {
        expression: Expr
    },
    Return {
        keyword: Token,
        value: Option<Expr>
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    },
    Var {
        name: Token,
        initializer: Expr
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::Block { ref statements } => {
                write!(f, "Block: {:?}", statements)
            },
            Stmt::Expression { ref expression } => {
                write!(f, "Expression: {:?}", expression)
            },
            Stmt::Function { ref name, ref params, ref body } => {
                write!(f, "Function: {:?}, {:?}, {:?}", name, params, body)
            },
            Stmt::If { ref condition, ref then_branch, ref else_branch } => {
                write!(f, "If: {:?}, {:?}, {:?}", condition, then_branch, else_branch)
            },
            Stmt::Print { ref expression } => {
                write!(f, "Print: {:?}", expression)
            },
            Stmt::While { ref condition, ref body } => {
                write!(f, "While: {:?}, {:?}", condition, body)
            },
            Stmt::Var { ref name, ref initializer } => {
                write!(f, "Var: {:?}, {:?}", name, initializer)
            },
            Stmt::Return { ref keyword, ref value } => {
                write!(f, "Return: {:?}, {:?}", keyword, value)
            }
        }
    }
}