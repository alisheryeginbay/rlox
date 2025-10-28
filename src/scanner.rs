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
