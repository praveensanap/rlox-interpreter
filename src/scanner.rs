use crate::token::*;
use crate::LoxError;
pub struct Scanner {
    pub source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn Keywords(key: &str) -> Option<TokenType> {
    match key {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        //self.source
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                }
            }
        }
        self.tokens.push(Token::eof(self.line));
        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }
    pub fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        let k = match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.is_match('/') {
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.is_match('*') {
                    self.comment_block()?;
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            '0'..='9' => self.number(),
            _ => {
                if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    return Err(LoxError::error(
                        self.line,
                        "Unexpected character".to_string(),
                    ));
                }
            }
        };
        Ok(k)
    }

    pub fn comment_block(&mut self) -> Result<(), LoxError> {
        while !self.is_double_match('*', '/') {
            if let Some(ch) = self.peek() {
                if ch == '\n' {
                    self.line += 1;
                }
            }
            self.advance();
            if self.is_double_match('/', '*') {
                self.comment_block()?;
            }
            if self.is_at_end() {
                return Err(LoxError::error(
                    self.line,
                    "Unterminated comment block".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(ttype) = Keywords(&text) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }
    pub fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if let Some(ch) = self.peek() {
            if ch == '.' && Scanner::is_digit(self.peek_next()) {
                self.advance();
                while Scanner::is_digit(self.peek()) {
                    self.advance();
                }
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();

        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Literals, Some(Object::Num(num)));
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
    pub fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            return ch.is_ascii_alphanumeric();
        } else {
            return false;
        }
    }

    pub fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    pub fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::Literals, Some(Object::Str(value)));
        Ok(())
    }

    pub fn is_match(&mut self, ex: char) -> bool {
        if let Some(ch) = self.source.get(self.current) {
            if *ch == ex {
                self.current += 1;
                return true;
            }
            return false;
        }
        false
        // if self.is_at_end() {
        //     return false;
        // } else if *self.source.get(self.current).unwrap() != ex {
        //     return false;
        // }
        // self.current += 1;
        // return true;
    }

    pub fn is_double_match(&mut self, ex1: char, ex2: char) -> bool {
        if let Some(ch1) = self.source.get(self.current) {
            if let Some(ch2) = self.source.get(self.current + 1) {
                if *ch1 == ex1 && *ch2 == ex2 {
                    self.current += 2;
                    return true;
                }
            }
            return false;
        }
        false
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    pub fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current = self.current + 1;
        result
    }

    pub fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, Some(Object::Nil))
    }

    pub fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let s: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, s.to_string(), literal, self.line))
    }
}
