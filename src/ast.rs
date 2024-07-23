//! Defines the ast structure for the Lox language.
//!
//! This module contains the `Program` Type, `Statement` struct and
//! `StmtKind` enum `Expression` struct and `ExprKind` enum,
//! which together represent the various types of statements and expressions
//! that can occur in Lox source code.

use crate::token::{Literal, Operator, TokenType};

pub type Program = Vec<Declaration>;

#[derive(Clone, Debug)]
pub struct Declaration {
    pub kind: DeclKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub identifier: String,
    pub initializer: Option<Expression>,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub kind: StmtKind,
    pub line: usize,
    pub column: usize,
}

/// Represents an expression in the Lox language.
///
/// An expression is a combination of values, variables, operators,
/// and function calls that can be evaluated to produce a value.
#[derive(Clone, Debug)]
pub struct Expression {
    /// The specific kind of expression.
    pub kind: ExprKind,
    /// The line number where this expression appears in the source code.
    pub line: usize,
    /// The column number where this expression starts in the source code.
    pub column: usize,
}

#[derive(Clone, Debug)]
pub enum StmtKind {
    ExprStmt {
        expression: Box<Expression>,
    },
    IfStmt {
        condition: Box<Expression>,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    WhileStmt {
        condition: Box<Expression>,
        do_stmt: Box<Statement>,
    },
    ForStmt {
        initializer: Option<Box<Declaration>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    PrintStmt {
        expression: Box<Expression>,
    },
    Block {
        declarations: Vec<Declaration>,
    },
}

#[derive(Clone, Debug)]
pub enum DeclKind {
    VarDecl(VarDecl),
    Statement(Statement),
}

/// Enumerates the different kinds of expressions in Lox.
#[derive(Clone, Debug)]
pub enum ExprKind {
    // Highest precedence
    Lit {
        value: Literal,
    },
    Var {
        identifier: String,
    },
    Grouping {
        expression: Box<Expression>,
    },
    // High precedence
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
    // Medium precedence
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    // Lower precedence
    Logical {
        left: Box<Expression>,
        logic_op: TokenType,
        right: Box<Expression>,
    },
    // Lowest precedence
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
}
