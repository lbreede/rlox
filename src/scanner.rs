use crate::token::{Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance().expect("advance called at end");

        if Scanner::is_alpha(c) {
            return self.identifier();
        }

        if c.is_ascii_digit() {
            return self.number();
        }

        match c {
            b'(' => self.make_token(TokenKind::LeftParen),
            b')' => self.make_token(TokenKind::RightParen),
            b'{' => self.make_token(TokenKind::LeftBrace),
            b'}' => self.make_token(TokenKind::RightBrace),
            b';' => self.make_token(TokenKind::Semicolon),
            b',' => self.make_token(TokenKind::Comma),
            b'.' => self.make_token(TokenKind::Dot),
            b'-' => self.make_token(TokenKind::Minus),
            b'+' => self.make_token(TokenKind::Plus),
            b'/' => self.make_token(TokenKind::Slash),
            b'*' => self.make_token(TokenKind::Star),
            b'!' => {
                let kind = if self.match_byte(b'=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.make_token(kind)
            }
            b'=' => {
                let kind = if self.match_byte(b'=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            b'<' => {
                let kind = if self.match_byte(b'=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.make_token(kind)
            }
            b'>' => {
                let kind = if self.match_byte(b'=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.make_token(kind)
            }
            b'"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'a> {
        Token {
            kind: TokenKind::Error,
            lexeme: message.as_bytes(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            let b = self.source[self.current];
            self.current += 1;
            Some(b)
        }
    }

    fn match_byte(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(b' ' | b'\r' | b'\t') => {
                    self.advance();
                }
                Some(b'\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some(b'/') => {
                    if self.peek_next() == Some(b'/') {
                        // Skip comment until newline or end-of-input
                        while let Some(c) = self.peek() {
                            if c == b'\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Token<'a> {
        while let Some(c) = self.peek() {
            if c == b'"' {
                break; // closing quote
            }
            if c == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // Consume the closing quote
        self.advance();

        self.make_token(TokenKind::String)
    }

    fn number(&mut self) -> Token<'a> {
        // Consume the integer part
        while let Some(b) = self.peek() {
            if !b.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == Some(b'.') && self.peek_next().is_some_and(|b| b.is_ascii_digit()) {
            self.advance(); // consume the '.'

            // Consume the fractional digits
            while let Some(b) = self.peek() {
                if !b.is_ascii_digit() {
                    break;
                }
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn identifier(&mut self) -> Token<'a> {
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let kind = self.identifier_type(&self.source[self.start..self.current]);
        self.make_token(kind)
    }

    fn identifier_type(&self, lexeme: &[u8]) -> TokenKind {
        match lexeme {
            b"and" => TokenKind::And,
            b"class" => TokenKind::Class,
            b"else" => TokenKind::Else,
            b"false" => TokenKind::False,
            b"for" => TokenKind::For,
            b"fun" => TokenKind::Fun,
            b"if" => TokenKind::If,
            b"nil" => TokenKind::Nil,
            b"or" => TokenKind::Or,
            b"print" => TokenKind::Print,
            b"return" => TokenKind::Return,
            b"super" => TokenKind::Super,
            b"this" => TokenKind::This,
            b"true" => TokenKind::True,
            b"var" => TokenKind::Var,
            b"while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_character_tokens() {
        let mut scanner = Scanner::new("(){};,.-+/*");
        let expected = vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Semicolon,
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Slash,
            TokenKind::Star,
            TokenKind::Eof,
        ];
        for expected_kind in expected {
            let token = scanner.scan_token();
            assert_eq!(token.kind, expected_kind);
        }
    }

    #[test]
    fn one_or_two_character_tokens() {
        let mut scanner = Scanner::new("! != = == < <= > >=");
        let expected = vec![
            TokenKind::Bang,
            TokenKind::BangEqual,
            TokenKind::Equal,
            TokenKind::EqualEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Eof,
        ];
        for expected_kind in expected {
            let token = scanner.scan_token();
            assert_eq!(token.kind, expected_kind);
        }
    }

    #[test]
    fn string() {
        // TODO: Make these tests more sophisticated!
        let mut scanner = Scanner::new("\"hello\"");
        assert_eq!(scanner.scan_token().kind, TokenKind::String);
        assert_eq!(scanner.scan_token().kind, TokenKind::Eof);
        let mut scanner = Scanner::new("\"world");
        assert_eq!(scanner.scan_token().kind, TokenKind::Error);
        assert_eq!(scanner.scan_token().kind, TokenKind::Eof);
    }

    #[test]
    fn keywords() {
        let mut scanner = Scanner::new(
            "and class else false for fun if nil or print return super this true var while",
        );
        let expected = vec![
            TokenKind::And,
            TokenKind::Class,
            TokenKind::Else,
            TokenKind::False,
            TokenKind::For,
            TokenKind::Fun,
            TokenKind::If,
            TokenKind::Nil,
            TokenKind::Or,
            TokenKind::Print,
            TokenKind::Return,
            TokenKind::Super,
            TokenKind::This,
            TokenKind::True,
            TokenKind::Var,
            TokenKind::While,
        ];
        for expected_kind in expected {
            let token = scanner.scan_token();
            assert_eq!(token.kind, expected_kind);
        }
    }
}
