use rlox::chunk::{Chunk, OpCode};
use rlox::debug;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::Return.into(), 123);

    debug::disassemble_chunk(&chunk, "test chunk");
}
