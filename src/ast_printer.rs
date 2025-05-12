use std::rc::Rc;

use crate::{
    expr::{Accept, Binary, Expr, Grouping, Literal, Unary, Visitor},
    literal_object::Literal as LiteralValue,
    token::Token,
};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator().lexeme, &[expr.left(), expr.right()])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[expr.expression()])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        match expr.value() {
            LiteralValue::Null => String::from("null"),
            LiteralValue::Number(number) => format!("{}", number),
            LiteralValue::String(string) => format!("{}", string),
            LiteralValue::Boolean(boolean) => format!("{}", boolean),
        }
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator().lexeme, &[expr.right()])
    }

    fn visit_variable_expr(&self, expr: &crate::expr::Variable) -> String {
        todo!()
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => binary.accept(self),
            Expr::Grouping(grouping) => grouping.accept(self),
            Expr::Literal(literal) => literal.accept(self),
            Expr::Unary(unary) => unary.accept(self),
            Expr::Variable(variable) => todo!(),
        }
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();

        builder.push_str("(");
        builder.push_str(name);

        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&self.print(expr));
        }
        builder.push_str(")");
        builder
    }
}

pub fn print_ast() {
    let operator = Token::new(
        crate::token_type::TokenType::STAR,
        "*".to_string(),
        LiteralValue::Null,
        1,
    );

    let grouping = Grouping::new(Expr::Literal(Rc::new(Literal::new(LiteralValue::Number(
        45.67,
    )))));

    let unary = Unary::new(
        Token::new(
            crate::token_type::TokenType::MINUS,
            "-".to_string(),
            LiteralValue::Null,
            1,
        ),
        Expr::Literal(Rc::new(Literal::new(LiteralValue::Number(123.0)))),
    );

    let expr = Expr::Binary(Rc::new(Binary::new(
        Expr::Unary(Rc::new(unary)),
        operator,
        Expr::Grouping(Rc::new(grouping)),
    )));

    let ast_printer = AstPrinter;

    println!("{}", ast_printer.print(&expr))
}
