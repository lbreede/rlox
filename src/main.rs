use std::io::{self, Write};
use std::process::{self, ExitCode};
use std::{env, fs};

use rlox::vm::{InterpretResult, VM};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            return ExitCode::from(64);
        }
    }

    ExitCode::SUCCESS
}

fn repl() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line).expect("failed to read line");

        // EOF (Ctrl+D)
        if bytes_read == 0 {
            println!();
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        VM::interpret(&line);
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read file {path}: {e}");
            process::exit(74);
        }
    };

    match VM::interpret(&source) {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => {}
    }
}
