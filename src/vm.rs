use crate::{
    bytecode::{
        core::{Chunk, Instruction},
        parser::BytecodeParseError,
    },
    dissembler::DissemblerPrinter,
};

pub struct VM {
    pub chunk: Chunk,
}

impl VM {
    pub fn interpret(&self, debug: bool) -> Result<(), BytecodeParseError> {
        let mut dissembler = debug.then(|| DissemblerPrinter::new());

        for (metadata, parsed) in self.chunk.iter() {
            if let Some(dissembler) = dissembler.as_mut() {
                dissembler.print(metadata, parsed);
            }

            let instruction = parsed?;
            match instruction {
                Instruction::Constant(value) => println!("{}", value.0),
                Instruction::Return => break,
            }
        }

        Ok(())
    }
}
