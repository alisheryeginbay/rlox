use crate::{
    expr::Expr,
    token::{Literal, TokenType},
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expr: Expr) {
        let value = self.evaluate(&expr);
        println!("{}", self.stringify(&value));
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

    fn evaluate(&self, expr: &Expr) -> Literal {
        match expr {
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate(&expression),
            Expr::Unary { operator, right } => {
                let value = self.evaluate(&right);
                match operator.token_type {
                    TokenType::Minus => {
                        let value = value.as_number();
                        match value {
                            Some(v) => Literal::Number(-v),
                            None => {
                                panic!("Operand must be a number")
                            }
                        }
                    }
                    TokenType::Bang => Literal::Boolean(!self.is_truthy(&value)),
                    _ => panic!("Invalid operator for an unary expression"),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(&left);
                let right_value = self.evaluate(&right);

                match (left_value, right_value) {
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
                        _ => panic!("Invalid operator for a binary expression"),
                    },
                    _ => {
                        panic!("Both operands must be numbers")
                    }
                }
            }
            Expr::Ternary {
                condition,
                positive,
                negative,
            } => {
                let value = self.evaluate(&condition);
                if self.is_truthy(&value) {
                    self.evaluate(positive)
                } else {
                    self.evaluate(negative)
                }
            }
        }
    }
}
