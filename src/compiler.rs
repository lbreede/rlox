use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::scanner::{ScanError, Scanner};
use crate::token::{Token, TokenKind};
use crate::value::Value;

#[derive(Debug)]
pub enum CompileError {
    Scan(ScanError),
    Parse(String),
    Unknown,
}

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

struct Parser {
    scanner: Scanner,
    current: Result<Token, ScanError>,
    previous: Result<Token, ScanError>,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    fn new(source: &str) -> Self {
        let mut scanner = Scanner::new(source);
        let current = scanner.scan_token();
        Self {
            scanner,
            current,
            previous: Ok(Token::new(TokenKind::Eof, 0)),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current = self.scanner.scan_token();
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        match &self.current {
            Ok(token) if token.kind == kind => self.advance(),
            Ok(_) | Err(_) => self.error_at_current(message),
        }
    }

    fn error_at(&mut self, token: &Result<Token, ScanError>, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        match token {
            Ok(token) => eprint!("[line {}] Error '{}'", token.line, token.lexeme()),
            Err(err) => eprint!("[scanner error]: {:?}", err),
        }
        eprintln!(": {message}");
        self.had_error = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message);
    }
}

pub struct Compiler {
    parser: Parser,
    chunk: Chunk,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Self {
            parser: Parser::new(source),
            chunk: Chunk::new(),
        }
    }
    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn emit_byte(&mut self, byte: u8) -> Result<(), CompileError> {
        let token = match self.parser.previous.clone() {
            Ok(t) => t,
            Err(e) => return Err(CompileError::Scan(e)),
        };
        let line = token.line;
        self.current_chunk().write(byte, line);
        Ok(())
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) -> Result<(), CompileError> {
        self.emit_byte(byte1)?;
        self.emit_byte(byte2)?;
        Ok(())
    }

    fn emit_return(&mut self) -> Result<(), CompileError> {
        self.emit_byte(OpCode::Return.into())?;
        Ok(())
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > u8::MAX.into() {
            self.parser.error("Too many constants in this chunk.");
            return 0;
        }
        constant as u8
    }

    fn emit_constant(&mut self, value: Value) -> Result<(), CompileError> {
        let byte2 = self.make_constant(value);
        self.emit_bytes(OpCode::Constant.into(), byte2)?;
        Ok(())
    }

    fn end_compiler(&mut self) -> Result<(), CompileError> {
        self.emit_return()?;
        #[cfg(debug_assertions)]
        {
            if !self.parser.had_error {
                self.chunk.disassemble("code");
            }
        }
        Ok(())
    }

    fn advance(&mut self) {
        self.parser.advance();
    }

    fn grouping(&mut self) -> Result<(), CompileError> {
        self.expression()?;
        self.parser
            .consume(TokenKind::RightParen, "Expect '(' after expression.");
        Ok(())
    }

    fn unary(&mut self) -> Result<(), CompileError> {
        let token = match self.parser.previous.clone() {
            Ok(t) => t,
            Err(e) => return Err(CompileError::Scan(e)),
        };
        let operator_kind = token.kind;
        self.parse_precedence(Prec::Unary)?;
        match operator_kind {
            TokenKind::Minus => self.emit_byte(OpCode::Negate.into())?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn binary(&mut self) -> Result<(), CompileError> {
        let operator_kind = self.parser.previous.clone().unwrap().kind;
        let rule_prec = get_precedence(&operator_kind);
        self.parse_precedence(next_prec(&rule_prec))?;

        match &operator_kind {
            TokenKind::Plus => self.emit_byte(OpCode::Add.into())?,
            TokenKind::Minus => self.emit_byte(OpCode::Subtract.into())?,
            TokenKind::Star => self.emit_byte(OpCode::Multiply.into())?,
            TokenKind::Slash => self.emit_byte(OpCode::Divide.into())?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Prec) -> Result<(), CompileError> {
        match self.parser.current.clone().unwrap().kind {
            TokenKind::Number(s) => {
                self.advance();
                self.emit_constant(s.parse().expect("failed to parse '{s}'"))?;
            }
            TokenKind::LeftParen => {
                self.advance();
                self.grouping()?;
            }
            TokenKind::Minus => {
                self.advance();
                self.unary()?;
            }
            _ => {
                // self.parser.error_at_current("Expect expression.");
                return Err(CompileError::Parse("Expect expression".to_owned()));
            }
        }

        while precedence <= get_precedence(&self.parser.current.clone().unwrap().kind) {
            self.advance();
            match self.parser.previous.clone().unwrap().kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    self.binary()?;
                }
                _ => return Err(CompileError::Unknown),
            }
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<(), CompileError> {
        self.parse_precedence(Prec::Assignment)?;
        Ok(())
    }

    pub fn compile(&mut self) -> Result<Chunk, CompileError> {
        self.expression()?;
        self.parser
            .consume(TokenKind::Eof, "Expect end of expression.");
        self.end_compiler()?;
        if self.parser.had_error {
            Err(CompileError::Unknown)
        } else {
            Ok(self.chunk.clone())
        }
    }
}

fn get_precedence(kind: &TokenKind) -> Prec {
    match kind {
        TokenKind::Plus | TokenKind::Minus => Prec::Term,
        TokenKind::Star | TokenKind::Slash => Prec::Factor,
        _ => Prec::None,
    }
}

fn next_prec(prec: &Prec) -> Prec {
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
