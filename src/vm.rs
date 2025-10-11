use crate::debug::disassemble_instruction;
use crate::value::{Value, print_value};
use crate::{chunk::Chunk, opcode::OpCode};
const STACK_MAX: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    ip: u8,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    fn reset_stack(&mut self) {
        self.stack.clear()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.reset_stack();
        self.run(chunk)
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip as usize].0;
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = self.read_byte(chunk) as usize;
        chunk.constants[index]
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            #[cfg(debug_assertions)]
            {
                print!("          ");
                for val in &self.stack {
                    print!("[ ");
                    print_value(val);
                    print!(" ]");
                }
                println!();
                disassemble_instruction(chunk, self.ip.into());
            }
            let instruction = self.read_byte(chunk);
            let opcode = OpCode::try_from(instruction).expect("Invalid opcode");

            match opcode {
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.push(constant);
                }
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
                OpCode::Negate => {
                    let v = self.pop();
                    self.push(-v);
                }
                OpCode::Return => {
                    print_value(&self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
            }
        }
    }
}
