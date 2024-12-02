use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    Nil,
}

pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
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
