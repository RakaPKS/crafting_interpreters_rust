use crate::expression::{ExprKind, Expression};
use crate::token::{Literal, Operator};

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter
    }

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

    fn print_literal(&self, value: &Literal) -> String {
        match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    fn print_grouping(&self, expression: &Expression) -> String {
        format!("(group {})", self.print(expression))
    }

    fn print_unary(&self, operator: &Operator, right: &Expression) -> String {
        format!("({} {})", operator, self.print(right))
    }

    fn print_binary(&self, left: &Expression, operator: &Operator, right: &Expression) -> String {
        format!("({} {} {})", operator, self.print(left), self.print(right))
    }
}

