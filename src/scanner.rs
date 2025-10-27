use std::vec;

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanError>,
}

#[derive(Clone)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(ScanError {
            line,
            message: message.to_string(),
        })
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token::new(token_type, None, None, self.line);

        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let char = self.source.chars().nth(self.current);
        self.current += 1;

        if let Some(ch) = char {
            match ch {
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
                _ => self.error(self.line, &format!("Unexpected character: {}", ch)),
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScanError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        return Ok(self.tokens.clone());
    }
}
