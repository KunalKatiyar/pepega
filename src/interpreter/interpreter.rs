use crate::interpreter::environment::Environment;
use crate::lexer::token::LiteralValue;
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;

#[derive(Clone)]
pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new()
        }
    }

    pub fn interpret_stmt(&mut self, stmt: Vec<Stmt>) {
        for s in stmt {
            self.execute(s);
        }
    }

    pub fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Block { statements } => {
                let previous = self.environment.clone();
                self.environment = Environment::new_with_enclosing(previous.clone());
                for s in statements {
                    self.execute(s);
                }
                self.environment = *(self.environment.enclosing.clone().unwrap());
            },
            Stmt::Expression { expression } => {
                match self.evaluate_expr(expression) {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e)
                }
            },
            Stmt::While { condition, body } => {
                loop {
                    match self.evaluate_expr(condition.clone()) {
                        Ok(v) => {
                            if v.is_truthy() {
                                self.execute(*(body.clone()));
                            } else {
                                break;
                            }
                        },
                        Err(e) => panic!("{}", e)
                    }
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                match self.evaluate_expr(condition) {
                    Ok(v) => {
                        if v.is_truthy() {
                            self.execute(*then_branch);
                        } else {
                            match else_branch {
                                Some(b) => self.execute(*b),
                                None => ()
                            }
                        }
                    },
                    Err(e) => panic!("{}", e)
                }
            },
            Stmt::Print { expression } => {
                match self.evaluate_expr(expression) {
                    Ok(v) => println!("{}", v.to_string()),
                    Err(e) => panic!("{}", e)
                }
            },
            Stmt::Var { name, initializer } => {
                let value = self.evaluate_expr(initializer).unwrap();
                self.environment.define(name.lexeme, value);
            }
        }
    }

    pub fn check_operands(&mut self, left: LiteralValue, right: LiteralValue, message: &str) -> Result<(), String> {
        match (left, right) {
            (LiteralValue::NumberVal(_), LiteralValue::NumberVal(_)) => Ok(()),
            (LiteralValue::FloatVal(_), LiteralValue::FloatVal(_)) => Ok(()),
            _ => Err(message.to_string())
        }
    }

    pub fn evaluate_expr(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate_expr(*value)?;
                self.environment.assign(&name, value.clone())?;
                Ok(value)
            },
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;
                match operator.lexeme.as_str() {
                    ">" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::BooleanVal(l > r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::BooleanVal(l > r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    "<" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::BooleanVal(l < r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::BooleanVal(l < r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    ">=" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::BooleanVal(l >= r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::BooleanVal(l >= r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    "<=" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::BooleanVal(l <= r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::BooleanVal(l <= r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    "==" => Ok(LiteralValue::BooleanVal(LiteralValue::is_equal(left.clone(), right.clone()))),
                    "!=" => Ok(LiteralValue::BooleanVal(!LiteralValue::is_equal(left.clone(), right.clone()))),
                    "+" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::NumberVal(l + r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::FloatVal(l + r)),
                            (LiteralValue::StringVal(l), LiteralValue::StringVal(r)) => Ok(LiteralValue::StringVal(format!("{}{}", l, r))),
                            _ => Err("Operands must be two numbers or two strings.".to_string())
                        }
                    },
                    "-" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::NumberVal(l - r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::FloatVal(l - r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    "*" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::NumberVal(l * r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::FloatVal(l * r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    "/" => {
                        let check_op = self.check_operands(left.clone(), right.clone(), "Operands must be two numbers.");
                        match check_op {
                            Ok(_) => (),
                            Err(e) => panic!("{}", e)
                        }
                        match (left, right) {
                            (LiteralValue::NumberVal(l), LiteralValue::NumberVal(r)) => Ok(LiteralValue::NumberVal(l / r)),
                            (LiteralValue::FloatVal(l), LiteralValue::FloatVal(r)) => Ok(LiteralValue::FloatVal(l / r)),
                            _ => Err("Operands must be two numbers.".to_string())
                        }
                    },
                    _ => Err("Invalid operator.".to_string())
                }
            },
            Expr::Grouping { expression } => self.evaluate_expr(*expression),
            Expr::Logical { left, operator, right } => {
                let left = self.evaluate_expr(*left)?;
                match operator.lexeme.as_str() {
                    "or" => {
                        if left.is_truthy() {
                            Ok(left)
                        } else {
                            self.evaluate_expr(*right)
                        }
                    },
                    "and" => {
                        if !left.is_truthy() {
                            Ok(left)
                        } else {
                            self.evaluate_expr(*right)
                        }
                    },
                    _ => Err("Invalid operator.".to_string())
                }
            },
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expr(*right)?;
                match (right, operator.lexeme.as_str()) {
                    (LiteralValue::NumberVal(r), "-") => Ok(LiteralValue::NumberVal(-r)),
                    (LiteralValue::FloatVal(r), "-") => Ok(LiteralValue::FloatVal(-r)),
                    (any, "!") => Ok(LiteralValue::BooleanVal(!any.is_truthy())),
                    _ => Err("Invalid operand.".to_string())
                }
            },
            Expr::Variable { name } => {
                match self.environment.get(&name) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e)
                }
            },

        }
    }
}