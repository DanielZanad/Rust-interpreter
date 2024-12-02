use std::cell::{Ref, RefCell};

use crate::token::Token;

use crate::token::Literal;
use crate::token_type::*;

pub struct Scanner {
    source: String,
    tokens: RefCell<Vec<Token>>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new().into(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Ref<'_, Vec<Token>> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.borrow_mut().push(Token::new(
            TokenType::EOF,
            String::new(),
            Literal::Nil,
            self.line,
        ));
        return self.tokens.borrow();
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token_nil(TokenType::LEFT_PAREN),
            ')' => self.add_token_nil(TokenType::RIGHT_PAREN),
            '{' => self.add_token_nil(TokenType::LEFT_BRACE),
            '}' => self.add_token_nil(TokenType::RIGHT_BRACE),
            ',' => self.add_token_nil(TokenType::COMMA),
            '.' => self.add_token_nil(TokenType::DOT),
            '-' => self.add_token_nil(TokenType::MINUS),
            '+' => self.add_token_nil(TokenType::PLUS),
            ';' => self.add_token_nil(TokenType::SEMICOLON),
            '*' => self.add_token_nil(TokenType::STAR),
            _ => {}
        }
    }

    fn advance(&mut self) ->char {
        let current_char = self.source.chars().nth(self.current as usize);
        self.current += 1;

        match current_char {
            Some(c) => c,
            None => '\0'
        }
    }

    fn add_token_nil(&mut self, _type: TokenType) {
        self.add_token_literal(_type, Literal::Nil);
    }

    fn add_token_literal(&mut self, _type: TokenType, literal: Literal) {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens.borrow_mut().push(Token::new(_type, text, literal, self.line));
    }


    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}
