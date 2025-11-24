use std::{error::Error, fmt::Display};

use crate::{
    expr::Expr,
    token::{Literal, TokenType},
};

pub struct Interpreter;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[error at line {}] {}", self.line, self.message)
    }
}

impl Error for RuntimeError {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expr: Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(&expr)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn stringify(&self, value: &Literal) -> String {
        value.to_string()
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Number(value) => *value != 0.0,
            Literal::Boolean(value) => *value,
            Literal::String(_) => true,
            Literal::Nil => false,
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => self.evaluate(&expression),
            Expr::Unary { operator, right } => {
                let value = self.evaluate(&right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        let value = value.as_number();
                        match value {
                            Some(v) => Ok(Literal::Number(-v)),
                            None => {
                                panic!("Operand must be a number")
                            }
                        }
                    }
                    TokenType::Bang => Ok(Literal::Boolean(!self.is_truthy(&value))),
                    _ => {
                        eprintln!("Invalid operator for an unary expression");
                        Err(RuntimeError {
                            message: "Invalid operator for an unary expression".to_string(),
                            line: operator.line,
                        })
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(&left)?;
                let right_value = self.evaluate(&right)?;

                Ok(match (left_value, right_value) {
                    (Literal::Number(l), Literal::Number(r)) => match operator.token_type {
                        TokenType::GreaterEqual => Literal::Boolean(l >= r),
                        TokenType::Greater => Literal::Boolean(l > r),
                        TokenType::Less => Literal::Boolean(l < r),
                        TokenType::LessEqual => Literal::Boolean(l <= r),
                        TokenType::EqualEqual => Literal::Boolean(l == r),
                        TokenType::BangEqual => Literal::Boolean(l != r),
                        TokenType::Plus => Literal::Number(l + r),
                        TokenType::Minus => Literal::Number(l - r),
                        TokenType::Star => Literal::Number(l * r),

                        _ => {
                            return Err(RuntimeError {
                                line: operator.line,
                                message: "Invalid operator for a binary expression".to_string(),
                            });
                        }
                    },
                    (Literal::String(l), r) => Literal::String(l.to_string() + &r.to_string()),
                    (l, Literal::String(r)) => Literal::String(l.to_string() + &r.to_string()),
                    _ => {
                        return Err(RuntimeError {
                            line: operator.line,
                            message: format!(
                                "Can not perform {} on this expression",
                                operator.lexeme
                            ),
                        });
                    }
                })
            }
            Expr::Ternary {
                condition,
                positive,
                negative,
            } => {
                let value = self.evaluate(&condition)?;
                if self.is_truthy(&value) {
                    self.evaluate(positive)
                } else {
                    self.evaluate(negative)
                }
            }
        }
    }
}
