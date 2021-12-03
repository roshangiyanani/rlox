mod bytecode;
mod dissembler;
mod value;

use bytecode::core::{Chunk, OpCode};
use value::Value;

fn main() {
    let chunk = Chunk {
        code: vec![
            OpCode::Return.into(),
            OpCode::Constant.into(),
            0,
            OpCode::Return.into(),
            OpCode::Constant.into(),
            1,
            OpCode::Return.into(),
        ],
        lines: vec![1, 2, 2, 2, 3, 3, 3],
        constants: vec![Value(3.0)],
    };

    chunk.dissemble("missing constant");

    std::process::exit(0);
}
