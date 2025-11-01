use crate::token::*;
use std::rc::Rc;

pub enum Expr {
	Binary {
		left: Rc<Expr>,
		operator: Token,
		right: Rc<Expr>,
	},
	Unary {
		operator: Token,
		right: Rc<Expr>,
	},
	Literal {
		value: Literal,
	},
	Grouping {
		expression: Rc<Expr>,
	},
}
