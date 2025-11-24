use crate::token::*;

pub enum Stmt {
	Expression {
		expression: Box<Expr>,
	},
	Print {
		expression: Box<Expr>,
	},
}

