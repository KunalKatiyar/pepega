use crate::lexer::token::Token;
use crate::parser::expr::Expr;

pub enum Stmt {
    Block {
        statements: Vec<Stmt>
    },
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    },
    Var {
        name: Token,
        initializer: Expr
    }
}