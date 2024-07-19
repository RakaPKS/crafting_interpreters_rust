//! Provides functionality for pretty-printing Lox expressions and declarations.
//!
//! This module contains the `PrettyPrinter` struct, which can convert
//! Lox programs, declarations, statements, and expressions into a readable string format
//! for debugging or display purposes.

use crate::ast::{
    DeclKind, Declaration, ExprKind, Expression, Program, Statement, StmtKind, VarDecl,
};
use crate::token::{Literal, Operator};

/// A utility for converting Lox programs, declarations, statements, and expressions into a readable string format.
pub struct PrettyPrinter;

impl PrettyPrinter {
    /// Creates a new `PrettyPrinter` instance.
    pub fn new() -> Self {
        PrettyPrinter
    }

    /// Prints an entire Lox program.
    pub fn print_program(&self, program: &Program) -> String {
        program
            .iter()
            .map(|decl| self.print_declaration(decl))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Prints a declaration.
    pub fn print_declaration(&self, decl: &Declaration) -> String {
        match &decl.kind {
            DeclKind::VarDecl(var_decl) => self.print_var_decl(var_decl),
            DeclKind::Statement(stmt) => self.print_statement(stmt),
        }
    }

    /// Prints a variable declaration.
    pub fn print_var_decl(&self, var_decl: &VarDecl) -> String {
        match &var_decl.initializer {
            Some(expr) => format!(
                "var {} = {};",
                var_decl.identifier,
                self.print_expression(expr)
            ),
            None => format!("var {};", var_decl.identifier),
        }
    }

    /// Prints a statement.
    pub fn print_statement(&self, stmt: &Statement) -> String {
        match &stmt.kind {
            StmtKind::ExprStmt { expression } => format!("{};", self.print_expression(expression)),
            StmtKind::PrintStmt { expression } => {
                format!("print {};", self.print_expression(expression))
            }
        }
    }

    /// Converts an expression to its string representation.
    pub fn print_expression(&self, expr: &Expression) -> String {
        match &expr.kind {
            ExprKind::Lit { value } => self.print_literal(value),
            ExprKind::Var { identifier } => identifier.clone(),
            ExprKind::Grouping { expression } => self.print_grouping(expression),
            ExprKind::Unary { operator, right } => self.print_unary(operator, right),
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.print_binary(left, operator, right),
            ExprKind::Assignment { identifier, value } => self.print_assignment(identifier, value),
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

    /// Prints an assignment expression.
    fn print_assignment(&self, identifier: &str, value: &Expression) -> String {
        format!("{} = {}", identifier, self.print_expression(value))
    }
}

