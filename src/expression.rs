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

pub trait Visitor<T> {
    fn visit_lit(&mut self, value: &Literal) -> T;
    fn visit_grouping(&mut self, value: &Box<Expression>) -> T;
    fn visit_unary_expr(&mut self, operator: &Operator, right: &Box<Expression>) -> T;
    fn visit_binary_expr(
        &mut self,
        left: &Box<Expression>,
        operator: &Operator,
        right: &Box<Expression>,
    ) -> T;
}

impl Expression {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expression::Lit { value } => visitor.visit_lit(value),
            Expression::Grouping { expression } => visitor.visit_grouping(expression),
            Expression::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
        }
    }
}
