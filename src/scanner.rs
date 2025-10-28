use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source[self.current..].chars().next()
        }
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.source[self.current..].chars();
        iter.next(); // skip current
        iter.next()
    }

    fn match_byte(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    // Example use inside string():
    fn string(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance(); // consume closing quote

        let lexeme = self.lexeme().to_owned();
        self.make_token(TokenKind::String(lexeme))
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
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '-' => self.make_token(TokenKind::Minus),
            '+' => self.make_token(TokenKind::Plus),
            '/' => self.make_token(TokenKind::Slash),
            '*' => self.make_token(TokenKind::Star),
            '!' => {
                let kind = if self.match_byte('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.make_token(kind)
            }
            '=' => {
                let kind = if self.match_byte('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(kind)
            }
            '<' => {
                let kind = if self.match_byte('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.make_token(kind)
            }
            '>' => {
                let kind = if self.match_byte('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.make_token(kind)
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
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

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        // Skip comment until newline or end-of-input
                        while let Some(c) = self.peek() {
                            if c == '\n' {
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

    fn number(&mut self) -> Token {
        // Consume the integer part
        while let Some(b) = self.peek() {
            if !b.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == Some('.') && self.peek_next().is_some_and(|b| b.is_ascii_digit()) {
            self.advance(); // consume the '.'

            // Consume the fractional digits
            while let Some(b) = self.peek() {
                if !b.is_ascii_digit() {
                    break;
                }
                self.advance();
            }
        }

        // TODO: Consider converting to `f64` here instead of storing the owned `String`
        let lexeme = &self.source[self.start..self.current];
        Token {
            kind: TokenKind::Number(lexeme.to_owned()),
            line: self.line,
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn identifier(&mut self) -> Token {
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let kind = self.identifier_type(&self.source[self.start..self.current]);
        self.make_token(kind)
    }

    fn identifier_type(&self, lexeme: &str) -> TokenKind {
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
            _ => TokenKind::Identifier(lexeme.to_owned()),
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
            Token::identifier("andy", 1),
            Token::identifier("formless", 1),
            Token::identifier("fo", 1),
            Token::identifier("_", 1),
            Token::identifier("_123", 1),
            Token::identifier("_abc", 1),
            Token::identifier("ab123", 1),
            Token::identifier(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                2,
            ),
            Token::new(TokenKind::Eof, 2),
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
            Token::new(TokenKind::And, 1),
            Token::new(TokenKind::Class, 1),
            Token::new(TokenKind::Else, 1),
            Token::new(TokenKind::False, 1),
            Token::new(TokenKind::For, 1),
            Token::new(TokenKind::Fun, 1),
            Token::new(TokenKind::If, 1),
            Token::new(TokenKind::Nil, 1),
            Token::new(TokenKind::Or, 1),
            Token::new(TokenKind::Return, 1),
            Token::new(TokenKind::Super, 1),
            Token::new(TokenKind::This, 1),
            Token::new(TokenKind::True, 1),
            Token::new(TokenKind::Var, 1),
            Token::new(TokenKind::While, 1),
            Token::new(TokenKind::Eof, 1),
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
            Token::new(TokenKind::Number("123".to_string()), 1),
            Token::new(TokenKind::Number("123.456".to_string()), 2),
            Token::new(TokenKind::Dot, 3),
            Token::new(TokenKind::Number("456".to_string()), 3),
            Token::new(TokenKind::Number("123".to_string()), 4),
            Token::new(TokenKind::Dot, 4),
            Token::new(TokenKind::Eof, 4),
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
            Token::new(TokenKind::LeftParen, 1),
            Token::new(TokenKind::RightParen, 1),
            Token::new(TokenKind::LeftBrace, 1),
            Token::new(TokenKind::RightBrace, 1),
            Token::new(TokenKind::Semicolon, 1),
            Token::new(TokenKind::Comma, 1),
            Token::new(TokenKind::Plus, 1),
            Token::new(TokenKind::Minus, 1),
            Token::new(TokenKind::Star, 1),
            Token::new(TokenKind::BangEqual, 1),
            Token::new(TokenKind::EqualEqual, 1),
            Token::new(TokenKind::LessEqual, 1),
            Token::new(TokenKind::GreaterEqual, 1),
            Token::new(TokenKind::BangEqual, 1),
            Token::new(TokenKind::Less, 1),
            Token::new(TokenKind::Greater, 1),
            Token::new(TokenKind::Slash, 1),
            Token::new(TokenKind::Dot, 1),
            Token::new(TokenKind::Eof, 1),
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
            Token::new(TokenKind::String("\"\"".to_string()), 1),
            Token::new(TokenKind::String("\"string\"".to_string()), 2),
            Token::new(TokenKind::Eof, 2),
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
            Token::identifier("space", 1),
            Token::identifier("tabs", 1),
            Token::identifier("newlines", 1),
            Token::identifier("end", 6),
            Token::new(TokenKind::Eof, 6),
        ];
        for expected in expected_tokens {
            assert_eq!(scanner.scan_token(), expected);
        }
    }

    #[test]
    fn utf8() {
        let source = "var foo = \"🦀\";";
        let mut scanner = Scanner::new(source);
        for _ in 0..6 {
            println!("{:?}", scanner.scan_token());
        }
    }
}
