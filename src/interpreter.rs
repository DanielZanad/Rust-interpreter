use std::fmt::Error;

use crate::expr::{Accept, Expr, Visitor};
use crate::literal_object::Literal;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl RuntimeError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

pub struct Interpreter {}

impl Visitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&self, expr: &crate::expr::Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;

        match expr.operator().type_ {
            TokenType::GREATER => self.eval_comparison_binary_op(left, right, |l, r| l > r, ">"),
            TokenType::GREATER_EQUAL => {
                self.eval_comparison_binary_op(left, right, |l, r| l >= r, ">")
            }
            TokenType::LESS => self.eval_comparison_binary_op(left, right, |l, r| l < r, ">"),
            TokenType::LESS_EQUAL => {
                self.eval_comparison_binary_op(left, right, |l, r| l <= r, ">")
            }
            TokenType::MINUS => self.eval_number_binary_op(left, right, |l, r| l - r, "-"),
            TokenType::SLASH => self.eval_number_binary_op(left, right, |l, r| l / r, "/"),
            TokenType::STAR => self.eval_number_binary_op(left, right, |l, r| l * r, "*"),
            TokenType::PLUS => match (right, left) {
                (Literal::Number(l), Literal::Number(r)) => self.eval_number_binary_op(
                    Literal::Number(l),
                    Literal::Number(r),
                    |l, r| l + r,
                    "+",
                ),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::new("Binary expression error")),
            },
            TokenType::BANG_EQUAL => Ok(Literal::Boolean(!self.is_equals(left, right))),
            TokenType::EQUAL_EQUAL => Ok(Literal::Boolean(!self.is_equals(left, right))),
            _ => Err(RuntimeError::new("Binary expression error")),
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::expr::Grouping) -> Result<Literal, RuntimeError> {
        Ok(self.evaluate(&expr.expression())?)
    }

    fn visit_literal_expr(&self, expr: &crate::expr::Literal) -> Result<Literal, RuntimeError> {
        Ok(expr.value().clone())
    }

    fn visit_unary_expr(&self, expr: &crate::expr::Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr.right());

        match expr.operator().type_ {
            TokenType::MINUS => match right {
                Ok(Literal::Number(n)) => return Ok(Literal::Number(-n)),
                _ => Err(RuntimeError::new("Error casting number")),
            },
            TokenType::BANG => match right {
                Ok(right) => return Ok(Literal::Boolean(self.is_truthy(right))),
                _ => Err(RuntimeError::new("Unary expression error")),
            },
            _ => Err(RuntimeError::new("Unary expression error")),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Binary(binary) => binary.accept(self),
            Expr::Grouping(grouping) => grouping.accept(self),
            Expr::Literal(literal) => literal.accept(self),
            Expr::Unary(unary) => unary.accept(self),
        }
    }

    fn is_truthy(&self, literal: Literal) -> bool {
        match literal {
            Literal::Null => false,
            Literal::Boolean(value) => value,
            _ => true,
        }
    }

    fn eval_number_binary_op<F: Fn(f64, f64) -> f64>(
        &self,
        left: Literal,
        right: Literal,
        op: F,
        op_name: &str,
    ) -> Result<Literal, RuntimeError> {
        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => Ok(Literal::Number(op(left, right))),
            _ => Err(RuntimeError::new(&format!(
                "Operator '{}' requires two numbers",
                op_name
            ))),
        }
    }

    fn eval_comparison_binary_op<F: Fn(f64, f64) -> bool>(
        &self,
        left: Literal,
        right: Literal,
        op: F,
        op_name: &str,
    ) -> Result<Literal, RuntimeError> {
        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => {
                Ok(Literal::Boolean(op(left, right)))
            }
            _ => Err(RuntimeError::new(&format!(
                "Operator '{}' requires two numbers",
                op_name
            ))),
        }
    }

    fn is_equals(&self, left: Literal, right: Literal) -> bool {
        if left == Literal::Null && right == Literal::Null {
            return true;
        }

        if left == Literal::Null {
            return false;
        }

        left.eq(&right)
    }
}
