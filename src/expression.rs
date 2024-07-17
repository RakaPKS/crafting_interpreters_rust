//! Defines the structure for expressions in the Lox language.
//!
//! This module contains the `Expression` struct and `ExprKind` enum,
//! which together represent the various types of expressions that can
//! occur in Lox source code.

use crate::token::{Literal, Operator};

/// Represents an expression in the Lox language.
///
/// An expression is a combination of values, variables, operators,
/// and function calls that can be evaluated to produce a value.
#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    /// The specific kind of expression.
    pub kind: ExprKind,
    /// The line number where this expression appears in the source code.
    pub line: usize,
    /// The column number where this expression starts in the source code.
    pub column: usize,
}

/// Enumerates the different kinds of expressions in Lox.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Lit {
        value: Literal,
    },
    /// A parenthesized expression.
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
