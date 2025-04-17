use std::rc::Rc;

use crate::{token::Literal as LiteralValue, token::Token};

pub enum Expr {
    Binary(Rc<Binary>),
    Grouping(Rc<Grouping>),
    Literal(Rc<Literal>),
    Unary(Rc<Unary>),
}

pub trait Accept {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

pub struct Grouping {
    expression: Expr,
}

pub struct Literal {
    value: LiteralValue,
}

pub struct Unary {
    operator: Token,
    right: Expr,
}

impl Accept for Binary {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}

impl Accept for Grouping {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

impl Accept for Literal {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}

impl Accept for Unary {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Expr {
        &self.left
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }
}

impl Grouping {
    pub fn new(expr: Expr) -> Self {
        Self { expression: expr }
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }
}

impl Literal {
    pub fn new(literal: LiteralValue) -> Self {
        Self { value: literal }
    }

    pub fn value(&self) -> &LiteralValue {
        &self.value
    }
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }
}
