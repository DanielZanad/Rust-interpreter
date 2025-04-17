use std::{collections::HashMap, io::LineWriter};

use crate::{literal_object::Literal, token::Token, token_type::TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u64,
    pub current: u64,
    pub line: u64,
    pub keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn default(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("fun".to_string(), TokenType::FUN);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("nil".to_string(), TokenType::NIL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords.insert("while".to_string(), TokenType::WHILE);

        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // we are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Null,
            self.line,
        ));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some('(') => self.add_token(TokenType::LEFT_PAREN),
            Some(')') => self.add_token(TokenType::RIGHT_PAREN),
            Some('{') => self.add_token(TokenType::LEFT_BRACE),
            Some('}') => self.add_token(TokenType::RIGHT_BRACE),
            Some(',') => self.add_token(TokenType::COMMA),
            Some('.') => self.add_token(TokenType::DOT),
            Some('-') => self.add_token(TokenType::MINUS),
            Some('+') => self.add_token(TokenType::PLUS),
            Some(';') => self.add_token(TokenType::SEMICOLON),
            Some('*') => self.add_token(TokenType::STAR),
            Some('!') => {
                match self.match_lexeme('=') {
                    true => TokenType::BANG_EQUAL,
                    false => TokenType::BANG,
                };
            }
            Some('=') => {
                match self.match_lexeme('=') {
                    true => TokenType::EQUAL_EQUAL,
                    false => TokenType::EQUAL,
                };
            }
            Some('<') => {
                match self.match_lexeme('=') {
                    true => TokenType::LESS_EQUAL,
                    false => TokenType::LESS,
                };
            }
            Some('>') => {
                match self.match_lexeme('=') {
                    true => TokenType::GREATER_EQUAL,
                    false => TokenType::GREATER,
                };
            }
            Some('/') => {
                if self.match_lexeme('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            Some(' ') => {}
            Some('\r') => {}
            Some('\t') => {}
            Some('\n') => {
                // Ignore whitespace.
                self.line = self.line + 1;
            }
            Some('"') => self.string(),
            Some(c) => {
                if self.is_digit(c) {
                    self.number();
                    return;
                } else if self.is_alpha(c) {
                    self.identifier();
                    return;
                } else {
                    crate::error(self.line, "Unexpected character");
                }
                crate::error(self.line, "Unexpected character")
            }
            None => {}
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u64 {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth((self.current as usize) + 1)
            .unwrap();
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let start = self.start as usize;
        let current = self.current as usize;

        let text = &self.source[start..current];
        let type_check = self.keywords.get(text);
        let type_;
        match type_check {
            Some(type_check) => type_ = type_check.clone(),
            None => type_ = TokenType::IDENTIFIER,
        }
        self.add_token(type_);
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c < '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.peek();
        }

        let start = self.start as usize;
        let current = self.current as usize;
        let number = &self.source[start..current];
        let number: f64 = number.parse().expect("Failed to parse number");
        self.add_token_literal(TokenType::NUMBER, Literal::Number(number));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let start = self.start as usize + 1;
        let current = self.current as usize - 1;
        let value = &self.source[start..current];
        println!("{}", value);
        self.add_token_literal(TokenType::STRING, Literal::String(value.to_string()));
    }

    fn match_lexeme(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }
        self.current = self.current + 1;
        return true;
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current as usize);
        self.current = self.current + 1;
        return c;
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_literal(type_, Literal::Null);
    }

    fn add_token_literal(&mut self, type_: TokenType, literal: Literal) {
        let start = self.start as usize;
        let current = self.current as usize;

        let text = self.source[start..current].to_string().clone();
        self.tokens
            .push(Token::new(type_, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }
}
