use crate::scanner::Scanner;
use crate::token::TokenKind;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut last_line = 0;

    loop {
        let token = scanner.scan_token();

        if token.line != last_line {
            print!("{:4} ", token.line);
            last_line = token.line;
        } else {
            print!("   | ");
        }

        println!(
            "{:2?} '{}'",
            token.kind as u8, // u8 so that it match C-version
            String::from_utf8_lossy(token.lexeme)
        );

        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
