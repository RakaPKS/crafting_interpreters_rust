use crate::token::Operator;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Literal,
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
