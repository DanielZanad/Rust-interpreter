use crate::{literal_object::Literal, token_type::TokenType};

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u64,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Literal, line: u64) -> Self {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {} {:?}", self.type_, self.lexeme, self.literal);
    }
}
