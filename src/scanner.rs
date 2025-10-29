use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
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
            source: source.chars().collect(),
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

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let token = Token::new(token_type, None, literal, self.line);

        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        return char;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.advance();
        return true;
    }

    // TODO: add escape symbosl support
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = self.source[self.start..self.current].to_vec();
        self.add_token(TokenType::String, Some(value.iter().collect()));
    }

    fn number(&mut self) {
        while self.peek().is_numeric() && !self.is_at_end() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .to_vec()
            .iter()
            .collect();

        self.add_token(TokenType::Number, Some(value));
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),

            '!' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }

            '/' => {
                if self.consume_if('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }

            ' ' | '\r' | '\t' => {
                // Ignoring whitespace
            }

            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),

            '0'..='9' => self.number(),

            _ => self.error(self.line, &format!("Unexpected character: {}", char)),
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
