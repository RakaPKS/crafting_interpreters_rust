use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
    pub column: usize,
}
impl Token {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

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
    fn _is_binary_op(&self) -> bool {
        !matches!(self, Operator::Bang)
    }
    fn _is_unary_op(&self) -> bool {
        matches!(self, Operator::Bang | Operator::Minus)
    }
}

// Implement Display for Operator
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

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Operators
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

    Eof,
}

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
