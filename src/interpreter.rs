//! Implements an interpreter for the Lox language.
//!
//! This module is responsible for evaluating an expression to a value.

use crate::ast::{DeclKind, Declaration, ExprKind, Expression, Statement, StmtKind, VarDecl};
use crate::environment::Environment;
use crate::error_reporter::{ErrorReporter, RuntimeError};
use crate::token::{Literal, Operator};

/// Represents a value to clarify difference between literal input and value output.
pub type Value = Literal;

/// The Lox Interpreter
pub struct Interpreter {
    /// Handles reporting of runtime errors
    pub error_reporter: ErrorReporter,
    pub environment_stack: Environment,
}

impl Interpreter {
    /// Creates a new Interpreter instance
    pub fn new() -> Self {
        Interpreter {
            error_reporter: ErrorReporter::new(),
            environment_stack: Environment::new(),
        }
    }

    pub fn evaluate_program(&mut self, program: &Vec<Declaration>) {
        for declaration in program {
            self.evaluate_declaration(declaration)
        }
    }

    fn evaluate_declaration(&mut self, declaration: &Declaration) {
        match &declaration.kind {
            DeclKind::VarDecl(var_decl) => self.evaluate_var_decl(var_decl),
            DeclKind::Statement(statement) => self.evaluate_statement(statement),
        }
    }

    fn evaluate_var_decl(&mut self, var_decl: &VarDecl) {
        let value = match &var_decl.initializer {
            Some(expression) => Some(self.evaluate_expression(expression)),
            None => None,
        };
        self.environment_stack
            .define(var_decl.identifier.clone(), value);
    }

    fn evaluate_statement(&mut self, statement: &Statement) {
        match &statement.kind {
            StmtKind::PrintStmt { expression } => {
                println!("{}", self.evaluate_expression(expression))
            }

            StmtKind::ExprStmt { expression } => {
                let _ = self.evaluate_expression(expression);
            }
            StmtKind::Block { declarations } => {
                self.environment_stack.increase_scope();
                for declaration in declarations {
                    self.evaluate_declaration(declaration);
                }
                if let Err(_) = self.environment_stack.reduce_scope() {
                    self.error_reporter.error(
                        statement.line,
                        statement.column,
                        "Trying to reduce scope but already at global",
                    );
                }
            }
        }
    }

    /// Evaluates an entire expression and returns a Value
    fn evaluate_expression(&mut self, expression: &Expression) -> Value {
        match &expression.kind {
            ExprKind::Lit { value } => value.clone(),
            ExprKind::Var { identifier } => {
                self.evaluate_var(identifier, expression.line, expression.column)
            }
            ExprKind::Grouping { expression } => self.evaluate_expression(expression),
            ExprKind::Unary { operator, right } => {
                self.evaluate_unary(operator, right, expression.line, expression.column)
            }
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right, expression.line, expression.column),
            ExprKind::Assignment { identifier, value } => {
                self.evaluate_assignment(identifier, value)
            }
        }
    }

    fn evaluate_var(&mut self, identifier: &str, line: usize, column: usize) -> Value {
        match self.environment_stack.get(identifier) {
            Ok(value) => value,
            Err(RuntimeError::UnInitializedVariable) => {
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Uninitialized Variable: {}", identifier),
                );
                Value::Nil
            }
            Err(_) => {
                self.error_reporter.error(
                    line,
                    column,
                    &format!("Undefined Variable: {}", identifier),
                );
                Value::Nil
            }
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
        let right_val = self.evaluate_expression(right);
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
        let left_val = self.evaluate_expression(left);
        let right_val = self.evaluate_expression(right);
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
                    "Cannot do binary operations on Boolean or Nil types",
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
                self.error_reporter
                    .error(line, column, "Cannot use comparators on non-numbers");
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
    fn evaluate_assignment(&mut self, identifier: &str, value: &Expression) -> Value {
        let evaluated_value = self.evaluate_expression(value);
        match self
            .environment_stack
            .assign(identifier, evaluated_value.clone())
        {
            Ok(()) => evaluated_value,
            Err(_) => {
                self.error_reporter.error(
                    value.line,
                    value.column,
                    &format!("Undefined variable '{}' in assignment.", identifier),
                );
                Value::Nil
            }
        }
    }
}
