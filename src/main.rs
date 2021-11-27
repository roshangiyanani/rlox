mod bytecode;
mod dissembler;
mod value;

use bytecode::{Chunk, Instruction};
use value::Value;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_instruction(Instruction::Constant(Value(3.0)), 1);
    chunk.add_instruction(Instruction::Return, 2);
    chunk.add_instruction(Instruction::Constant(Value(2.0)), 3);
    chunk.add_instruction(Instruction::Return, 3);

    chunk.dissemble("test chunk");

    let chunk = Chunk {
        code: vec![1, 2, 0],
        lines: vec![1, 1, 1],
        constants: vec![],
    };

    chunk.dissemble("missing constant");

    let chunk = Chunk {
        code: vec![1, 2],
        lines: vec![1, 1, 1],
        constants: vec![],
    };

    chunk.dissemble("early eof");

    std::process::exit(0);
}
