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

    // keywords
    Keyword(Keyword),

    Eof,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
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
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "and" => Some(Self::And),
            "class" => Some(Self::Class),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "fun" => Some(Self::Fun),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "nil" => Some(Self::Nil),
            "or" => Some(Self::Or),
            "print" => Some(Self::Print),
            "return" => Some(Self::Return),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::True),
            "var" => Some(Self::Var),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

// token with location information
#[derive(Debug, Clone, Copy)]
pub struct Token<'s> {
    pub token_type: TokenType<'s>,
    pub lexeme: &'s str,
    pub line: usize,
}

impl<'s> Token<'s> {
    pub fn new(token_type: TokenType<'s>, lexeme: &'s str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl<'s> std::fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}
