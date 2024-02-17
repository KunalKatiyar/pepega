use crate::lexer::token::{LiteralValue, Token};

#[derive(Clone)]
#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>
    },
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Variable {
        name: Token
    }
}

impl Expr {
    pub fn new_assign(name: Token, value: Expr) -> Expr {
        Expr::Assign {
            name,
            value: Box::new(value)
        }
    }
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

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }
    }

    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right)
        }
    }

    pub fn new_variable(name: Token) -> Expr {
        Expr::Variable {
            name
        }
    }
}