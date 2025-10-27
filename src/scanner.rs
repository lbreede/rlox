use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub enum ScanError {
    UnexpectedCharacter(u8),
    UnterminatedString,
}

#[derive(Debug)]
pub struct Scanner {
    source: Box<[u8]>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: Box::from(source.as_bytes()),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, ScanError> {
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
            _ => Err(ScanError::UnexpectedCharacter(c)),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, kind: TokenKind) -> Result<Token, ScanError> {
        Ok(Token {
            kind,
            line: self.line,
        })
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

    fn string(&mut self) -> Result<Token, ScanError> {
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
            return Err(ScanError::UnterminatedString);
        }

        // Consume the closing quote
        self.advance();

        let lexeme = String::from_utf8_lossy(&self.source[self.start..self.current]);
        self.make_token(TokenKind::String(lexeme.into()))
    }

    fn number(&mut self) -> Result<Token, ScanError> {
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

        let lexeme = String::from_utf8_lossy(&self.source[self.start..self.current]);
        Ok(Token {
            kind: TokenKind::Number(lexeme.into()),
            line: self.line,
        })
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn identifier(&mut self) -> Result<Token, ScanError> {
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
            _ => TokenKind::Identifier(String::from_utf8_lossy(lexeme).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        let source = r#"andy formless fo _ _123 _abc ab123
abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_"#;
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::identifier("andy", 1)),
            Ok(Token::identifier("formless", 1)),
            Ok(Token::identifier("fo", 1)),
            Ok(Token::identifier("_", 1)),
            Ok(Token::identifier("_123", 1)),
            Ok(Token::identifier("_abc", 1)),
            Ok(Token::identifier("ab123", 1)),
            Ok(Token::identifier(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                2,
            )),
            Ok(Token::new(TokenKind::Eof, 2)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn keywords() {
        let source = "and class else false for fun if nil or return super this true var while";
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::new(TokenKind::And, 1)),
            Ok(Token::new(TokenKind::Class, 1)),
            Ok(Token::new(TokenKind::Else, 1)),
            Ok(Token::new(TokenKind::False, 1)),
            Ok(Token::new(TokenKind::For, 1)),
            Ok(Token::new(TokenKind::Fun, 1)),
            Ok(Token::new(TokenKind::If, 1)),
            Ok(Token::new(TokenKind::Nil, 1)),
            Ok(Token::new(TokenKind::Or, 1)),
            Ok(Token::new(TokenKind::Return, 1)),
            Ok(Token::new(TokenKind::Super, 1)),
            Ok(Token::new(TokenKind::This, 1)),
            Ok(Token::new(TokenKind::True, 1)),
            Ok(Token::new(TokenKind::Var, 1)),
            Ok(Token::new(TokenKind::While, 1)),
            Ok(Token::new(TokenKind::Eof, 1)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn numbers() {
        let source = r#"123
123.456
.456
123."#;
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::new(TokenKind::Number("123".to_string()), 1)),
            Ok(Token::new(TokenKind::Number("123.456".to_string()), 2)),
            Ok(Token::new(TokenKind::Dot, 3)),
            Ok(Token::new(TokenKind::Number("456".to_string()), 3)),
            Ok(Token::new(TokenKind::Number("123".to_string()), 4)),
            Ok(Token::new(TokenKind::Dot, 4)),
            Ok(Token::new(TokenKind::Eof, 4)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn punctuators() {
        let source = "(){};,+-*!===<=>=!=<>/.";
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::new(TokenKind::LeftParen, 1)),
            Ok(Token::new(TokenKind::RightParen, 1)),
            Ok(Token::new(TokenKind::LeftBrace, 1)),
            Ok(Token::new(TokenKind::RightBrace, 1)),
            Ok(Token::new(TokenKind::Semicolon, 1)),
            Ok(Token::new(TokenKind::Comma, 1)),
            Ok(Token::new(TokenKind::Plus, 1)),
            Ok(Token::new(TokenKind::Minus, 1)),
            Ok(Token::new(TokenKind::Star, 1)),
            Ok(Token::new(TokenKind::BangEqual, 1)),
            Ok(Token::new(TokenKind::EqualEqual, 1)),
            Ok(Token::new(TokenKind::LessEqual, 1)),
            Ok(Token::new(TokenKind::GreaterEqual, 1)),
            Ok(Token::new(TokenKind::BangEqual, 1)),
            Ok(Token::new(TokenKind::Less, 1)),
            Ok(Token::new(TokenKind::Greater, 1)),
            Ok(Token::new(TokenKind::Slash, 1)),
            Ok(Token::new(TokenKind::Dot, 1)),
            Ok(Token::new(TokenKind::Eof, 1)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn strings() {
        let source = r#"""
"string""#;
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::new(TokenKind::String("\"\"".to_string()), 1)),
            Ok(Token::new(TokenKind::String("\"string\"".to_string()), 2)),
            Ok(Token::new(TokenKind::Eof, 2)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn whitespace() {
        let source = r#"space    tabs				newlines




end"#;
        let mut scanner = Scanner::new(source);
        let expected_tokens = vec![
            Ok(Token::identifier("space", 1)),
            Ok(Token::identifier("tabs", 1)),
            Ok(Token::identifier("newlines", 1)),
            Ok(Token::identifier("end", 6)),
            Ok(Token::new(TokenKind::Eof, 6)),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }
}
