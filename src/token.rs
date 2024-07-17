//! This module defines the token types and related structures for the Lox interpreter.
//!
//! It includes the `Token` struct, `Literal` and `Operator` enums, and the `TokenType` enum
//! which are fundamental to lexical analysis and parsing in the Lox language implementation.

use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

/// Represents a token in the Lox language.
///
/// A token is the smallest unit of the language that the parser deals with.
/// It contains information about the type of the token, its lexeme (the actual
/// text), any literal value associated with it, and its position in the source code.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    /// The lexeme (actual text) of the token.
    lexeme: String,
    /// The literal value, if any.
    pub literal: Option<Literal>,
    /// The line number where the token appears.
    pub line: usize,
    /// The column number where the token starts.
    pub column: usize,
}

impl Token {
    /// Creates a new Token with given properties.
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

/// Represents literal values in the Lox language.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

/// Represents operators in the Lox language.
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    // Single-character operators.
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character operators.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Operator {
    /// Checks if the operator is a binary operator.
    fn _is_binary_op(&self) -> bool {
        !matches!(self, Operator::Bang)
    }

    /// Checks if the operator is a unary operator.
    fn _is_unary_op(&self) -> bool {
        matches!(self, Operator::Bang | Operator::Minus)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Slash => write!(f, "/"),
            Operator::Star => write!(f, "*"),
            Operator::Bang => write!(f, "!"),
            Operator::BangEqual => write!(f, "!="),
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
        }
    }
}

/// Represents all possible token types in the Lox language.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    /// Operators (arithmetic, comparison, equality)
    Operator(Operator),

    // Single-character Tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,

    // Literals.
    /// Identifier (variable names, function names, etc.)
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    /// End of file
    Eof,
}

/// A map of Lox keywords to their corresponding `TokenType`.
///
/// This `Lazy` static allows for efficient lookup of keywords during lexical analysis.
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("and", TokenType::And);
    map.insert("class", TokenType::Class);
    map.insert("else", TokenType::Else);
    map.insert("false", TokenType::False);
    map.insert("fun", TokenType::Fun);
    map.insert("for", TokenType::For);
    map.insert("if", TokenType::If);
    map.insert("nil", TokenType::Nil);
    map.insert("or", TokenType::Or);
    map.insert("print", TokenType::Print);
    map.insert("return", TokenType::Return);
    map.insert("super", TokenType::Super);
    map.insert("this", TokenType::This);
    map.insert("true", TokenType::True);
    map.insert("var", TokenType::Var);
    map.insert("while", TokenType::While);
    map
});
