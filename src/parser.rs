//! Implements a recursive descent parser for the Lox language.
//!
//! This module is responsible for converting the tokens to a single big expression.
use crate::{
    ast::{ExprKind, Expression, Program, Statement, StmtKind, StmtType},
    error_reporter::ErrorReporter,
    token::{Literal, Operator, Token, TokenType},
};
use std::{iter::Peekable, slice::Iter};

/// Represents errors that can occur during parsing.
pub enum ParseError {
    UnexpectedToken(),
    MissingToken(),
}

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
            program.push(self.parse_statement());
        }
        program
    }

    pub fn parse_statement(&mut self) -> Statement {
        match self.search(&[TokenType::Print]) {
            Some(_) => {
                self.token_iterator.next();
                self.parse_stmt(StmtType::Print)
            }
            None => self.parse_stmt(StmtType::Expression),
        }
    }

    pub fn parse_stmt(&mut self, stmt_type: StmtType) -> Statement {
        let expression = self.parse_expression();
        let line = expression.line;
        let column = expression.column;
        match self.search(&[TokenType::Semicolon]) {
            Some(_) => {
                self.token_iterator.next();
                Statement {
                    kind: match stmt_type {
                        StmtType::Print => StmtKind::PrintStmt {
                            expression: Box::new(expression),
                        },
                        StmtType::Expression => StmtKind::ExprStmt {
                            expression: Box::new(expression),
                        },
                    },
                    line,
                    column,
                }
            }
            None => {
                self.error_reporter
                    .error(line, column, "Expected ; after expression.");
                self.synchronize();
                if self.token_iterator.peek().is_some() {
                    self.parse_statement()
                } else {
                    Statement {
                        kind: StmtKind::ExprStmt {
                            expression: Box::new(expression),
                        },
                        line,
                        column,
                    }
                }
            }
        }
    }

    /// Parses an expression. This is the entry point of the Lox Parser.
    pub fn parse_expression(&mut self) -> Expression {
        match self.equality() {
            Ok(expr) => expr,
            Err(_) => {
                self.synchronize();
                self.create_expression(ExprKind::Lit {
                    value: Literal::Nil,
                })
            }
        }
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
                return Err(ParseError::UnexpectedToken());
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
                    Err(ParseError::UnexpectedToken())
                }
            }
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.token_iterator.next().ok_or_else(|| {
            self.error_reporter.error(0, 0, "Unexpected end of input");
            ParseError::UnexpectedToken()
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
                    ParseError::UnexpectedToken()
                })?;
                Ok(self.create_expression(ExprKind::Lit { value }))
            }
            TokenType::LeftParen => {
                let expression = self.parse_expression();
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
                Err(ParseError::UnexpectedToken())
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
                return Err(ParseError::MissingToken());
            }
            Err(ParseError::UnexpectedToken())
        }
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.token_iterator
            .peek()
            .map_or(false, |t| t.token_type == token_type)
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
    fn synchronize(&mut self) {
        while let Some(token) = self.token_iterator.next() {
            if token.token_type == TokenType::Semicolon {
                self.token_iterator.next();
                return;
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
                    | TokenType::Return => return,
                    _ => {
                        self.token_iterator.next();
                    }
                }
            }
        }
    }
}
