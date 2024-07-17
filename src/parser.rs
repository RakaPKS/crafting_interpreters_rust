//! Implements a recursive descent parser for the Lox language.
//!
//! This module is responsible for converting the tokens to a single big expression.
use crate::{
    error_reporter::ErrorReporter,
    expression::{ExprKind, Expression},
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
            let token = self.token_iterator.next().unwrap(); // Consume the operator
            let right = next_precedence(self)?;
            left = self.create_expression(ExprKind::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });
            left.line = token.line;
            left.column = token.column;
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
        let search_types = [
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
            TokenType::LeftParen,
        ];
        if let Some(token_type) = self.search(&search_types) {
            match token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number
                | TokenType::String => {
                    let token = self.token_iterator.next().unwrap();
                    let value = token.literal.clone().unwrap();
                    Ok(self.create_expression(ExprKind::Lit { value }))
                }
                TokenType::LeftParen => {
                    self.token_iterator.next().unwrap();
                    let expression = self.parse_expression();
                    if self.search(&[TokenType::RightParen]).is_some() {
                        self.token_iterator.next();
                        Ok(self.create_expression(ExprKind::Grouping {
                            expression: Box::new(expression),
                        }))
                    } else {
                        let token = self.token_iterator.peek().unwrap();
                        self.error_reporter.error(
                            token.line,
                            token.column,
                            "Expect ')' after expression.",
                        );
                        Err(ParseError::MissingToken())
                    }
                }
                _ => unreachable!("search should only return primary Operators"),
            }
        } else {
            let token = self.token_iterator.peek().unwrap();
            self.error_reporter.error(
                token.line,
                token.column,
                &format!("Unexpected token: {:?}", token.token_type),
            );
            Err(ParseError::UnexpectedToken())
        }
    }

    /// Creates an Expression with the current token's line and column information.    
    fn create_expression(&mut self, kind: ExprKind) -> Expression {
        let token = self.token_iterator.peek().unwrap();
        Expression {
            kind,
            line: token.line,
            column: token.column,
        }
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
                    _ => {}
                }
            }
        }
    }
}
