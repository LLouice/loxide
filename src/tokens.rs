#[derive(Debug, Clone, Copy)]
pub enum TokenType<'t> {
    // Single-character tokens
    LeftParen, // "("
    RightParen, // ")"
    LeftBrace, // "{"
    RightBrace, // "}"
    Comma, // ","
    Dot, // "."
    Minus, // "-"
    Plus, // "+"
    Semicolon, // ";"
    Slash, // "/"
    Star, // "*"//
    
    // One or two character tokens
    Bang, // "!"
    BangEqual, // "!="
    Equal, // "="
    EqualEqual, // "=="
    Greater, // ">"
    GreaterEqual, // ">="
    Less, // "<"
    LessEqual, // "<="
    
    // Literals
    Identifer(&'t str),
    String(&'t str),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof
}
