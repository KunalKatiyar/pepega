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
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Print {
        expression: Expr
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