use std::{error::Error, fmt::Display};

use crate::{
    expr::Expr,
    primary_expr::PrimaryExprValue,
    token::{Token, TokenType},
};

#[derive(Clone, Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.message, self.line)
    }
}

impl Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: vec![],
        }
    }

    fn report_error<T: Into<String>>(&mut self, message: T, token: Token) {
        self.errors.push(ParseError {
            message: message.into(),
            line: token.line,
        });
    }

    pub fn parse(&mut self) -> Result<Expr, Vec<ParseError>> {
        self.expression().or_else(|err| {
            self.errors.push(err);
            Err(self.errors.clone())
        })
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
            self.consume_if(TokenType::Colon)?;
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
            self.consume_if(TokenType::RightParen)?;
            return Ok(PrimaryExprValue::Grouping(expr));
        }

        let current_token = self.tokens[self.current].clone();

        if matches!(
            current_token.token_type,
            TokenType::Plus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::EqualEqual
                | TokenType::BangEqual
        ) {
            self.report_error(
                format!("Expected operand before {}", current_token.lexeme),
                current_token.clone(),
            );

            self.consume()?;

            return self
                .expression()
                .map(|expr| PrimaryExprValue::Grouping(expr));
        }

        Err(ParseError {
            message: "Invalid expression (primary)".to_string(),
            line: current_token.line,
        })
    }

    fn consume(&mut self) -> Result<(), ParseError> {
        let current_token = self.tokens[self.current].clone();

        if self.is_at_end() {
            return Err(ParseError {
                message: "Beyond tokens' length".to_string(),
                line: current_token.line,
            });
        }

        self.current += 1;

        Ok(())
    }

    fn consume_if(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        let current_token = self.tokens[self.current].clone();

        if self.is_at_end() {
            return Err(ParseError {
                message: "Beyond tokens' length".to_string(),
                line: current_token.line,
            });
        }

        if current_token.token_type != token_type {
            return Err(ParseError {
                message: format!("Expected {} after expression.", token_type),
                line: current_token.line,
            });
        }

        self.current += 1;

        Ok(())
    }
}
