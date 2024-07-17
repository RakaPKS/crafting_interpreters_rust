use crate::token::{Literal, Operator};
#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub kind: ExprKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Lit {
        value: Literal,
    },
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



