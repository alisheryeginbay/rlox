use crate::token::*;

pub enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        positive: Box<Expr>,
        negative: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
}
