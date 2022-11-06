use crate::tokens::{Keyword, Token, TokenType};

pub struct Lexer<'s> {
    source: &'s str,
    tokens: Vec<Token<'s>>,
    // inner state of scanning
    /// offset of source, point to the **first** character of the lexeme being scanned
    start: usize,
    /// offset of source, point to the **current** character of the lexeme being scanned
    current: usize,
    line: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token<'s>] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // Eof
        self.tokens.push(Token::new(TokenType::Eof, "", self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        // scan single token
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // ambiguous(二义性) ['!', "!=", '=', '==', '<', "<=", '>', ">="]
            '!' => {
                let token_type = if self.advance_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.advance_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if self.advance_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }
            '>' => {
                let token_type = if self.advance_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }

            // `/` for ['/', '//']
            '/' => {
                if self.advance_match('/') {
                    // scanning util to EOL
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                };
            }

            // skip whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }

            // literal
            // string
            '"' => self.scan_string(),

            x => {
                // number
                if self.scan_number(c) {
                    return;
                }

                // identifer then reserved word
                if self.scan_ident_and_keyword(c) {
                    return;
                }

                // eprintln and skip it, not stop scanning for report all errors at once
                eprintln!(
                    "Unexpect character {} at line {}, state: {}",
                    x,
                    self.line,
                    self.pluck()
                );
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType<'s>) {
        let lexeme = self.pluck();
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn scan_ident_and_keyword(&mut self, c: char) -> bool {
        // start char
        if c.is_ascii() || c == '_' {
            loop {
                let c = self.peek();
                if c.is_ascii_alphanumeric() || c == '_' {
                    self.advance();
                } else {
                    break;
                }
            }
            let ident = self.pluck();
            if let Some(keyword) = Keyword::from_str(ident) {
                self.add_token(TokenType::Keyword(keyword));
            } else {
                self.add_token(TokenType::Identifer(ident));
            }
            true
        } else {
            false
        }
    }

    fn scan_number(&mut self, c: char) -> bool {
        if Self::is_digit(c) {
            // must match the all happy path
            // integer(pre '.') part
            while Self::is_digit(self.peek()) {
                self.advance();
            }
            // look for a fractional part and consume '.'
            if self.peek() == '.' && Self::is_digit(self.peek_next()) {
                self.advance();
            }
            // the fractional part
            while Self::is_digit(self.peek()) {
                self.advance();
            }
            // must have token, because we have match a digit then call this function
            self.add_token(TokenType::Number(
                self.pluck().parse().expect("parse number failed!"),
            ));
            true
        } else {
            false
        }
    }

    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn scan_string(&mut self) {
        // in `""`
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string.");
            return;
        }

        // current at '"'
        let string = &self.source[self.start + 1..self.current];
        self.add_token(TokenType::String(string));
    }

    /// `'s` is must used! otherwise the lexeme is tied with the whole Token, not just source!
    fn pluck(&self) -> &'s str {
        &self.source[self.start..self.current]
    }

    fn advance_match(&mut self, expected: char) -> bool {
        // peek next and next index just is `self.current` because this is called after calling
        // `self.advance`, read `self.advance` for detail.
        if self.is_at_end() {
            return false;
        }
        if self.current_char() == expected {
            // advance(consume the lexeme)
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// after call this function, return current char, and `self.current` is next position index
    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    // next and next to the coverd
    fn peek_next(&self) -> char {
        let index = self.current + 1;
        if index >= self.source.len() {
            '\0'
        } else {
            self.char_at(index)
        }
    }

    // next to the coverd
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn current_char(&self) -> char {
        self.char_at(self.current)
    }

    fn char_at(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
