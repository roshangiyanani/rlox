mod bytecode;
mod dissembler;
mod value;

use bytecode::{Chunk, Instruction};
use value::Value;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_instruction(Instruction::Return, 1);
    chunk.add_instruction(Instruction::Constant(Value(3.0)), 2);

    chunk.dissemble("test chunk");
    std::process::exit(0);
}
