use std::fmt::Display;
use std::iter::FusedIterator;

use crate::bytecode::core::{Chunk, Instruction, OpCode};

pub struct InstructionMetadata {
    pub line: u32,
    pub pos: usize,
}

impl InstructionMetadata {
    fn new(pos: usize, line: u32) -> InstructionMetadata {
        InstructionMetadata { pos, line }
    }
}

pub enum BytecodeParseError {
    UnknownInstruction(u8),
    InvalidConstantIndex(u8),
    UnexpectedEndOfBytecode(OpCode),
}

impl Display for BytecodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeParseError::UnknownInstruction(code) => {
                write!(f, "Unknown ({})", code)
            }
            BytecodeParseError::InvalidConstantIndex(index) => {
                write!(f, "{:?} (Invalid Index: {})", OpCode::Constant, index)
            }
            BytecodeParseError::UnexpectedEndOfBytecode(op) => {
                write!(
                    f,
                    "Unexpected End of Bytecode when parsing arguments for {:?}",
                    op
                )
            }
        }
    }
}

pub struct ChunkIterator<'a> {
    pub(crate) chunk: &'a Chunk,
    pub(crate) pos: usize,
}

impl<'a> ChunkIterator<'a> {
    fn read_constant(&mut self) -> Result<Instruction, BytecodeParseError> {
        self.pos += 1;
        if let Some(&constant_id) = self.chunk.code.get(self.pos - 1) {
            if let Some(&constant) = self.chunk.constants.get(constant_id as usize) {
                Ok(Instruction::Constant(constant))
            } else {
                Err(BytecodeParseError::InvalidConstantIndex(constant_id))
            }
        } else {
            Err(BytecodeParseError::UnexpectedEndOfBytecode(
                OpCode::Constant,
            ))
        }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (InstructionMetadata, Result<Instruction, BytecodeParseError>);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let code = self.chunk.code.get(self.pos);
        let line = self.chunk.lines.get(self.pos);
        self.pos += 1;

        // these should always be the same length, so the extra bounds checks can be removed (if not optimized away)
        if let Some((&code, &line)) = code.zip(line) {
            let metadata = InstructionMetadata::new(pos, line);
            let instruction = match OpCode::try_from(code) {
                Ok(op) => match op {
                    OpCode::Return => Ok(Instruction::Return),
                    OpCode::Constant => self.read_constant(),
                },
                Err(_) => Err(BytecodeParseError::UnknownInstruction(code)),
            };
            // read opcode
            Some((metadata, instruction))
        } else {
            None
        }
    }
}

impl FusedIterator for ChunkIterator<'_> {}

impl Chunk {
    pub fn iter(&self) -> ChunkIterator<'_> {
        ChunkIterator {
            chunk: self,
            pos: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = (InstructionMetadata, Result<Instruction, BytecodeParseError>);
    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIterator {
            chunk: &self,
            pos: 0,
        }
    }
}
