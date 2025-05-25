use std::rc::Rc;

use crate::{literal_object::Literal as LiteralValue, token::Token};

#[derive(Debug)]
pub enum Expr {
    Assign(Rc<Assign>),
    Binary(Rc<Binary>),
    Grouping(Rc<Grouping>),
    Literal(Rc<Literal>),
    Unary(Rc<Unary>),
    Variable(Rc<Variable>),
}

pub trait Accept {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
}

#[derive(Debug)]
pub struct Assign {
    name: Token,
    value: Expr,
}

#[derive(Debug)]
pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

#[derive(Debug)]
pub struct Grouping {
    expression: Expr,
}

#[derive(Debug)]
pub struct Literal {
    value: LiteralValue,
}

#[derive(Debug)]
pub struct Unary {
    operator: Token,
    right: Expr,
}

#[derive(Debug)]
pub struct Variable {
    name: Token,
}

impl Accept for Assign {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_assign_expr(self)
    }
}

impl Accept for Binary {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}

impl Accept for Grouping {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

impl Accept for Literal {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}

impl Accept for Unary {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}

impl Accept for Variable {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_variable_expr(self)
    }
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Expr {
        &self.value
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

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}
