use std::{iter::Peekable, str::Chars, usize};

use crate::{
    error_reporter::ErrorReporter,
    token::{Literal, Token, TokenType, KEYWORDS},
};

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
            line: 1,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self, mut error_reporter: ErrorReporter) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.chars.next() {
            match c {
                //Single Character Tokens
                '(' => tokens.push(self.add_single_character_token(TokenType::LeftParen, c)),
                ')' => tokens.push(self.add_single_character_token(TokenType::RightParen, c)),
                '{' => tokens.push(self.add_single_character_token(TokenType::LeftBrace, c)),
                '}' => tokens.push(self.add_single_character_token(TokenType::RightBrace, c)),
                ',' => tokens.push(self.add_single_character_token(TokenType::Comma, c)),
                '.' => tokens.push(self.add_single_character_token(TokenType::Dot, c)),
                '-' => tokens.push(self.add_single_character_token(TokenType::Minus, c)),
                '+' => tokens.push(self.add_single_character_token(TokenType::Plus, c)),
                ';' => tokens.push(self.add_single_character_token(TokenType::Semicolon, c)),
                '*' => tokens.push(self.add_single_character_token(TokenType::Star, c)),

                //Operators
                '!' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(TokenType::BangEqual, "!=".to_string(), None))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Bang, c))
                    }
                }
                '=' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(TokenType::EqualEqual, "==".to_string(), None))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Equal, c))
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(TokenType::GreaterEqual, ">=".to_string(), None))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Greater, c))
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(TokenType::LessEqual, "<=".to_string(), None))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Less, c))
                    }
                }
                '/' => {
                    if self.match_next('/') {
                        //Handle comments by ignoring untill newline
                        while matches!(self.chars.peek(), Some(&c) if c != '\n') {
                            self.chars.next();
                        }
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Slash, c))
                    }
                }

                //Handle String Literals
                '"' => {
                    let mut lexeme = String::new();
                    loop {
                        match self.chars.peek() {
                            Some(&c) if c != '"' => {
                                if c == '\n' {
                                    self.line += 1;
                                }
                                lexeme.push(c);
                                self.chars.next();
                            }
                            _ => break,
                        }
                    }
                    tokens.push(self.add_token(
                        TokenType::String,
                        lexeme.clone(),
                        Some(Literal::String(lexeme)),
                    ));
                }
                // Handle whitespace by ignoring it
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,

                _ => {
                    if c.is_ascii_digit() {
                        tokens.push(self.number(c))
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        tokens.push(self.identifier(c))
                    } else {
                        error_reporter.error(self.line, "Unexepected character.")
                    }
                }
            }
            self.current += 1;
        }
        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        tokens
    }

    fn add_single_character_token(&self, token_type: TokenType, c: char) -> Token {
        self.add_token(token_type, c.to_string(), None)
    }

    fn add_token(&self, token_type: TokenType, lexeme: String, literal: Option<Literal>) -> Token {
        Token::new(token_type, lexeme, literal, self.line)
    }

    fn match_next(&mut self, next_char: char) -> bool {
        matches!(self.chars.peek(), Some(&c) if c == next_char) && {
            self.chars.next();
            true
        }
    }
    fn number(&mut self, first_digit: char) -> Token {
        let mut lexeme = first_digit.to_string();
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_digit() || c == '.' => {
                    if c == '.' {
                        let mut next_iter = self.chars.clone();
                        next_iter.next();
                        if let Some(&n) = next_iter.peek() {
                            if !n.is_ascii_digit() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    lexeme.push(c);
                    self.chars.next();
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
                    self.chars.next();
                }
                _ => break,
            }
        }
        let token_type = KEYWORDS
            .get(lexeme.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);
        if token_type == TokenType::Nil {
            self.add_token(token_type, lexeme, Some(Literal::Nil))
        } else {
            self.add_token(token_type, lexeme, None)
        }
    }
}
