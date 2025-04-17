use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Null,
}
#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: u32,
}

impl Token {
    pub fn new(_type: TokenType, lexeme: String, literal: Literal, line: u32) -> Self {
        Token {
            _type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self._type, self.lexeme, self.literal)
    }
}
