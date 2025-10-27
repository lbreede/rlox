use crate::{opcode::OpCode, value::Value};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<(u8, usize)>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push((byte, line));
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        let (byte, line) = self.code[offset];
        if offset > 0 && line == self.code[offset - 1].1 {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }

        if let Ok(instruction) = OpCode::try_from(byte) {
            match instruction {
                OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
                OpCode::Add => Self::simple_instruction("OP_ADD", offset),
                OpCode::Subtract => Self::simple_instruction("OP_SUBTRACT", offset),
                OpCode::Multiply => Self::simple_instruction("OP_MULTIPLY", offset),
                OpCode::Divide => Self::simple_instruction("OP_DIVIDE", offset),
                OpCode::Negate => Self::simple_instruction("OP_NEGATE", offset),
                OpCode::Return => Self::simple_instruction("OP_RETURN", offset),
            }
        } else {
            println!("Unknown opcode: {}", byte);
            offset + 1
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1].0;
        println!(
            "{:<16} {:4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
