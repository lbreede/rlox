use std::fmt;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
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
    Identifier(String),
    String(String),
    Number(String),
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
    // Other
    Error(String),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }

    pub fn identifier<S: ToString>(identifier: S, line: usize) -> Self {
        Self {
            kind: TokenKind::Identifier(identifier.to_string()),
            line,
        }
    }

    pub fn lexeme(&self) -> &str {
        match self.kind {
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Minus => "-",
            TokenKind::Plus => "+",
            TokenKind::Semicolon => ";",
            TokenKind::Slash => "/",
            TokenKind::Star => "*",
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Identifier(ref s) => s,
            TokenKind::String(ref s) => s,
            TokenKind::Number(ref s) => s,
            TokenKind::And => "and",
            TokenKind::Class => "class",
            TokenKind::Else => "else",
            TokenKind::False => "false",
            TokenKind::For => "for",
            TokenKind::Fun => "fun",
            TokenKind::If => "if",
            TokenKind::Nil => "nil",
            TokenKind::Or => "or",
            TokenKind::Print => "print",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::This => "this",
            TokenKind::True => "true",
            TokenKind::Var => "var",
            TokenKind::While => "while",
            TokenKind::Error(ref s) => s,
            TokenKind::Eof => " ",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TokenKind::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            TokenKind::String(s) => write!(f, "STRING {s} {}", &s[1..s.len() - 1]),
            TokenKind::Number(s) => {
                let value: Value = s.parse().expect("failed to parse number");
                if value == value.trunc() {
                    write!(f, "NUMBER {s} {value}.0")
                } else {
                    write!(f, "NUMBER {s} {value}")
                }
            }
            TokenKind::LeftParen => write!(f, "LEFT_PAREN ( null"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN ) null"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            TokenKind::Comma => write!(f, "COMMA , null"),
            TokenKind::Dot => write!(f, "DOT . null"),
            TokenKind::Minus => write!(f, "MINUS - null"),
            TokenKind::Plus => write!(f, "PLUS + null"),
            TokenKind::Semicolon => write!(f, "SEMICOLON ; null"),
            TokenKind::Slash => write!(f, "SLASH / null"),
            TokenKind::Star => write!(f, "STAR * null"),
            TokenKind::Bang => write!(f, "BANG ! null"),
            TokenKind::BangEqual => write!(f, "BANG_EQUAL != null"),
            TokenKind::Equal => write!(f, "EQUAL = null"),
            TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            TokenKind::Greater => write!(f, "GREATER > null"),
            TokenKind::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            TokenKind::Less => write!(f, "LESS < null"),
            TokenKind::LessEqual => write!(f, "LESS_EQUAL <= null"),
            TokenKind::And => write!(f, "AND and null"),
            TokenKind::Class => write!(f, "CLASS class null"),
            TokenKind::Else => write!(f, "ELSE else null"),
            TokenKind::False => write!(f, "FALSE false null"),
            TokenKind::For => write!(f, "FOR for null"),
            TokenKind::Fun => write!(f, "FUN fun null"),
            TokenKind::If => write!(f, "IF if null"),
            TokenKind::Nil => write!(f, "NIL nil null"),
            TokenKind::Or => write!(f, "OR or null"),
            TokenKind::Print => write!(f, "PRINT print null"),
            TokenKind::Return => write!(f, "RETURN return null"),
            TokenKind::Super => write!(f, "SUPER super null"),
            TokenKind::This => write!(f, "THIS this null"),
            TokenKind::True => write!(f, "TRUE true null"),
            TokenKind::Var => write!(f, "VAR var null"),
            TokenKind::While => write!(f, "WHILE while null"),
            TokenKind::Error(s) => write!(f, "ERROR {s}"),
            TokenKind::Eof => write!(f, "EOF  null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        assert_eq!(
            Token::identifier("andy", 1).to_string(),
            "IDENTIFIER andy null"
        );
        assert_eq!(
            Token::identifier("formless", 1).to_string(),
            "IDENTIFIER formless null"
        );
    }

    #[test]
    fn keywords() {
        assert_eq!(Token::new(TokenKind::And, 1).to_string(), "AND and null");
        assert_eq!(
            Token::new(TokenKind::Class, 1).to_string(),
            "CLASS class null"
        );
    }

    #[test]
    fn numbers() {
        let token = Token::new(TokenKind::Number("123".to_string()), 1);
        assert_eq!(token.to_string(), "NUMBER 123 123.0");
        let token = Token::new(TokenKind::Number("123.456".to_string()), 2);
        assert_eq!(token.to_string(), "NUMBER 123.456 123.456");
    }

    #[test]
    fn strings() {
        let token = Token::new(TokenKind::String("\"\"".to_string()), 1);
        assert_eq!(token.to_string(), "STRING \"\" ");
        let token = Token::new(TokenKind::String("\"string\"".to_string()), 1);
        assert_eq!(token.to_string(), "STRING \"string\" string");
    }
}
