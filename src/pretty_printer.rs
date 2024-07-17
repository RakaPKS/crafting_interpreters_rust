//! Provides functionality for pretty-printing Lox expressions.
//!
//! This module contains the `PrettyPrinter` struct, which can convert
//! Lox expressions into a readable string format for debugging or display purposes.

use crate::expression::{ExprKind, Expression};
use crate::token::{Literal, Operator};

/// A utility for converting Lox expressions into a readable string format.
pub struct PrettyPrinter;

impl PrettyPrinter {
    /// Creates a new `PrettyPrinter` instance.
    pub fn new() -> Self {
        PrettyPrinter
    }

    /// Converts an expression to its string representation.
    ///
    /// This method dispatches to the appropriate printing method based on the expression kind.
    pub fn print(&self, expr: &Expression) -> String {
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
        format!("(group {})", self.print(expression))
    }

    /// Prints a unary expression.
    fn print_unary(&self, operator: &Operator, right: &Expression) -> String {
        format!("({} {})", operator, self.print(right))
    }

    /// Prints a binary expression.
    fn print_binary(&self, left: &Expression, operator: &Operator, right: &Expression) -> String {
        format!("({} {} {})", operator, self.print(left), self.print(right))
    }
}
