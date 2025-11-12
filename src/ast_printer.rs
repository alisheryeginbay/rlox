use crate::expr::Expr;

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::from("(");
        result.push_str(name);

        for expr in exprs {
            result.push(' ');
            result.push_str(&self.print(expr));
        }

        result.push(')');

        result
    }

    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right]),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression]),
            Expr::Literal { value } => value.to_string(),
            Expr::Ternary {
                condition,
                positive,
                negative,
            } => self.parenthesize("?:", &[condition, positive, negative]),
        }
    }
}
