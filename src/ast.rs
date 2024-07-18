//! Defines the ast structure for the Lox language.
//!
//! This module contains the `Program` Type, `Statement` struct and
//! `StmtKind` enum `Expression` struct and `ExprKind` enum,
//! which together represent the various types of statements and expressions
//! that can occur in Lox source code.

use crate::token::{Literal, Operator};

pub type Program = Vec<Statement>;

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub kind: StmtKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtType {
    Print,
    Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    ExprStmt { expression: Box<Expression> },
    PrintStmt { expression: Box<Expression> },
}

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
