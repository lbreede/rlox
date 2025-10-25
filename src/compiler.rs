use std::iter::Peekable;

use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::scanner::Scanner;
use crate::token::{Token, TokenKind};
use crate::value::Value;

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
enum Prec {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut scanner = Scanner::new(source);
        let current = scanner.next();
        Self {
            scanner,
            current,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.take();
        self.current = self.scanner.next();
    }
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    pub chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            parser: Parser::new(source),
            chunk: Chunk::new(),
        }
    }
    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn error_at(&mut self, token: Option<Token>, message: &'static str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        match token {
            Some(t) => {
                eprint!("[line {}] Error", t.line);
                match t.kind {
                    TokenKind::Error => (),
                    _ => {
                        let lexeme = std::str::from_utf8(t.lexeme).unwrap();
                        eprint!(" '{}'", lexeme);
                    }
                }
            }
            None => {
                eprint!("Error at end")
            }
        }
        eprintln!(": {message}");
        self.parser.had_error = true;
    }

    fn error(&mut self, message: &'static str) {
        self.error_at(self.parser.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: &'static str) {
        self.error_at(self.parser.current.clone(), message);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.clone().unwrap().line;
        self.current_chunk().write(byte, line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return.into());
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > u8::MAX.into() {
            self.error("Too many constants in this chunk.");
            return 0;
        }
        constant as u8
    }

    fn emit_constant(&mut self, value: Value) {
        let byte2 = self.make_constant(value);
        self.emit_bytes(OpCode::Constant.into(), byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn advance(&mut self) {
        self.parser.advance();
    }

    fn binary(&mut self) {
        let operator_kind = self.parser.previous.clone().map(|t| t.kind);
        let rule_prec = get_precedence(operator_kind);
        self.parse_precedence(next_prec(rule_prec));

        match operator_kind {
            Some(TokenKind::Plus) => self.emit_byte(OpCode::Add.into()),
            Some(TokenKind::Minus) => self.emit_byte(OpCode::Subtract.into()),
            Some(TokenKind::Star) => self.emit_byte(OpCode::Multiply.into()),
            Some(TokenKind::Slash) => self.emit_byte(OpCode::Divide.into()),
            _ => todo!(),
        }
    }

    fn parse_precedence(&mut self, precedence: Prec) {
        match self.parser.current.clone().unwrap().kind {
            TokenKind::Number(value) => {
                self.advance();
                self.emit_constant(value);
            }
            _ => {
                self.error_at_current("Expect expression.");
                return;
            }
        }

        while precedence <= get_precedence(self.parser.current.clone().map(|t| t.kind)) {
            self.advance();
            match self.parser.previous.clone().map(|t| t.kind) {
                Some(TokenKind::Plus)
                | Some(TokenKind::Minus)
                | Some(TokenKind::Star)
                | Some(TokenKind::Slash) => {
                    self.binary();
                }
                _ => return,
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Prec::Assignment);
    }

    pub fn compile(&mut self) -> bool {
        self.expression();
        self.end_compiler();
        return !self.parser.had_error;
    }
}

fn get_precedence(kind: Option<TokenKind>) -> Prec {
    match kind {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => Prec::Term,
        Some(TokenKind::Star) | Some(TokenKind::Slash) => Prec::Factor,
        _ => Prec::None,
    }
}

fn next_prec(prec: Prec) -> Prec {
    use Prec::*;
    match prec {
        None => Assignment,
        Assignment => Or,
        Or => And,
        And => Equality,
        Equality => Comparison,
        Comparison => Term,
        Term => Factor,
        Factor => Unary,
        Unary => Call,
        Call => Primary,
        Primary => Primary,
    }
}
