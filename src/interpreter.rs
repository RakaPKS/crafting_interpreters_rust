use crate::error_reporter::ErrorReporter;
use crate::expression::Expression;
use crate::token::{Literal, Operator};

pub type Value = Literal;

pub struct Interpreter {
    pub error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            error_reporter: ErrorReporter::new(),
        }
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Lit { value } => value.clone(),
            Expression::Grouping { expression } => self.evaluate(expression),
            Expression::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
        }
    }

    fn evaluate_unary(&mut self, operator: &Operator, right: &Expression) -> Value {
        let right_val = self.evaluate(right);
        match operator {
            Operator::Bang => Value::Boolean(!self.is_truthy(&right_val)),
            Operator::Minus => match right_val {
                Value::Number(n) => Value::Number(-n),
                _ => {
                    self.error_reporter
                        .interpreter_error(&format!("{}, is not a number", right_val));
                    Value::Nil
                }
            },
            _ => {
                self.error_reporter.interpreter_error(&format!(
                    "Using {} as unary operator not allowed.",
                    operator
                ));
                Value::Nil
            }
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expression,
        operator: &Operator,
        right: &Expression,
    ) -> Value {
        let left_val = self.evaluate(left);
        let right_val = self.evaluate(right);
        match operator {
            Operator::Minus | Operator::Plus | Operator::Star | Operator::Slash => {
                self.evaluate_arithmetic(left_val, operator, right_val)
            }
            Operator::Greater | Operator::GreaterEqual | Operator::Less | Operator::LessEqual => {
                self.evaluate_comparator(left_val, operator, right_val)
            }

            Operator::EqualEqual | Operator::BangEqual => {
                self.evaluate_equals(left_val, operator, right_val)
            }
            _ => {
                self.error_reporter.interpreter_error(&format!(
                    "Using {} as a unary operator is not allowed",
                    operator
                ));
                Value::Nil
            }
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(n) => *n,
            _ => true,
        }
    }

    fn evaluate_arithmetic(
        &mut self,
        left_val: Value,
        operator: &Operator,
        right_val: Value,
    ) -> Value {
        match (left_val, right_val) {
            (Value::Number(l), Value::Number(r)) => match operator {
                Operator::Minus => Value::Number(l - r),
                Operator::Plus => Value::Number(l + r),
                Operator::Slash => Value::Number(l / r),
                Operator::Star => Value::Number(l * r),
                _ => unreachable!("Operator is not part of arithmetic"),
            },
            (Value::String(l), Value::String(r)) => match operator {
                Operator::Plus => Value::String(format!("{}{}", l, r)),
                _ => {
                    self.error_reporter.interpreter_error(&format!(
                        "Using {} on strings [{}, {}] is not allowed",
                        operator, l, r
                    ));
                    Value::Nil
                }
            },
            _ => {
                self.error_reporter.interpreter_error(&format!(
                    "Cannot do binary operations on Boolean or Nil types"
                ));
                Value::Nil
            }
        }
    }

    fn evaluate_comparator(
        &mut self,
        left_val: Value,
        operator: &Operator,
        right_val: Value,
    ) -> Value {
        match (left_val, right_val) {
            (Value::Number(l), Value::Number(r)) => match operator {
                Operator::Greater => Value::Boolean(l > r),
                Operator::GreaterEqual => Value::Boolean(l >= r),
                Operator::Less => Value::Boolean(l < r),
                Operator::LessEqual => Value::Boolean(l <= r),
                _ => unreachable!("Operator is not part of Comparators"),
            },
            _ => {
                self.error_reporter
                    .interpreter_error(&format!("Cannot use comparators on non-numbers"));
                Value::Nil
            }
        }
    }

    fn evaluate_equals(&self, left_val: Value, operator: &Operator, right_val: Value) -> Value {
        match operator {
            Operator::BangEqual => Value::Boolean(left_val != right_val),
            Operator::EqualEqual => Value::Boolean(left_val == right_val),
            _ => unreachable!("Operator is not part of Equality"),
        }
    }
}
