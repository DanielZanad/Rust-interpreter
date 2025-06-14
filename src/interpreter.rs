use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::expr::{Accept as AcceptExpr, Expr, Visitor};
use crate::literal_object::Literal;
use crate::stmt::{Accept as AcceptStmt, Stmt, Visitor as VisitorStmt};
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, msg: &str) -> Self {
        Self {
            token,
            message: msg.to_string(),
        }
    }
}

pub struct Interpreter {
    pub environment: Environment,
}

impl VisitorStmt<Result<(), RuntimeError>> for Interpreter {
    fn visit_expression_stmt(
        &mut self,
        stmt: &crate::stmt::Expression,
    ) -> Result<(), RuntimeError> {
        let result = self.evaluate(stmt.expression());
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Print) -> Result<(), RuntimeError> {
        let value = self.evaluate(stmt.expression());
        match value {
            Ok(value) => {
                println!("{}", self.stringify(value));
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Var) -> Result<(), RuntimeError> {
        let value = self.evaluate(stmt.initializer());
        match value {
            Ok(value) => {
                self.environment.define(&stmt.name().lexeme, value);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Block) -> Result<(), RuntimeError> {
        let new_env = Environment::new_enclosed(Rc::new(RefCell::new(self.environment.clone())));
        self.execute_block(stmt.statements().to_vec(), new_env)
    }
}

impl Visitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &crate::expr::Binary) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;
        match expr.operator().type_ {
            TokenType::GREATER => self.eval_comparison_binary_op(
                left,
                right,
                |l, r| l > r,
                ">",
                expr.operator().clone(),
            ),
            TokenType::GREATER_EQUAL => self.eval_comparison_binary_op(
                left,
                right,
                |l, r| l >= r,
                ">=",
                expr.operator().clone(),
            ),
            TokenType::LESS => self.eval_comparison_binary_op(
                left,
                right,
                |l, r| l < r,
                "<",
                expr.operator().clone(),
            ),
            TokenType::LESS_EQUAL => self.eval_comparison_binary_op(
                left,
                right,
                |l, r| l <= r,
                "<=",
                expr.operator().clone(),
            ),
            TokenType::MINUS => {
                self.eval_number_binary_op(left, right, |l, r| l - r, "-", expr.operator().clone())
            }
            TokenType::SLASH => {
                self.eval_number_binary_op(left, right, |l, r| l / r, "/", expr.operator().clone())
            }
            TokenType::STAR => {
                self.eval_number_binary_op(left, right, |l, r| l * r, "*", expr.operator().clone())
            }
            TokenType::PLUS => match (right, left) {
                (Literal::Number(l), Literal::Number(r)) => self.eval_number_binary_op(
                    Literal::Number(l),
                    Literal::Number(r),
                    |l, r| l + r,
                    "+",
                    expr.operator().clone(),
                ),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::new(
                    expr.operator().clone(),
                    "Operands must be two numbers or two strings",
                )),
            },
            TokenType::BANG_EQUAL => Ok(Literal::Boolean(!self.is_equals(left, right))),
            TokenType::EQUAL_EQUAL => Ok(Literal::Boolean(self.is_equals(left, right))),
            _ => Err(RuntimeError::new(
                expr.operator().clone(),
                "Binary expression error",
            )),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &crate::expr::Grouping,
    ) -> Result<Literal, RuntimeError> {
        Ok(self.evaluate(&expr.expression())?)
    }

    fn visit_literal_expr(&self, expr: &crate::expr::Literal) -> Result<Literal, RuntimeError> {
        Ok(expr.value().clone())
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr.right());

        match expr.operator().type_ {
            TokenType::MINUS => match right {
                Ok(Literal::Number(n)) => return Ok(Literal::Number(-n)),
                _ => Err(RuntimeError::new(
                    expr.operator().clone(),
                    "Error casting number",
                )),
            },
            TokenType::BANG => match right {
                Ok(right) => return Ok(Literal::Boolean(self.is_truthy(right))),
                _ => Err(RuntimeError::new(
                    expr.operator().clone(),
                    "Unary must have a valid value",
                )),
            },
            _ => Err(RuntimeError::new(
                expr.operator().clone(),
                "Unary expression error",
            )),
        }
    }

    fn visit_variable_expr(
        &mut self,
        expr: &crate::expr::Variable,
    ) -> Result<Literal, RuntimeError> {
        self.environment.get(expr.name().clone())
    }

    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(expr.value());
        match value {
            Ok(value) => {
                if let Err(err) = self.environment.assign(expr.name().clone(), value.clone()) {
                    return Err(err);
                }
                Ok(value)
            }
            Err(_) => Err(RuntimeError::new(
                expr.name().clone(),
                "Assign expression error",
            )),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(None),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Err(error) = self.execute(statement) {
                return Err(error);
            }
        }
        Ok(())
        // let value = self.evaluate(&expression);
        // let _ = match value {
        //     Ok(value) => println!("Value: {}", self.stringify(value)),
        //     Err(err) => {
        //         crate::run_time_error(err);
        //     }
        // };
    }

    pub fn execute(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expression) => return expression.accept(self),
            Stmt::Print(print) => print.accept(self),
            Stmt::Var(var) => var.accept(self),
            Stmt::Block(block) => block.accept(self),
        }
    }

    fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let actual = self.environment.clone();
        let previous = self.environment.clone();
        match actual.into() {
            Some(_) => {
                self.environment = environment;

                for statement in statements {
                    self.execute(statement)?;
                }

                self.environment = previous;
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Binary(binary) => binary.accept(self),
            Expr::Grouping(grouping) => grouping.accept(self),
            Expr::Literal(literal) => literal.accept(self),
            Expr::Unary(unary) => unary.accept(self),
            Expr::Variable(variable) => variable.accept(self),
            Expr::Assign(assign) => assign.accept(self),
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
        token: Token,
    ) -> Result<Literal, RuntimeError> {
        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => Ok(Literal::Number(op(left, right))),
            _ => Err(RuntimeError::new(
                token,
                &format!("Operator '{}' requires two numbers", op_name),
            )),
        }
    }

    fn eval_comparison_binary_op<F: Fn(f64, f64) -> bool>(
        &self,
        left: Literal,
        right: Literal,
        op: F,
        op_name: &str,
        token: Token,
    ) -> Result<Literal, RuntimeError> {
        match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => {
                Ok(Literal::Boolean(op(left, right)))
            }
            _ => Err(RuntimeError::new(
                token,
                &format!("Operator '{}' requires two numbers", op_name),
            )),
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

    fn stringify(&self, value: Literal) -> String {
        match value {
            Literal::Null => return String::from("nil"),
            Literal::Number(value) => {
                let mut text = value.to_string();
                if text.ends_with(".0") {
                    text = text[..text.len() - 2].to_string();
                }
                return text;
            }
            Literal::String(str) => {
                return str;
            }
            Literal::Boolean(bool) => return bool.to_string(),
        }
    }
}
