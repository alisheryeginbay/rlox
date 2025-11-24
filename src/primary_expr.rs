use crate::{expr::Expr, token::Literal};

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
