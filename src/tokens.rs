#[derive(Debug, Clone, Copy)]
pub enum TokenType<'t> {
    // Single-character tokens
    LeftParen,  // "("
    RightParen, // ")"
    LeftBrace,  // "{"
    RightBrace, // "}"
    Comma,      // ","
    Dot,        // "."
    Minus,      // "-"
    Plus,       // "+"
    Semicolon,  // ";"
    Slash,      // "/"
    Star,       // "*"//

    // One or two character tokens
    // ambiguous(二义性) ['!', "!=", '=', '==', '<', "<=", '>', ">="]
    Bang,         // "!"
    BangEqual,    // "!="
    Equal,        // "="
    EqualEqual,   // "=="
    Greater,      // ">"
    GreaterEqual, // ">="
    Less,         // "<"
    LessEqual,    // "<="

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

    Eof,
}

// token with location information
pub struct Token<'s> {
    pub token_type: TokenType<'s>,
    pub lexme: &'s str,
    pub line: usize,
}

impl<'s> Token<'s> {
    pub fn new(token_type: TokenType<'s>, lexme: &'s str, line: usize) -> Self {
        Self {
            token_type,
            lexme,
            line,
        }
    }
}

impl<'s> std::fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexme)
    }
}
