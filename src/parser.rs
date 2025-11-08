use crate::{
    expr::{Expr, PrimaryExprValue},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        // a == != b
        self.equality()
    }

    fn matches(&mut self, tokens: Vec<TokenType>) -> bool {
        if self.current + 1 >= self.tokens.len() {
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

    fn equality(&mut self) -> Expr {
        // a > >= < <= b
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let previous = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        // a + - b
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let previous = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        // a * / b
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let previous = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let previous = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: previous,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let previous = self.previous();
            let value = self.primary();

            return Expr::Unary {
                operator: previous,
                right: Box::new(Expr::from(value)),
            };
        }

        let value = self.primary();

        return Expr::from(value);
    }

    fn primary(&mut self) -> PrimaryExprValue {
        if self.matches(vec![
            TokenType::String,
            TokenType::Identifier,
            TokenType::Number,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            let previous = self.previous();
            return PrimaryExprValue::Literal(previous.literal.unwrap());
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen);
            return PrimaryExprValue::Grouping(expr);
        }

        panic!("Invalid expression (primary)");
    }

    fn consume(&mut self, token_type: TokenType) {
        if self.current >= self.tokens.len() {
            panic!("Beyond tokens' length");
        }

        if self.tokens[self.current].token_type != token_type {
            panic!("Expected {} after expression.", token_type);
        }

        self.current += 1;
    }
}
