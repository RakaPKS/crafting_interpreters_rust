use std::{iter::Peekable, str::Chars};

use crate::{
    error_reporter::ErrorReporter,
    token::{Literal, Operator, Token, TokenType, KEYWORDS},
};

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    pub fn scan_tokens(&mut self, error_reporter: &mut ErrorReporter) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.advance() {
            match c {
                //Single Character Tokens
                '(' => tokens.push(self.add_single_character_token(TokenType::LeftParen, c)),
                ')' => tokens.push(self.add_single_character_token(TokenType::RightParen, c)),
                '{' => tokens.push(self.add_single_character_token(TokenType::LeftBrace, c)),
                '}' => tokens.push(self.add_single_character_token(TokenType::RightBrace, c)),
                ',' => tokens.push(self.add_single_character_token(TokenType::Comma, c)),
                '.' => tokens.push(self.add_single_character_token(TokenType::Dot, c)),
                '-' => tokens
                    .push(self.add_single_character_token(TokenType::Operator(Operator::Minus), c)),
                '+' => tokens
                    .push(self.add_single_character_token(TokenType::Operator(Operator::Plus), c)),
                ';' => tokens.push(self.add_single_character_token(TokenType::Semicolon, c)),

                '*' => {
                    if self.match_next('/') {
                        error_reporter.error(self.line, self.column, "Unexpected closing comment marker '*/' without a corresponding opening '/*'.");
                    } else {
                        tokens.push(
                            self.add_single_character_token(TokenType::Operator(Operator::Star), c),
                        )
                    }
                }
                //Operators
                '!' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::Operator(Operator::BangEqual),
                            "!=".to_string(),
                            None,
                        ))
                    } else {
                        tokens.push(
                            self.add_single_character_token(TokenType::Operator(Operator::Bang), c),
                        )
                    }
                }
                '=' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::Operator(Operator::EqualEqual),
                            "==".to_string(),
                            None,
                        ))
                    } else {
                        tokens.push(
                            self.add_single_character_token(
                                TokenType::Operator(Operator::Equal),
                                c,
                            ),
                        )
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::Operator(Operator::GreaterEqual),
                            ">=".to_string(),
                            None,
                        ))
                    } else {
                        tokens.push(
                            self.add_single_character_token(
                                TokenType::Operator(Operator::Greater),
                                c,
                            ),
                        )
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::Operator(Operator::LessEqual),
                            "<=".to_string(),
                            None,
                        ))
                    } else {
                        tokens.push(
                            self.add_single_character_token(TokenType::Operator(Operator::Less), c),
                        )
                    }
                }
                '/' => {
                    if self.match_next('/') {
                        //Handle comments by ignoring untill newline
                        while matches!(self.chars.peek(), Some(&c) if c != '\n') {
                            self.advance();
                        }
                    } else if self.match_next('*') {
                        // Multi-line comment
                        loop {
                            match (self.advance(), self.chars.peek()) {
                                (Some('\n'), _) => {
                                    self.line += 1;
                                    self.column = 1;
                                }
                                (Some('*'), Some(&'/')) => {
                                    self.advance();
                                    break;
                                }
                                (None, _) => {
                                    error_reporter.error(
                                        self.line,
                                        self.column,
                                        "Unterminated multi-line comment.",
                                    );
                                    break;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        tokens.push(
                            self.add_single_character_token(
                                TokenType::Operator(Operator::Slash),
                                c,
                            ),
                        )
                    }
                }

                //Handle String Literals
                '"' => {
                    let mut lexeme = String::new();
                    lexeme.push('"'); // Include the opening quote in the lexeme
                    self.advance(); // Move past the opening quote
                    let mut closed = false;
                    while let Some(&c) = self.chars.peek() {
                        self.advance(); // Consume the character
                        if c == '"' {
                            lexeme.push(c); // Include the closing quote in the lexeme
                            closed = true;
                            break;
                        }
                        if c == '\n' {
                            self.line += 1;
                            self.column = 1;
                        }
                        lexeme.push(c);
                    }
                    if !closed {
                        error_reporter.error(self.line, self.column, "Unterminated string.");
                    } else {
                        let string_content = lexeme.trim_matches('"').to_string();
                        tokens.push(self.add_token(
                            TokenType::String,
                            lexeme,
                            Some(Literal::String(string_content)),
                        ));
                    }
                }
                // Handle whitespace by ignoring it
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                }

                _ => {
                    if c.is_ascii_digit() {
                        tokens.push(self.number(c, error_reporter))
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        tokens.push(self.identifier(c))
                    } else {
                        error_reporter.error(self.line, self.column, "Unexepected character.")
                    }
                }
            }
        }
        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            self.line,
            self.column,
        ));
        tokens
    }

    fn add_single_character_token(&self, token_type: TokenType, c: char) -> Token {
        self.add_token(token_type, c.to_string(), None)
    }

    fn add_token(&self, token_type: TokenType, lexeme: String, literal: Option<Literal>) -> Token {
        Token::new(token_type, lexeme, literal, self.line, self.column)
    }

    fn match_next(&mut self, next_char: char) -> bool {
        matches!(self.chars.peek(), Some(&c) if c == next_char) && {
            self.advance();
            true
        }
    }
    fn number(&mut self, first_digit: char, error_reporter: &mut ErrorReporter) -> Token {
        let mut has_decimal = false;
        let mut lexeme = first_digit.to_string();
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_digit() => {
                    lexeme.push(c);
                    self.advance();
                }
                Some(&'.') if !has_decimal => {
                    has_decimal = true;
                    lexeme.push('.');
                    self.advance();
                }
                Some(&'.') if has_decimal => {
                    error_reporter.error(
                        self.line,
                        self.column,
                        "Invalid number: multiple decimal points.",
                    );
                    break;
                }
                _ => break,
            }
        }
        self.add_token(
            TokenType::Number,
            lexeme.clone(),
            Some(Literal::Number(lexeme.parse().unwrap())),
        )
    }

    fn identifier(&mut self, c: char) -> Token {
        let mut lexeme = c.to_string();
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_alphanumeric() || c == '_' => {
                    lexeme.push(c);
                    self.advance();
                }
                _ => break,
            }
        }
        let token_type = KEYWORDS
            .get(lexeme.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);
        match token_type {
            TokenType::Nil => self.add_token(token_type, lexeme, Some(Literal::Nil)),
            TokenType::True => self.add_token(token_type, lexeme, Some(Literal::Boolean(true))),
            TokenType::False => self.add_token(token_type, lexeme, Some(Literal::Boolean(false))),
            _ => self.add_token(token_type, lexeme, None),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        self.column += 1;
        c
    }
}
