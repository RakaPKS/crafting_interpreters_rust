use crate::{
    error_reporter::ErrorReporter,
    expression::Expression,
    token::{Literal, Operator, Token, TokenType},
};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    token_iterator: Peekable<Iter<'a, Token>>,
    error_reporter: ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(token_list: &'a Vec<Token>, error_reporter: ErrorReporter) -> Self {
        Parser {
            token_iterator: token_list.iter().peekable(),
            error_reporter,
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.equality()
    }
    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();
        let search_types = vec![
            TokenType::Operator(Operator::BangEqual),
            TokenType::Operator(Operator::EqualEqual),
        ];
        while let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::BangEqual | Operator::EqualEqual => {
                    self.token_iterator.next(); // Consume the token
                    let right: Expression = self.comparison();
                    expression = Expression::Binary {
                        left: Box::new(expression),
                        operator: op,
                        right: Box::new(right),
                    }
                }
                _ => unreachable!("search() should only return Equality Operators"),
            }
        }
        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression: Expression = self.term();
        let search_types = vec![
            TokenType::Operator(Operator::Greater),
            TokenType::Operator(Operator::GreaterEqual),
            TokenType::Operator(Operator::Less),
            TokenType::Operator(Operator::LessEqual),
        ];
        while let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::Greater
                | Operator::GreaterEqual
                | Operator::Less
                | Operator::LessEqual => {
                    self.token_iterator.next(); // Consume the token
                    let right: Expression = self.term();
                    expression = Expression::Binary {
                        left: Box::new(expression),
                        operator: op,
                        right: Box::new(right),
                    }
                }
                _ => unreachable!("search() should only return Comparison Operators"),
            }
        }
        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression: Expression = self.factor();
        let search_types = vec![
            TokenType::Operator(Operator::Minus),
            TokenType::Operator(Operator::Plus),
        ];
        while let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::Minus | Operator::Plus => {
                    self.token_iterator.next(); // Consume the token
                    let right: Expression = self.factor();
                    expression = Expression::Binary {
                        left: Box::new(expression),
                        operator: op,
                        right: Box::new(right),
                    }
                }
                _ => unreachable!("search() should only return Term Operators"),
            }
        }
        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression: Expression = self.unary();
        let search_types = vec![
            TokenType::Operator(Operator::Slash),
            TokenType::Operator(Operator::Star),
        ];
        while let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::Slash | Operator::Star => {
                    self.token_iterator.next(); // Consume the token
                    let right: Expression = self.unary();
                    expression = Expression::Binary {
                        left: Box::new(expression),
                        operator: op,
                        right: Box::new(right),
                    }
                }
                _ => unreachable!("search() should only return Factor Operators"),
            }
        }
        expression
    }

    fn unary(&mut self) -> Expression {
        let search_types = vec![
            TokenType::Operator(Operator::Bang),
            TokenType::Operator(Operator::Minus),
        ];
        if let Some(TokenType::Operator(op)) = self.search(&search_types) {
            match op {
                Operator::Bang | Operator::Minus => {
                    self.token_iterator.next(); // Consume the token
                    let right: Expression = self.unary();
                    return Expression::Unary {
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => unreachable!("search() should only return Unary Operators"),
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Expression {
        let search_types = vec![
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
                    let value = token.get_literal().unwrap();
                    return Expression::Lit { value };
                }
                TokenType::LeftParen => {
                    let token = self.token_iterator.next().unwrap();
                    let expression: Expression = self.parse_expression();
                    if let Some(_) = self.search(&vec![TokenType::RightParen]) {
                        self.token_iterator.next();
                        return Expression::Grouping {
                            expression: Box::new(expression),
                        };
                    } else {
                        self.error_reporter.error(
                            token.get_line(),
                            token.get_column(),
                            "Expect ') after expression.",
                        );
                        expression
                    }
                }
                _ => unreachable!("search should only return primary Operators"),
            }
        } else {
            let token = self.token_iterator.peek().unwrap();
            self.error_reporter.error(
                token.get_line(),
                token.get_column(),
                &format!("Unexpected token: {:?}", token.get_token_type()),
            );
            Expression::Lit {
                value: Literal::Nil,
            }
        }
    }

    fn search(&mut self, search_types: &Vec<TokenType>) -> Option<TokenType> {
        if let Some(token) = self.token_iterator.peek() {
            let token_type = token.get_token_type();
            if search_types.contains(&token_type) {
                return Some(token_type);
            }
        }
        None
    }
}
