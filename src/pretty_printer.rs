use crate::expression::{Expression, Visitor};
use crate::token::{Literal, Operator};

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter
    }
}

impl Visitor<String> for PrettyPrinter {
    fn visit_lit(&mut self, value: &Literal) -> String {
        match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }
    fn visit_grouping(&mut self, expression: &Box<Expression>) -> String {
        format!("(group {})", expression.accept(self))
    }

    fn visit_unary_expr(&mut self, operator: &Operator, right: &Box<Expression>) -> String {
        format!("({} {})", operator, right.accept(self))
    }

    fn visit_binary_expr(
        &mut self,
        left: &Box<Expression>,
        operator: &Operator,
        right: &Box<Expression>,
    ) -> String {
        format!(
            "({} {} {})",
            operator,
            left.accept(self),
            right.accept(self)
        )
    }
}
