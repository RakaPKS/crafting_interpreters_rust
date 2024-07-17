//! Implements an interpreter for the Lox language.
//!
//! This module is responsible for evaluating an expression to a value.
use crate::error_reporter::ErrorReporter;
use crate::expression::{ExprKind, Expression};
use crate::token::{Literal, Operator};

/// Represents a value to clarify difference between literal input and value output.
pub type Value = Literal;

/// The Lox Interpreter
pub struct Interpreter {
    /// Handles reporting of runtime errors
    pub error_reporter: ErrorReporter,
}

impl Interpreter {
    /// Creates a new Interpreter instance
    pub fn new() -> Self {
        Interpreter {
            error_reporter: ErrorReporter::new(),
        }
    }

    /// Evaluates an entire expression and returns a Value
    pub fn evaluate(&mut self, expression: &Expression) -> Value {
        match &expression.kind {
            ExprKind::Lit { value } => value.clone(),
            ExprKind::Grouping { expression } => self.evaluate(expression),
            ExprKind::Unary { operator, right } => {
                self.evaluate_unary(operator, right, expression.line, expression.column)
            }
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right, expression.line, expression.column),
        }
    }

    /// Evaluates a unary expression.
    fn evaluate_unary(
        &mut self,
        operator: &Operator,
        right: &Expression,
        line: usize,
        column: usize,
    ) -> Value {
        let right_val = self.evaluate(right);
        match operator {
            Operator::Bang => Value::Boolean(!self.is_truthy(&right_val)),
            Operator::Minus => match right_val {
                Value::Number(n) => Value::Number(-n),
                _ => {
                    self.error_reporter.error(
                        line,
                        column,
                        &format!("{}, is not a number", right_val),
                    );
                    Value::Nil
                }
            },
            _ => {
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Using {} as unary operator not allowed.", operator),
                );
                Value::Nil
            }
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expression,
        operator: &Operator,
        right: &Expression,
        line: usize,
        column: usize,
    ) -> Value {
        let left_val = self.evaluate(left);
        let right_val = self.evaluate(right);
        match operator {
            Operator::Minus | Operator::Plus | Operator::Star | Operator::Slash => {
                self.evaluate_arithmetic(left_val, operator, right_val, line, column)
            }
            Operator::Greater | Operator::GreaterEqual | Operator::Less | Operator::LessEqual => {
                self.evaluate_comparator(left_val, operator, right_val, line, column)
            }
            Operator::EqualEqual | Operator::BangEqual => {
                self.evaluate_equals(left_val, operator, right_val)
            }
            _ => {
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Using {} as a binary operator is not allowed", operator),
                );
                Value::Nil
            }
        }
    }

    /// Determines if a value is true in Lox.
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
        line: usize,
        column: usize,
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
                    self.error_reporter.error(
                        line,
                        column,
                        &format!(
                            "Using {} on strings [{}, {}] is not allowed",
                            operator, l, r
                        ),
                    );
                    Value::Nil
                }
            },
            _ => {
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Cannot do binary operations on Boolean or Nil types"),
                );
                Value::Nil
            }
        }
    }

    fn evaluate_comparator(
        &mut self,
        left_val: Value,
        operator: &Operator,
        right_val: Value,
        line: usize,
        column: usize,
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
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Cannot use comparators on non-numbers"),
                );
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
