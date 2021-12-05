mod bytecode;
mod dissembler;
mod value;
mod vm;

use bytecode::core::{Chunk, Instruction};
use dissembler::DissemblerPrinter;
use log::Level;
use value::Value;

use crate::vm::VM;

fn main() {
    simple_logger::init_with_level(Level::Trace).expect("cannot initialize logger");

    let mut chunk = Chunk::new();

    chunk.add_instructions(&[
        (1, Instruction::Constant(Value(1.2))),
        (2, Instruction::Constant(Value(3.1415))),
        (2, Instruction::Return),
        (3, Instruction::Constant(Value(f64::NAN))),
    ]);
    DissemblerPrinter::dissemble(&chunk, "chunk");

    let vm = VM { chunk };
    vm.interpret().expect("unable to interpret");

    std::process::exit(0);
}
