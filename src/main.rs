use rlox::chunk::Chunk;
use rlox::debug;
use rlox::opcode::OpCode;
use rlox::vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::Add.into(), 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::Divide.into(), 123);
    chunk.write(OpCode::Negate.into(), 123);

    chunk.write(OpCode::Return.into(), 123);

    debug::disassemble_chunk(&chunk, "test chunk");
    vm.interpret(&chunk);
}
