use std::usize;

use super::token_type::TokenType;
use super::literals::Literal;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Self { token_type, lexeme: lexeme.to_string(), literal, line }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}