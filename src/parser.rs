//! Implements a recursive descent parser for the Lox language.
//!
//! This module is responsible for converting the tokens to a single big expression.
use crate::{
    ast::{DeclKind, Declaration, ExprKind, Expression, Program, Statement, StmtKind, VarDecl},
    error_reporter::{ErrorReporter, ParseError},
    token::{Operator, Token, TokenType},
};
use std::{iter::Peekable, slice::Iter};

/// The parser for Lox expressions.
///
/// Uses a peekable iterator.
pub struct Parser<'a> {
    token_iterator: Peekable<Iter<'a, Token>>,
    pub error_reporter: ErrorReporter,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance.    
    pub fn new(token_list: &'a [Token]) -> Self {
        Parser {
            token_iterator: token_list.iter().peekable(),
            error_reporter: ErrorReporter::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = vec![];
        while self.token_iterator.peek().is_some() {
            match self.parse_declaration() {
                Ok(declaration) => program.push(declaration),
                Err(_) => match self.synchronize() {
                    Err(ParseError::UnexpectedEOF) => break,
                    _ => {}
                },
            }
        }
        program
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        match self.search(&[TokenType::Var]) {
            Some(_) => self.parse_var_decl().map(|var_decl| {
                let line = var_decl.line;
                let column = var_decl.column;
                Declaration {
                    kind: DeclKind::VarDecl(var_decl),
                    line,
                    column,
                }
            }),
            None => self.parse_statement().map(|statement| {
                let line = statement.line;
                let column = statement.column;
                Declaration {
                    kind: DeclKind::Statement(statement),
                    line,
                    column,
                }
            }),
        }
    }

    pub fn parse_var_decl(&mut self) -> Result<VarDecl, ParseError> {
        let var_keyword = self.consume(TokenType::Var, "Expected 'var'")?;
        let line = var_keyword.line;
        let column = var_keyword.column;

        match self.token_iterator.next() {
            Some(token) if token.token_type == TokenType::Identifier => {
                match self.search(&[TokenType::Operator(Operator::Equal), TokenType::Semicolon]) {
                    Some(TokenType::Operator(Operator::Equal)) => {
                        self.token_iterator.next();

                        let expression = self.parse_assignment()?;
                        self.consume(
                            TokenType::Semicolon,
                            "Expect ';' after variable declaration.",
                        )?;
                        Ok(VarDecl {
                            identifier: token.lexeme.clone(),
                            initializer: Some(expression),
                            line,
                            column,
                        })
                    }
                    Some(TokenType::Semicolon) => {
                        self.token_iterator.next();
                        Ok(VarDecl {
                            identifier: token.lexeme.clone(),
                            initializer: None,
                            line,
                            column,
                        })
                    }
                    _ => Err(ParseError::UnexpectedToken),
                }
            }
            Some(_) => Err(ParseError::UnexpectedToken),
            _ => Err(ParseError::UnexpectedEOF),
        }
    }
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let search_tokens = vec![
            TokenType::Print,
            TokenType::LeftBrace,
            TokenType::While,
            TokenType::For,
            TokenType::If,
        ];
        match self.search(&search_tokens) {
            Some(TokenType::Print) => self.parse_print_statement(),
            Some(TokenType::LeftBrace) => self.parse_block(),
            Some(TokenType::If) => self.parse_if_statement(),
            Some(TokenType::While) => self.parse_while_statement(),
            Some(TokenType::For) => self.parse_for_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    fn parse_print_statement(&mut self) -> Result<Statement, ParseError> {
        let print_keyword = self.consume(TokenType::Print, "Expected 'print'")?;
        let line = print_keyword.line;
        let column = print_keyword.column;
        let expression = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after Expression.")?;
        Ok(Statement {
            kind: StmtKind::PrintStmt {
                expression: Box::new(expression),
            },
            line,
            column,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        let while_keyword = self.consume(TokenType::While, "Expected 'while'")?;
        let line = while_keyword.line;
        let column = while_keyword.column;
        self.consume(TokenType::LeftParen, "Expected '(' after while")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after while condition")?;
        let do_stmt = self.parse_statement()?;
        Ok(Statement {
            kind: StmtKind::WhileStmt {
                condition: Box::new(condition),
                do_stmt: Box::new(do_stmt),
            },
            line,
            column,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParseError> {
        let for_keyword = self.consume(TokenType::For, "Expected 'for'")?;
        let line = for_keyword.line;
        let column = for_keyword.column;
        self.consume(TokenType::LeftParen, "Expected '(' after for")?;
        let initializer = if self.check(TokenType::Semicolon) {
            self.token_iterator.next(); // Consume the semicolon
            None
        } else {
            Some(Box::new(self.parse_declaration()?))
        };

        let condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after for loop condition",
        )?;

        let update = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement {
            kind: StmtKind::ForStmt {
                initializer,
                condition,
                update,
                body,
            },
            line,
            column,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        let if_keyword = self.consume(TokenType::If, "Expected 'if'")?;
        let line = if_keyword.line;
        let column = if_keyword.column;
        self.consume(TokenType::LeftParen, "Expected '(' after if")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition")?;
        let then_stmt = self.parse_statement()?;
        let mut else_stmt: Option<Box<Statement>> = None;
        if self.search(&[TokenType::Else]) == Some(TokenType::Else) {
            self.token_iterator.next();
            else_stmt = Some(Box::new(self.parse_statement()?));
        }
        Ok(Statement {
            kind: StmtKind::IfStmt {
                condition: Box::new(condition),
                then_stmt: Box::new(then_stmt),
                else_stmt,
            },
            line,
            column,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression()?;
        let line = expression.line;
        let column = expression.column;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement {
            kind: StmtKind::ExprStmt {
                expression: Box::new(expression),
            },
            line,
            column,
        })
    }

    fn parse_block(&mut self) -> Result<Statement, ParseError> {
        let brace = self.consume(TokenType::LeftBrace, "Expected '('")?;
        let line = brace.line;
        let column = brace.column;
        let mut declarations = Vec::new();

        while !self.check(TokenType::RightBrace) && self.token_iterator.peek().is_some() {
            declarations.push(self.parse_declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(Statement {
            kind: StmtKind::Block { declarations },
            line,
            column,
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.logical()?;

        if let Some(TokenType::Operator(Operator::Equal)) =
            self.search(&[TokenType::Operator(Operator::Equal)])
        {
            self.token_iterator.next(); // Consume the '=' token
            let value = self.parse_assignment()?;

            if let ExprKind::Var { identifier } = expr.kind {
                return Ok(self.create_expression(ExprKind::Assignment {
                    identifier,
                    value: Box::new(value),
                }));
            }

            self.error_reporter
                .error(expr.line, expr.column, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn logical(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;
        while let Some(token_type) = self.search(&[TokenType::And, TokenType::Or]) {
            self.token_iterator.next();
            let right = self.equality()?;
            expr = self.create_expression(ExprKind::Logical {
                left: Box::new(expr),
                logic_op: token_type,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    /// Helper method for parsing binary operations.
    ///
    /// This method is used by various parsing methods to handle binary operations
    /// at different precedence levels.

    fn binary_op<F>(
        &mut self,
        mut left: Expression,
        operators: &[TokenType],
        next_precedence: F,
    ) -> Result<Expression, ParseError>
    where
        F: Fn(&mut Self) -> Result<Expression, ParseError>,
    {
        while let Some(TokenType::Operator(op)) = self.search(operators) {
            if let Some(token) = self.token_iterator.next() {
                let right = next_precedence(self)?;
                left = self.create_expression(ExprKind::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                });
                left.line = token.line;
                left.column = token.column;
            } else {
                return Err(ParseError::UnexpectedToken);
            }
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let left = self.comparison()?;
        self.binary_op(
            left,
            &[
                TokenType::Operator(Operator::BangEqual),
                TokenType::Operator(Operator::EqualEqual),
            ],
            Self::comparison,
        )
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let left = self.term()?;
        self.binary_op(
            left,
            &[
                TokenType::Operator(Operator::Greater),
                TokenType::Operator(Operator::GreaterEqual),
                TokenType::Operator(Operator::Less),
                TokenType::Operator(Operator::LessEqual),
            ],
            Self::term,
        )
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let left = self.factor()?;
        self.binary_op(
            left,
            &[
                TokenType::Operator(Operator::Minus),
                TokenType::Operator(Operator::Plus),
            ],
            Self::factor,
        )
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let left = self.unary()?;
        self.binary_op(
            left,
            &[
                TokenType::Operator(Operator::Slash),
                TokenType::Operator(Operator::Star),
            ],
            Self::unary,
        )
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        let search_types = [
            TokenType::Operator(Operator::Bang),
            TokenType::Operator(Operator::Minus),
        ];
        if let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::Bang | Operator::Minus => {
                    self.token_iterator.next(); // Consume the token
                    let right = self.unary()?;
                    Ok(self.create_expression(ExprKind::Unary {
                        operator: op,
                        right: Box::new(right),
                    }))
                }
                _ => {
                    let token = self.token_iterator.peek().unwrap();
                    self.error_reporter.error(
                        token.line,
                        token.column,
                        "Unexpected operator in unary expression",
                    );
                    Err(ParseError::UnexpectedToken)
                }
            }
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.token_iterator.next().ok_or_else(|| {
            self.error_reporter.error(0, 0, "Unexpected end of input");
            ParseError::UnexpectedToken
        })?;

        match token.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number
            | TokenType::String => {
                let value = token.literal.clone().ok_or_else(|| {
                    self.error_reporter
                        .error(token.line, token.column, "Expected literal value");
                    ParseError::UnexpectedToken
                })?;
                Ok(self.create_expression(ExprKind::Lit { value }))
            }
            TokenType::Identifier => Ok(self.create_expression(ExprKind::Var {
                identifier: token.lexeme.clone(),
            })),
            TokenType::LeftParen => {
                let expression = self.parse_expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(self.create_expression(ExprKind::Grouping {
                    expression: Box::new(expression),
                }))
            }
            _ => {
                self.error_reporter.error(
                    token.line,
                    token.column,
                    &format!("Unexpected token: {:?}", token.token_type),
                );
                Err(ParseError::UnexpectedToken)
            }
        }
    }
    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.token_iterator.next().unwrap())
        } else {
            if let Some(token) = self.token_iterator.peek() {
                self.error_reporter
                    .error(token.line, token.column, error_message);
            } else {
                self.error_reporter.error(0, 0, "Unexpected end of input");
                return Err(ParseError::MissingToken);
            }
            Err(ParseError::UnexpectedToken)
        }
    }

    /// Creates an Expression with the current token's line and column information.    
    fn create_expression(&mut self, kind: ExprKind) -> Expression {
        let (line, column) = if let Some(token) = self.token_iterator.peek() {
            (token.line, token.column)
        } else {
            (0, 0)
        };
        Expression { kind, line, column }
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.token_iterator
            .peek()
            .map_or(false, |t| t.token_type == token_type)
    }

    fn search(&mut self, search_types: &[TokenType]) -> Option<TokenType> {
        if let Some(token) = self.token_iterator.peek() {
            let token_type = token.token_type.clone();
            if search_types.contains(&token_type) {
                return Some(token_type);
            }
        }
        None
    }

    /// Synchronizes the parser to a useable state after encountering an error.
    fn synchronize(&mut self) -> Result<(), ParseError> {
        while let Some(token) = self.token_iterator.next() {
            if token.token_type == TokenType::Semicolon {
                self.token_iterator.next();
                return Ok(());
            }

            if let Some(next_token) = self.token_iterator.peek() {
                match next_token.token_type {
                    TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::LeftBrace
                    | TokenType::Return => return Ok(()),
                    _ => {}
                }
            }
        }

        self.error_reporter.error(0, 0, "Unexpected End of File.");
        Err(ParseError::UnexpectedEOF)
    }
}
