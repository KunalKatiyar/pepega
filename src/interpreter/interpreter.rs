use crate::lexer::token::LiteralValue;
use crate::parser::expr::Expr;
pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {

        }
    }

    pub fn interpret(&mut self, expr: Expr) {
        match self.evaluate(expr) {
            Ok(v) => println!("{}", v.to_string()),
            Err(e) => panic!("{}", e)
        }
    }

    pub fn check_operands(&mut self, left: LiteralValue, right: LiteralValue, message: &str) -> Result<(), String> {
        match (left, right) {
            (LiteralValue::NumberVal(_), LiteralValue::NumberVal(_)) => Ok(()),
            (LiteralValue::FloatVal(_), LiteralValue::FloatVal(_)) => Ok(()),
            _ => Err(message.to_string())
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        match expr {
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
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
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right)?;
                match (right, operator.lexeme.as_str()) {
                    (LiteralValue::NumberVal(r), "-") => Ok(LiteralValue::NumberVal(-r)),
                    (LiteralValue::FloatVal(r), "-") => Ok(LiteralValue::FloatVal(-r)),
                    (any, "!") => Ok(LiteralValue::BooleanVal(!any.is_truthy())),
                    _ => Err("Invalid operand.".to_string())
                }
            }
        }
    }
}