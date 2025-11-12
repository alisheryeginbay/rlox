use std::fmt::Display;

use crate::{
    expr::{Expr, PrimaryExprValue},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct ParseError {
    pub message: String,
    token: Token,
}

impl ParseError {
    fn new(message: impl Into<String>, token: Token) -> Self {
        ParseError {
            message: message.into(),
            token,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} at line {}", self.message, self.token.line)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        // a == != b
        self.block()
    }

    fn is_at_end(&self) -> bool {
        let token = &self.tokens[self.current];
        token.token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fn
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }

    fn matches(&mut self, tokens: &[TokenType]) -> bool {
        if self.is_at_end() {
            return false;
        }

        let token = &self.tokens[self.current];
        if !tokens.contains(&token.token_type) {
            return false;
        }
        self.current += 1;
        true
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn block(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.matches(&[TokenType::Comma]) {
            let previous = self.previous();
            let right = self.ternary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        if self.matches(&[TokenType::Question]) {
            let positive = self.ternary()?;
            self.consume(TokenType::Colon)?;
            let negative = self.ternary()?;

            expr = Expr::Ternary {
                condition: Box::new(expr),
                positive: Box::new(positive),
                negative: Box::new(negative),
            };

            return Ok(expr);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        // a > >= < <= b
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let previous = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        // a + - b
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let previous = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        // a * / b
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let previous = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let previous = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let previous = self.previous();
            let value = self.primary()?;

            return Ok(Expr::Unary {
                operator: previous,
                right: Box::new(Expr::from(value)),
            });
        }

        let value = self.primary()?;

        return Ok(Expr::from(value));
    }

    fn primary(&mut self) -> Result<PrimaryExprValue, ParseError> {
        if self.matches(&[
            TokenType::String,
            TokenType::Identifier,
            TokenType::Number,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            let previous = self.previous();
            return Ok(PrimaryExprValue::Literal(previous.literal.unwrap()));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)?;
            return Ok(PrimaryExprValue::Grouping(expr));
        }

        let current_token = self.tokens[self.current].clone();

        Err(ParseError::new(
            "Invalid expression (primary)",
            current_token,
        ))
    }

    fn consume(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        let current_token = self.tokens[self.current].clone();

        if self.is_at_end() {
            return Err(ParseError::new("Beyond tokens' length", current_token));
        }

        if current_token.token_type != token_type {
            return Err(ParseError::new(
                format!("Expected {} after expression.", token_type),
                current_token,
            ));
        }

        self.current += 1;

        Ok(())
    }
}
