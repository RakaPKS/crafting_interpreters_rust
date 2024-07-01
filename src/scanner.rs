use std::{iter::Peekable, str::Chars, usize};

use crate::{
    error_reporter::ErrorReporter,
    token::{Literal, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    line: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
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
                        tokens.push(self.add_token(
                            TokenType::BangEqual,
                            "!=".to_string(),
                            Some(Literal::Nil),
                        ))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Bang, c))
                    }
                }
                '=' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::EqualEqual,
                            "==".to_string(),
                            Some(Literal::Nil),
                        ))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Equal, c))
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::GreaterEqual,
                            ">=".to_string(),
                            Some(Literal::Nil),
                        ))
                    } else {
                        tokens.push(self.add_single_character_token(TokenType::Greater, c))
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        tokens.push(self.add_token(
                            TokenType::LessEqual,
                            "<=".to_string(),
                            Some(Literal::Nil),
                        ))
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
                    while matches!(self.chars.peek(), Some(&c) if c!= '"') {
                        if c == '\n' {
                            self.line += 1;
                        }
                        lexeme.push(c);
                        self.chars.next();
                    }
                    tokens.push(self.add_token(
                        TokenType::String,
                        lexeme.clone(),
                        Some(Literal::String(lexeme)),
                    ))
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
        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Some(Literal::Nil),
            self.line,
        ));
        tokens
    }

    fn add_single_character_token(&self, token_type: TokenType, c: char) -> Token {
        self.add_token(token_type, c.to_string(), Some(Literal::Nil))
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

    fn number(&mut self, c: char) -> Token {
        todo!()
    }
    fn identifier(&self, c: char) -> Token {
        todo!()
    }
}
