use crate::lexer::token::{LiteralValue, Token};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LiteralValue
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
    }

    pub fn new_grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: Box::new(expression)
        }
    }

    pub fn new_literal(value: LiteralValue) -> Expr {
        Expr::Literal {
            value
        }
    }

    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right)
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string()),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme, right.to_string())
        }
    }
}