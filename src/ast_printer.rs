use crate::token::{Literal, Token};
use crate::expr::expr::{Expr, Visitor, Binary, Grouping, Unary};
use crate::token_type::TokenType::{MINUS, STAR};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => binary.accept(self),
            Expr::Grouping(grouping) => grouping.accept(self),
            Expr::Literal(literal) => literal.accept(self),
            Expr::Unary(unary) => unary.accept(self),
        }
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&self.print(expr));
        }
        builder.push(')');
        builder
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        match &expr {
            Literal::NumberLiteral(value) => format!("{:?}", value),
            Literal::StringLiteral(value) => format!("{:?}", value),
            Literal::Nil => format!("nil"),
            
        }
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

pub fn print_ast() {
    let expression = Expr::Binary(std::rc::Rc::new(Binary {
        left: Expr::Unary(std::rc::Rc::new(Unary {
            operator: Token {
                _type: MINUS,
                lexeme: "-".to_string(),
                literal: Literal::Nil,
                line: 1,
            },
            right: Expr::Literal(std::rc::Rc::new(Literal::NumberLiteral(123.00))),
        })),
        operator: Token {
            _type: STAR,
            lexeme: "*".to_string(),
            literal: Literal::Nil,
            line: 1,
        },
        right: Expr::Grouping(std::rc::Rc::new(Grouping {
            expression: Expr::Literal(std::rc::Rc::new(Literal::NumberLiteral(45.57))),
        })),
    }));

    let printer = AstPrinter;
    println!("{}", printer.print(&expression));
}
