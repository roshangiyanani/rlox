use crate::bytecode::{
    core::{Chunk, Instruction},
    parser::{BytecodeParseError, InstructionMetadata},
};

#[derive(Debug, Clone, Copy)]
pub struct DissemblerPrinter {
    prev_line: Option<u32>,
}

impl DissemblerPrinter {
    pub fn new() -> DissemblerPrinter {
        DissemblerPrinter { prev_line: None }
    }

    pub fn dissemble(chunk: &Chunk, name: &str) {
        println!("== {} ==", name);
        let mut dissembler = DissemblerPrinter::new();

        for (metadata, parsed) in chunk.iter() {
            dissembler.print(metadata, parsed)
        }
        println!()
    }

    pub fn print(
        &mut self,
        metadata: InstructionMetadata,
        parsed: Result<Instruction, BytecodeParseError>,
    ) {
        print!("{:04} ", metadata.pos);

        if self.prev_line == Some(metadata.line) {
            print!("   | ")
        } else {
            self.prev_line = Some(metadata.line);
            print!("{:4} ", metadata.line);
        }

        match parsed {
            Ok(instruction) => println!("{:?}", instruction),
            Err(e) => println!("{}", e),
        }
    }
}
