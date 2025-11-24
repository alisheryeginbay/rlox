use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
	Var {
		name: Token,
		initializer: Box<Expr>,
	},
	Print {
		expression: Box<Expr>,
	},
	Expression {
		expression: Box<Expr>,
	},
}

