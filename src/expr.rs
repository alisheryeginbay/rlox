use crate::token::*;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum PrimaryExprValue {
    Literal(Literal),
    Grouping(Expr),
}

impl From<PrimaryExprValue> for Expr {
    fn from(value: PrimaryExprValue) -> Self {
        match value {
            PrimaryExprValue::Literal(literal) => {
                return Expr::Literal { value: literal };
            }
            PrimaryExprValue::Grouping(expr) => {
                return Expr::Grouping {
                    expression: Box::new(expr),
                };
            }
        }
    }
}
