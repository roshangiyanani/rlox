use crate::bytecode::{
    core::{Chunk, Instruction},
    parser::BytecodeParseError,
};

pub struct VM {
    pub chunk: Chunk,
}

impl VM {
    pub fn interpret(&self) -> Result<(), BytecodeParseError> {
        for (metadata, parsed) in self.chunk.iter() {
            match parsed {
                Ok(instruction) => {
                    log::debug!("{:04} {:4} {:?}", metadata.pos, metadata.line, instruction)
                }
                Err(e) => {
                    log::debug!("{:04} {:4} {:?}", metadata.pos, metadata.line, e)
                }
            };

            let instruction = parsed?;
            match instruction {
                Instruction::Constant(value) => println!("{}", value.0),
                Instruction::Return => break,
            }
        }

        Ok(())
    }
}
