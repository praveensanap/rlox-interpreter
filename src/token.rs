use std::fmt;

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: Some(Object::Nil),
            line,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal = match &self.literal {
            None => "None".to_string(),
            Some(v) => v.to_string(),
        };
        write!(f, "{:?} {} {}", self.lexeme, literal, self.line)
    }
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Slash,
    Minus,
    Plus,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Literals,
    Identifier,
    String,
    Number,
    Keywords,
    Class,
    Nil,
    Fun,
    Return,
    This,
    Super,
    If,
    Else,
    And,
    True,
    False,
    Or,
    For,
    While,
    Var,
    Print,
    Eof,
}
