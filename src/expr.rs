use crate::token::*;

pub enum Expr {
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
	Unary {
		operator: Token,
		right: Box<Expr>,
	},
	Literal {
		value: Literal,
	},
	Grouping {
		expression: Box<Expr>,
	},
}

