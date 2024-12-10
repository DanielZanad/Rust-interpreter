use std::cell::{Ref, RefCell};
use std::collections::HashMap;

use crate::error;
use crate::token::Literal;
use crate::token::Token;
use crate::token_type::*;

pub struct Scanner<'a> {
    source: String,
    tokens: RefCell<Vec<Token>>,
    keywords: HashMap<&'a str, TokenType>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner<'_> {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("and", TokenType::AND);
        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);

        Self {
            source,
            tokens: Vec::new().into(),
            keywords,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Ref<'_, Vec<Token>> {
        while !self.is_at_end() {
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
            '!' => {
                let lexeme = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };

                self.add_token_nil(lexeme);
            }
            '=' => {
                let lexeme = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_nil(lexeme);
            }
            '<' => {
                let lexeme = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_nil(lexeme);
            }
            '>' => {
                let lexeme = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_nil(lexeme);
            }
            '/' => {
                if self.match_lexeme('/') {
                    loop {
                        if self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        } else {
                            self.add_token_nil(TokenType::SLASH);
                            break;
                        }
                    }
                } else if self.match_lexeme('*') {
                    loop {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        if self.peek() != '/' && !self.is_at_end() {
                            self.advance();
                        } else {
                            self.add_token_nil(TokenType::SLASH);
                            break;
                        }
                    }
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character");
                }
            }
        }
    }
    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start as usize..self.current as usize].to_string();
        let _type = self.keywords.get(text.as_str());

        if let Some(_type) = _type {
            self.add_token_nil(_type.clone());
        }

        self.add_token_nil(TokenType::IDENTIFIER);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        // The closing "
        self.advance();

        // trim the surrounding quotes
        let value = self.source[(self.start + 1) as usize..(self.current - 1) as usize].to_string();
        self.add_token_literal(TokenType::STRING, Literal::StringLiteral(value));
    }

    fn match_lexeme(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(current) = self.source.chars().nth(self.current as usize) {
            if current != expected {
                return false;
            }
        };

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        if let Some(current) = self.source.chars().nth(self.current as usize) {
            return current;
        } else {
            return '\0';
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        }
        if let Some(next_char) = self.source.chars().nth(self.current as usize) {
            next_char
        } else {
            '\0'
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let number = self.source[self.start as usize..self.current as usize]
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(TokenType::NUMBER, Literal::NumberLiteral(number));
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current as usize);
        self.current += 1;

        match current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    fn add_token_nil(&mut self, _type: TokenType) {
        self.add_token_literal(_type, Literal::Nil);
    }

    fn add_token_literal(&mut self, _type: TokenType, literal: Literal) {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens
            .borrow_mut()
            .push(Token::new(_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}
