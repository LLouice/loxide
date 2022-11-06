pub struct Lexer<'s> {
    source: &'s str,
} 

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source
        }
    }

    pub fn parse_tokens(&self) -> Vec<&str> {
        let tokens = self.source.split_ascii_whitespace().collect();
        tokens
    }
}
