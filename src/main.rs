use std::io::{self, Write};
use std::process;
use std::{env, fs};

use rlox::vm::{InterpretResult, VM};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}

fn repl() {
    // let mut vm = VM::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().expect("failed to flush stdout");

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line).expect("failed to read line");

        // EOF (Ctrl+D)
        if bytes_read == 0 {
            println!();
            break;
        }

        VM::interpret(&line);
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path, e);
            process::exit(74); // similar to C's read failure exit
        }
    };

    let result = VM::interpret(&source);

    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => {}
    }
}
