use crate::tokens::{Token, TokenType};

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

    pub fn scan_tokens(&mut self) -> Vec<Token<'s>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // Eof
        tokens.push(Token::new(TokenType::Eof, "", self.line));

        tokens
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

            x @ _ => eprintln!("Unexpect character {} at line {}", x, self.line), // eprintln and skip it, not stop scanning for report all errors at once
        }
    }

    /// after call this function, return current char, and `self.current` is next position index
    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType<'s>) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        ));
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn current_char(&self) -> char {
        self.char_at(self.current)
    }

    fn char_at(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap()
    }
}
