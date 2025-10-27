use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Interner {
    map: HashMap<String, usize>,
    vec: Vec<String>,
}

impl Interner {
    fn intern(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.map.get(name) {
            return idx;
        }
        let idx = self.map.len();
        self.map.insert(name.to_owned(), idx);
        self.vec.push(name.to_owned());

        debug_assert!(self.lookup(idx) == name);
        debug_assert!(self.intern(name) == idx);

        idx
    }

    pub fn lookup(&self, idx: usize) -> &str {
        self.vec[idx].as_str()
    }
}

#[derive(Debug)]
pub struct Scanner<'i> {
    source: Box<[u8]>,
    pub interner: &'i mut Interner,
    start: usize,
    current: usize,
    line: usize,
}

impl<'i> Scanner<'i> {
    pub fn new(source: &str, interner: &'i mut Interner) -> Self {
        Self {
            source: Box::from(source.as_bytes()),
            interner,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
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

    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            kind: TokenKind::Error(message.to_string()),
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

    fn string(&mut self) -> Token {
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

        let lexeme = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let idx = self.interner.intern(lexeme);
        self.make_token(TokenKind::String(idx))
    }

    fn number(&mut self) -> Token {
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

        let lexeme = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let idx = self.interner.intern(lexeme);
        self.make_token(TokenKind::Number(idx))
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn identifier(&mut self) -> Token {
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = String::from_utf8_lossy(&self.source[self.start..self.current]).to_string();
        let kind = self.identifier_type(&lexeme);
        self.make_token(kind)
    }

    fn identifier_type(&mut self, lexeme: &str) -> TokenKind {
        match lexeme {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => {
                let idx = self.interner.intern(lexeme);
                TokenKind::Identifier(idx)
            }
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

        let mut interner = Interner::default();
        let mut scanner = Scanner::new(source, &mut interner);

        let expected = vec![
            ("andy", 1),
            ("formless", 1),
            ("fo", 1),
            ("_", 1),
            ("_123", 1),
            ("_abc", 1),
            ("ab123", 1),
            (
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                2,
            ),
        ];

        for (lexeme, line) in expected {
            let token = scanner.scan_token();
            match &token.kind {
                TokenKind::Identifier(idx) => {
                    assert_eq!(scanner.interner.lookup(*idx), lexeme);
                }
                _ => panic!("expected identifier, got {:?}", token.kind),
            }
            assert_eq!(token.line, line);
        }

        // final token should be EOF
        let eof = scanner.scan_token();
        assert!(matches!(eof.kind, TokenKind::Eof));

        // verify lexeme for EOF using TryFrom
        assert_eq!(<&str>::try_from(&eof.kind).unwrap(), " ");
    }

    #[test]
    fn it_works() {
        let source = r#"var foo = 42;
var bar = 27;
foo = foo + bar;
"#;
        let mut interner = Interner::default();
        let mut scanner = Scanner::new(source, &mut interner);
        let expected = vec![
            ("var", 1),
            ("foo", 1),
            ("=", 1),
            ("42", 1),
            (";", 1),
            ("var", 2),
            ("bar", 2),
            ("=", 2),
            ("27", 2),
            (";", 2),
            ("foo", 3),
            ("=", 3),
            ("foo", 3),
            ("+", 3),
            ("bar", 3),
            (";", 3),
            (" ", 4),
        ];
        for (lexeme, line) in expected {
            let token = scanner.scan_token();
            println!("{token:?}");
            match &token.kind {
                TokenKind::Identifier(idx) | TokenKind::Number(idx) => {
                    assert_eq!(scanner.interner.lookup(*idx), lexeme);
                }
                kind => assert_eq!(<&str>::try_from(kind), Ok(lexeme)),
            }
            assert_eq!(token.line, line);
        }
        println!("{:?}", scanner.interner);
    }

    //     #[test]
    //     fn keywords() {
    //         let source = "and class else false for fun if nil or return super this true var while";
    //         let mut scanner = Scanner::new(source);
    //         let expected_tokens = vec![
    //             Token::new(TokenKind::And, 1),
    //             Token::new(TokenKind::Class, 1),
    //             Token::new(TokenKind::Else, 1),
    //             Token::new(TokenKind::False, 1),
    //             Token::new(TokenKind::For, 1),
    //             Token::new(TokenKind::Fun, 1),
    //             Token::new(TokenKind::If, 1),
    //             Token::new(TokenKind::Nil, 1),
    //             Token::new(TokenKind::Or, 1),
    //             Token::new(TokenKind::Return, 1),
    //             Token::new(TokenKind::Super, 1),
    //             Token::new(TokenKind::This, 1),
    //             Token::new(TokenKind::True, 1),
    //             Token::new(TokenKind::Var, 1),
    //             Token::new(TokenKind::While, 1),
    //             Token::new(TokenKind::Eof, 1),
    //         ];
    //         for expected in expected_tokens {
    //             assert_eq!(scanner.scan_token(), expected);
    //         }
    //     }
    //
    //     #[test]
    //     fn numbers() {
    //         let source = r#"123
    // 123.456
    // .456
    // 123."#;
    //         let mut scanner = Scanner::new(source);
    //         let expected_tokens = vec![
    //             Token::new(TokenKind::Number("123".to_string()), 1),
    //             Token::new(TokenKind::Number("123.456".to_string()), 2),
    //             Token::new(TokenKind::Dot, 3),
    //             Token::new(TokenKind::Number("456".to_string()), 3),
    //             Token::new(TokenKind::Number("123".to_string()), 4),
    //             Token::new(TokenKind::Dot, 4),
    //             Token::new(TokenKind::Eof, 4),
    //         ];
    //         for expected in expected_tokens {
    //             assert_eq!(scanner.scan_token(), expected);
    //         }
    //     }
    //
    //     #[test]
    //     fn punctuators() {
    //         let source = "(){};,+-*!===<=>=!=<>/.";
    //         let mut scanner = Scanner::new(source);
    //         let expected_tokens = vec![
    //             Token::new(TokenKind::LeftParen, 1),
    //             Token::new(TokenKind::RightParen, 1),
    //             Token::new(TokenKind::LeftBrace, 1),
    //             Token::new(TokenKind::RightBrace, 1),
    //             Token::new(TokenKind::Semicolon, 1),
    //             Token::new(TokenKind::Comma, 1),
    //             Token::new(TokenKind::Plus, 1),
    //             Token::new(TokenKind::Minus, 1),
    //             Token::new(TokenKind::Star, 1),
    //             Token::new(TokenKind::BangEqual, 1),
    //             Token::new(TokenKind::EqualEqual, 1),
    //             Token::new(TokenKind::LessEqual, 1),
    //             Token::new(TokenKind::GreaterEqual, 1),
    //             Token::new(TokenKind::BangEqual, 1),
    //             Token::new(TokenKind::Less, 1),
    //             Token::new(TokenKind::Greater, 1),
    //             Token::new(TokenKind::Slash, 1),
    //             Token::new(TokenKind::Dot, 1),
    //             Token::new(TokenKind::Eof, 1),
    //         ];
    //         for expected in expected_tokens {
    //             assert_eq!(scanner.scan_token(), expected);
    //         }
    //     }
    //
    //     #[test]
    //     fn strings() {
    //         let source = r#"""
    // "string""#;
    //         let mut scanner = Scanner::new(source);
    //         let expected_tokens = vec![
    //             Token::new(TokenKind::String("\"\"".to_string()), 1),
    //             Token::new(TokenKind::String("\"string\"".to_string()), 2),
    //             Token::new(TokenKind::Eof, 2),
    //         ];
    //         for expected in expected_tokens {
    //             assert_eq!(scanner.scan_token(), expected);
    //         }
    //     }
    //
    //     #[test]
    //     fn whitespace() {
    //         let source = r#"space    tabs				newlines
    //
    //
    //
    //
    // end"#;
    //         let mut scanner = Scanner::new(source);
    //         let expected_tokens = vec![
    //             Token::identifier("space", 1),
    //             Token::identifier("tabs", 1),
    //             Token::identifier("newlines", 1),
    //             Token::identifier("end", 6),
    //             Token::new(TokenKind::Eof, 6),
    //         ];
    //         for expected in expected_tokens {
    //             assert_eq!(scanner.scan_token(), expected);
    //         }
    //     }
}
