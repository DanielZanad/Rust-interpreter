use std::rc::Rc;

use crate::expr::Expr;

pub enum Stmt {
    Expression(Rc<Expression>),
    Print(Rc<Print>),
}

pub trait Accept {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit_expression_stmt(&self, stmt: &Expression) -> R;
    fn visit_print_stmt(&self, stmt: &Print) -> R;
}

pub struct Expression {
    pub expression: Expr,
}

pub struct Print {
    expression: Expr,
}

impl Accept for Expression {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_expression_stmt(self)
    }
}

impl Accept for Print {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_print_stmt(self)
    }
}

impl Expression {
    pub fn new(expression: Expr) -> Self {
        Expression { expression }
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }
}

impl Print {
    pub fn new(expression: Expr) -> Self {
        Print { expression }
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }
}
