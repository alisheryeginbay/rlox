use crate::token::*;

pub enum Expr {
	Binary {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},
	Ternary {
		condition: Box<Expr>,
		positive: Box<Expr>,
		negative: Box<Expr>,
	},
	Unary {
		operator: Token,
		right: Box<Expr>,
	},
	Grouping {
		expression: Box<Expr>,
	},
	Literal {
		value: Literal,
	},
}

