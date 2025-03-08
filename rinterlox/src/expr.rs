use crate::scanner::token::Token;

#[derive(Debug, Clone)]
pub enum AstLiteral {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum Expr {
    Literal(AstLiteral),
    Unary { unary_operator: Token, expression: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
}