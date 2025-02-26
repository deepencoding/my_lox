use std::usize;

use super::token_type::TokenType;
use super::literals::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    _line: usize,
    _col: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize, col: usize) -> Self {
        Self { token_type, lexeme: lexeme.to_owned(), literal, _line: line, _col: col }
    }

    pub fn to_owned(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}