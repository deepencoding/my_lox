#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single Char
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // Single or Double Char
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Lesser, LesserEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}