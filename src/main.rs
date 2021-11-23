mod bytecode;

use bytecode::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.append_op(OpCode::Return);

    chunk.dissemble("test chunk");
    std::process::exit(0);
}
