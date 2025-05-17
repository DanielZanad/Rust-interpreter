use std::rc::Rc;

use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Rc<Block>),
    Expression(Rc<Expression>),
    Print(Rc<Print>),
    Var(Rc<Var>),
}

pub trait Accept {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> R;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> R;
    fn visit_print_stmt(&mut self, stmt: &Print) -> R;
    fn visit_var_stmt(&mut self, stmt: &Var) -> R;
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct Print {
    expression: Expr,
}

#[derive(Debug)]
pub struct Var {
    name: Token,
    initializer: Expr,
}

#[derive(Clone, Debug)]
pub struct Block {
    statements: Vec<Stmt>,
}

impl Accept for Expression {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_expression_stmt(self)
    }
}

impl Accept for Print {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_print_stmt(self)
    }
}

impl Accept for Var {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_var_stmt(self)
    }
}

impl Accept for Block {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_block_stmt(self)
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

impl Var {
    pub fn new(name: Token, initializer: Expr) -> Self {
        Var { name, initializer }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Expr {
        &self.initializer
    }
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn statements(&self) -> &Vec<Stmt> {
        &self.statements
    }
}
