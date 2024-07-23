//! Provides functionality for pretty-printing Lox expressions and declarations.
//!
//! This module contains the `PrettyPrinter` struct, which can convert
//! Lox programs, declarations, statements, and expressions into a readable string format
//! for debugging or display purposes.
use crate::ast::{
    DeclKind, Declaration, ExprKind, Expression, Program, Statement, StmtKind, VarDecl,
};
use crate::token::{Literal, Operator, TokenType};

pub struct PrettyPrinter;

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter
    }

    pub fn print_program(&self, program: &Program) -> String {
        program
            .iter()
            .map(|decl| self.print_declaration(decl))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn print_declaration(&self, decl: &Declaration) -> String {
        match &decl.kind {
            DeclKind::VarDecl(var_decl) => self.print_var_decl(var_decl),
            DeclKind::Statement(stmt) => self.print_statement(stmt),
        }
    }

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

    pub fn print_statement(&self, stmt: &Statement) -> String {
        match &stmt.kind {
            StmtKind::ExprStmt { expression } => format!("{};", self.print_expression(expression)),
            StmtKind::PrintStmt { expression } => {
                format!("print {};", self.print_expression(expression))
            }
            StmtKind::Block { declarations } => self.print_block(declarations),
            StmtKind::IfStmt {
                condition,
                then_stmt,
                else_stmt,
            } => self.print_if_stmt(condition, then_stmt, else_stmt),
            StmtKind::WhileStmt { condition, do_stmt } => self.print_while_stmt(condition, do_stmt),
            StmtKind::ForStmt {
                initializer,
                condition,
                update,
                body,
            } => self.print_for_statement(initializer, condition, update, body),
        }
    }

    pub fn print_block(&self, declarations: &[Declaration]) -> String {
        let inner = declarations
            .iter()
            .map(|decl| self.print_declaration(decl))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{{\n{}\n}}",
            inner
                .lines()
                .map(|line| format!("  {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn print_while_stmt(&self, condition: &Expression, do_stmt: &Statement) -> String {
        format!(
            "while({}) {}",
            self.print_expression(condition),
            self.print_statement(do_stmt)
        )
    }
    fn print_for_statement(
        &self,
        initializer: &Option<Box<Declaration>>,
        condition: &Option<Box<Expression>>,
        update: &Option<Box<Expression>>,
        body: &Statement,
    ) -> String {
        let init_str = match initializer {
            Some(decl) => self.print_declaration(decl),
            None => String::new(),
        };

        let cond_str = match condition {
            Some(expr) => self.print_expression(expr),
            None => String::new(),
        };

        let update_str = match update {
            Some(expr) => self.print_expression(expr),
            None => String::new(),
        };

        let body_str = self.print_statement(body);

        format!(
            "for ({init}; {cond}; {update}) {body}",
            init = init_str.trim_end_matches(';'),
            cond = cond_str,
            update = update_str,
            body = body_str
        )
    }
    fn print_if_stmt(
        &self,
        condition: &Expression,
        then_stmt: &Statement,
        else_stmt: &Option<Box<Statement>>,
    ) -> String {
        let else_part = match else_stmt {
            Some(stmt) => format!(" else {}", self.print_statement(stmt)),
            None => String::new(),
        };
        format!(
            "if ({}) {}{}",
            self.print_expression(condition),
            self.print_statement(then_stmt),
            else_part
        )
    }

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
            ExprKind::Logical {
                left,
                logic_op,
                right,
            } => self.print_logical(left, logic_op, right),
            ExprKind::Assignment { identifier, value } => self.print_assignment(identifier, value),
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
        format!("(group {})", self.print_expression(expression))
    }

    fn print_unary(&self, operator: &Operator, right: &Expression) -> String {
        format!("({} {})", operator, self.print_expression(right))
    }

    fn print_binary(&self, left: &Expression, operator: &Operator, right: &Expression) -> String {
        format!(
            "({} {} {})",
            operator,
            self.print_expression(left),
            self.print_expression(right)
        )
    }

    fn print_logical(&self, left: &Expression, logic_op: &TokenType, right: &Expression) -> String {
        format!(
            "({} {} {})",
            self.print_expression(left),
            logic_op,
            self.print_expression(right)
        )
    }

    fn print_assignment(&self, identifier: &str, value: &Expression) -> String {
        format!("{} = {}", identifier, self.print_expression(value))
    }
}
