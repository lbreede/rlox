use crate::value::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number(Value),
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    Error,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a [u8],
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a [u8], line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lexeme = std::str::from_utf8(&self.lexeme).unwrap();
        match self.kind {
            TokenKind::Number(value) => {
                if value == value.trunc() {
                    write!(f, "NUMBER {} {}.0", lexeme, value)
                } else {
                    write!(f, "NUMBER {} {}", lexeme, value)
                }
            }
            TokenKind::String => {
                write!(f, "STRING {} {}", lexeme, &lexeme[1..lexeme.len() - 1])
            }
            _ => {
                let kind = format!("{:?}", self.kind).to_uppercase();
                write!(f, "{} {} null", kind, lexeme)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(
            Token::new(TokenKind::And, b"and", 1).to_string(),
            "AND and null"
        );
        assert_eq!(
            Token::new(TokenKind::Class, b"class", 1).to_string(),
            "CLASS class null"
        );
    }

    #[test]
    fn numbers() {
        let token = Token::new(TokenKind::Number(123.0), b"123", 1);
        assert_eq!(token.to_string(), "NUMBER 123 123.0");
        let token = Token::new(TokenKind::Number(123.456), b"123.456", 2);
        assert_eq!(token.to_string(), "NUMBER 123.456 123.456");
    }

    #[test]
    fn strings() {
        let token = Token::new(TokenKind::String, b"\"\"", 1);
        assert_eq!(token.to_string(), "STRING \"\" ");
        let token = Token::new(TokenKind::String, b"\"string\"", 1);
        assert_eq!(token.to_string(), "STRING \"string\" string");
    }
}
