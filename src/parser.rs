use std::{clone, fmt::Error, rc::Rc};

use crate::{
    expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable},
    literal_object::Literal as LiteralValue,
    stmt::{Block, Expression, Print, Stmt, Var},
    token::Token,
    token_type::TokenType::{self, *},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: u32,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        statements

        // match self.expression() {
        //     Ok(expr) => return Ok(expr),
        //     Err(err) => Err(ParseError::new(err.to_string())),
        // }
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(vec![PRINT]) {
            return self.print_statement();
        }
        if self.match_token(vec![LEFT_BRACE]) {
            return Stmt::Block(Rc::new(Block::new(self.block())));
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression().unwrap();
        self.consume(SEMICOLON, "Expect ';' after value.");
        return Stmt::Print(Rc::new(Print::new(value)));
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        if let Ok(name) = self.consume(IDENTIFIER, "Expect variable name.") {
            let mut initializer = None;
            let name = name.clone();

            if self.match_token(vec![EQUAL]) {
                initializer = Some(self.expression());
            }
            self.consume(SEMICOLON, "Expect ';' after variable declaration");
            match initializer {
                Some(initializer) => Ok(Stmt::Var(Rc::new(Var::new(
                    name.clone(),
                    initializer.unwrap_or_else(|_| {
                        Expr::Literal(Rc::new(Literal::new(LiteralValue::Null)))
                    }),
                )))),
                None => Err(ParseError::new("Error when parsing".to_string())),
            }
        } else {
            Err(ParseError::new("Error when parsing".to_string()))
        }
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression().unwrap();
        self.consume(SEMICOLON, "Expect ';' after expression");
        Stmt::Expression(Rc::new(Expression::new(expr)))
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        self.consume(RIGHT_BRACE, "Expect '}' after block");
        statements
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality();

        if self.match_token(vec![EQUAL]) {
            let equals = self.previous();
            let equals = equals.clone();
            let value = self.assignment();

            match expr {
                Ok(ref expr) => match expr {
                    Expr::Variable(variable) => {
                        let name = variable.name();
                        match value {
                            Ok(value) => {
                                return Ok(Expr::Assign(Rc::new(Assign::new(name.clone(), value))))
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    _ => {
                        crate::token_error(&equals, "Invalid assignment target");
                        return Err(ParseError {
                            message: "Invalid assignment target".to_string(),
                        });
                    }
                },
                Err(_) => {
                    crate::token_error(&equals, "Invalid assignment target");
                    return Err(ParseError {
                        message: "Invalid assignment target".to_string(),
                    });
                }
            }
        }
        return expr;
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.assignment();
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![VAR]) {
            match self.var_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(error) => return Err(error),
            }
        }

        return Ok(self.statement());
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        while self.match_token(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Ok(Expr::Binary(Rc::new(Binary::new(
                expr.unwrap(),
                operator,
                right.unwrap(),
            ))));
        }

        match expr {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParseError::new(
                "Error parsing equality expression".to_string(),
            )),
        }
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        while self.match_token(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Ok(Expr::Binary(Rc::new(Binary::new(
                expr.unwrap(),
                operator,
                right.unwrap(),
            ))))
        }

        match expr {
            Ok(expr) => Ok(expr),
            Err(error) => Err(error),
        }
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();

        while self.match_token(vec![MINUS, PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Ok(Expr::Binary(Rc::new(Binary::new(
                expr.unwrap(),
                operator,
                right.unwrap(),
            ))))
        }

        match expr {
            Ok(expr) => Ok(expr),
            Err(error) => Err(error),
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();

        while self.match_token(vec![SLASH, STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Ok(Expr::Binary(Rc::new(Binary::new(
                expr.unwrap(),
                operator,
                right.unwrap(),
            ))));
        }

        match expr {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParseError::new(
                "Error parsing a factor expression".to_string(),
            )),
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![BANG, MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(Unary::new(operator, right))));
        }

        match self.primary() {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParseError::new(
                "Error parsing a unary expression".to_string(),
            )),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![FALSE]) {
            return Ok(Expr::Literal(Rc::new(Literal::new(LiteralValue::Boolean(
                false,
            )))));
        }
        if self.match_token(vec![TRUE]) {
            return Ok(Expr::Literal(Rc::new(Literal::new(LiteralValue::Boolean(
                true,
            )))));
        }
        if self.match_token(vec![NIL]) {
            return Ok(Expr::Literal(Rc::new(Literal::new(LiteralValue::Null))));
        }

        if self.match_token(vec![NUMBER, STRING]) {
            return Ok(Expr::Literal(Rc::new(Literal::new(
                self.previous().literal.clone(),
            ))));
        }

        if self.match_token(vec![IDENTIFIER]) {
            return Ok(Expr::Variable(Rc::new(Variable::new(
                self.previous().clone(),
            ))));
        }

        if self.match_token(vec![LEFT_PAREN]) {
            let expr = self.expression();
            match self.consume(RIGHT_PAREN, "Expect ')' after expression") {
                Ok(_) => match expr {
                    Ok(expr) => return Ok(Expr::Grouping(Rc::new(Grouping::new(expr)))),
                    Err(_) => {
                        return Err(ParseError::new(
                            "Error parsing a primary expression".to_string(),
                        ))
                    }
                },
                Err(error) => {
                    return Err(ParseError::new(
                        "Error parsing a primary expression".to_string(),
                    ))
                }
            };
        }
        self.error(self.peek(), "Expect expression");
        Err(ParseError::new(
            "Error parsing a primary expression".to_string(),
        ))
    }

    fn consume(&mut self, type_: TokenType, message: &'static str) -> Result<&Token, Error> {
        if self.check(type_) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &'static str) -> Error {
        crate::token_error(token, message);
        Error
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().type_ == type_
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().type_ == SEMICOLON {
                return;
            }

            match self.peek().type_ {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        let _type = self.peek().type_;
        _type == TokenType::EOF
    }
    fn peek(&self) -> &Token {
        let token = self.tokens.get(self.current as usize).unwrap();
        token
    }
    fn previous(&mut self) -> &Token {
        let token = self.tokens.get((self.current as usize) - 1).unwrap();
        token
    }
}
