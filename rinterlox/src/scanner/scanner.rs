use std::collections::HashMap;

use crate::init::Lox;
use super::token::Token;
use super::token_type::TokenType;
use crate::expr::AstLiteral;

/*
* start: field points to the first character in the lexeme
* current points at the character currently being considered
* line: field tracks what source line current is on
* col: field tracks what column current is on
*/
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    col: usize,
    reserved_map: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_owned(), tokens: vec![] , start: 0, current: 0, line: 0, col: 0,
        reserved_map: HashMap::from([
            ("and".to_owned(),    TokenType::And),
            ("class".to_owned(),  TokenType::Class),
            ("else".to_owned(),   TokenType::Else),
            ("false".to_owned(),  TokenType::False),
            ("for".to_owned(),    TokenType::For),
            ("fun".to_owned(),    TokenType::Fun),
            ("if".to_owned(),     TokenType::If),
            ("nil".to_owned(),    TokenType::Nil),
            ("or".to_owned(),     TokenType::Or),
            ("print".to_owned(),  TokenType::Print),
            ("return".to_owned(), TokenType::Return),
            ("super".to_owned(),  TokenType::Super),
            ("this".to_owned(),   TokenType::This),
            ("true".to_owned(),   TokenType::True),
            ("var".to_owned(),    TokenType::Var),
            ("while".to_owned(),  TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line, self.col));
        self.tokens.clone()
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c: char = self.advance();
        match c {
            // Single Char Tokens
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),

            // Double Char Tokens
            '!' => {
                let token_type = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type);
            },
            '=' => {
                let token_type = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type);
            },
            '<' => {
                let token_type = if self.match_char('=') { TokenType::LesserEqual } else { TokenType::Lesser };
                self.add_token(token_type);
            },
            '>' => {
                let token_type = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type);
            },

            // Special Handling Tokens
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                /* } else if self.match_char('*') {
                    while !self.is_at_end() && self.peek() != '*' &&*/
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // Other meaningless Chars
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,

            // String
            '"' => self.string(lox),

            // Invalid Character
            _ => {
                if c.is_ascii_digit() { self.number(); }
                else if c == '_' || c.is_ascii_alphabetic() { self.identifier(); }
                else { lox.error(self.line, self.col, "Unexpected Character"); }
            },
        }
    }

    fn identifier(&mut self) {
        while self.peek() == '_' || self.peek().is_ascii_alphanumeric() { self.advance(); }

        let text = &self.source[self.start..self.current];
        let mut token_type: Option<TokenType>;
        if self.reserved_map.contains_key(text) {
            token_type = Some(self.reserved_map[text].clone());
        } else {
            token_type = None;
        }
        if token_type == None { token_type = Some(TokenType::Identifier); }

        self.add_token(token_type.unwrap());
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            lox.error(self.line, self.col, "Unterminated String.");
            return;
        }

        self.advance();
        let text = &self.source[self.start+1..self.current-1];
        self.add_token_w_lit(TokenType::String, Some(AstLiteral::String(text.to_owned())));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.as_bytes()[self.current] as char != expected { return false; }

        self.advance();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' } else { self.source.as_bytes()[self.current] as char }
    }

    fn peek_next(&self) -> char {
        if self.current+1 >= self.source.len() { '\0' } else { self.source.as_bytes()[self.current+1] as char }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        let text = &self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token_w_lit(TokenType::Number, Some(AstLiteral::Number(text.to_owned())));
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_w_lit(token_type, None);
    }

    fn add_token_w_lit(&mut self, token_type: TokenType, literal: Option<AstLiteral>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line, self.col));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}