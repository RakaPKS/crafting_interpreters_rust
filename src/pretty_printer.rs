//! Provides functionality for pretty-printing Lox expressions.
//!
//! This module contains the `PrettyPrinter` struct, which can convert
//! Lox expressions into a readable string format for debugging or display purposes.

use crate::ast::{ExprKind, Expression, Program, Statement, StmtKind};
use crate::token::{Literal, Operator};

/// A utility for converting Lox expressions into a readable string format.
pub struct PrettyPrinter;

impl PrettyPrinter {
    /// Creates a new `PrettyPrinter` instance.
    pub fn new() -> Self {
        PrettyPrinter
    }

    pub fn print_program(&self, program: &Vec<Statement>) -> String {
        program
            .iter()
            .map(|stmt| self.print_statement(stmt))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn print_statement(&self, stmt: &Statement) -> String {
        match &stmt.kind {
            StmtKind::ExprStmt { expression } => self.print_expression(expression),
            StmtKind::PrintStmt { expression } => {
                format!("print {};", self.print_expression(expression))
            }
        }
    }

    /// Converts an expression to its string representation.
    ///
    /// This method dispatches to the appropriate printing method based on the expression kind.
    pub fn print_expression(&self, expr: &Expression) -> String {
        match &expr.kind {
            ExprKind::Lit { value } => self.print_literal(value),
            ExprKind::Grouping { expression } => self.print_grouping(expression),
            ExprKind::Unary { operator, right } => self.print_unary(operator, right),
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.print_binary(left, operator, right),
        }
    }

    /// Converts a literal value to its string representation.
    fn print_literal(&self, value: &Literal) -> String {
        match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    /// Prints a grouping expression, wrapping it in parentheses.
    fn print_grouping(&self, expression: &Expression) -> String {
        format!("(group {})", self.print_expression(expression))
    }

    /// Prints a unary expression.
    fn print_unary(&self, operator: &Operator, right: &Expression) -> String {
        format!("({} {})", operator, self.print_expression(right))
    }

    /// Prints a binary expression.
    fn print_binary(&self, left: &Expression, operator: &Operator, right: &Expression) -> String {
        format!(
            "({} {} {})",
            operator,
            self.print_expression(left),
            self.print_expression(right)
        )
    }
}
