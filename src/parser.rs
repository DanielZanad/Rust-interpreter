use std::{fmt::Error, rc::Rc};

use crate::{
    expr::{Binary, Expr, Grouping, Literal, Unary},
    literal_object::Literal as LiteralValue,
    token::Token,
    token_type::TokenType::{self, *},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: u32,
}

pub struct ParseError {}

// TODO: Change result into panic!()
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => return Some(expr),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, Error> {
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
            Err(error) => Err(error),
        }
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
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

    fn term(&mut self) -> Result<Expr, Error> {
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

    fn factor(&mut self) -> Result<Expr, Error> {
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
            Err(error) => Err(error),
        }
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![BANG, MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(Unary::new(operator, right))));
        }

        match self.primary() {
            Ok(expr) => Ok(expr),
            Err(err) => Err(err),
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
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

        if self.match_token(vec![LEFT_PAREN]) {
            let expr = self.expression();
            match self.consume(RIGHT_PAREN, "Expect ')' after expression") {
                Ok(_) => {
                    return Ok(Expr::Grouping(Rc::new(Grouping::new(expr?))));
                }
                Err(error) => return Err(error),
            };
        }
        self.error(self.peek(), "Expect expression");
        Err(Error)
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
