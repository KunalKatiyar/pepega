use std::fmt::Display;
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    },
    Get {
        object: Box<Expr>,
        name: Token
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
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>
    },
    This {
        keyword: Token
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Variable {
        name: Token
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Assign { ref name, ref value } => {
                write!(f, "Assign: {:?}, {:?}", name, value)
            },
            Expr::Binary { ref left, ref operator, ref right } => {
                write!(f, "Binary: {:?}, {:?}, {:?}", left, operator, right)
            },
            Expr::Call { ref callee, ref paren, ref arguments } => {
                write!(f, "Call: {:?}, {:?}, {:?}", callee, paren, arguments)
            },
            Expr::Grouping { ref expression } => {
                write!(f, "Grouping: {:?}", expression)
            },
            Expr::Literal { ref value } => {
                write!(f, "Literal: {:?}", value)
            },
            Expr::Logical { ref left, ref operator, ref right } => {
                write!(f, "Logical: {:?}, {:?}, {:?}", left, operator, right)
            },
            Expr::Unary { ref operator, ref right } => {
                write!(f, "Unary: {:?}, {:?}", operator, right)
            },
            Expr::Variable { ref name } => {
                write!(f, "Variable: {:?}", name)
            },
            Expr::Get { ref object, ref name } => {
                write!(f, "Get: {:?}, {:?}", object, name)
            },
            Expr::Set { ref object, ref name, ref value } => {
                write!(f, "Set: {:?}, {:?}, {:?}", object, name, value)
            },
            Expr::This { ref keyword } => {
                write!(f, "This: {:?}", keyword)
            }
        }
    }
}

impl Expr {
    pub fn new_assign(name: Token, value: Expr) -> Expr {
        Expr::Assign {
            name,
            value: Box::new(value)
        }
    }

    pub fn new_get(object: Expr, name: Token) -> Expr {
        Expr::Get {
            object: Box::new(object),
            name
        }
    }

    pub fn new_set(object: Expr, name: Token, value: Expr) -> Expr {
        Expr::Set {
            object: Box::new(object),
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

    pub fn new_call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Expr {
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments
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

    pub fn new_this(keyword: Token) -> Expr {
        Expr::This {
            keyword
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