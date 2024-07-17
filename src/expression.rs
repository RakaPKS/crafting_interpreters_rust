use crate::token::{Literal, Operator};
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
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
