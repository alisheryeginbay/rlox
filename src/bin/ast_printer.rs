use rlox::{
    ast_printer::AstPrinter,
    expr::Expr,
    token::{Literal, Token, TokenType},
};

fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal {
                value: Literal::Number(123.0),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Number(45.67),
            }),
        }),
    };

    let ast_printer = AstPrinter;
    println!("{}", ast_printer.print(&expression));
}
