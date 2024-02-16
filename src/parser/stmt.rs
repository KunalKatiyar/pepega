use crate::lexer::token::Token;
use crate::parser::expr::Expr;

pub enum Stmt {
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