use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:04} ");
    let (byte, line) = chunk.code[offset];
    if offset > 0 && line == chunk.code[offset - 1].1 {
        print!("   | ");
    } else {
        print!("{:4} ", line);
    }

    if let Ok(instruction) = OpCode::try_from(byte) {
        match instruction {
            OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
        }
    } else {
        println!("Unknown opcode: {}", byte);
        offset + 1
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1].0;
    println!(
        "{:<16} {:4} '{}'",
        name, constant, chunk.constants[constant as usize]
    );
    offset + 2
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}
