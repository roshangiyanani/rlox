use std::fmt::Display;
use std::iter::FusedIterator;

use crate::bytecode::core::{Chunk, Instruction, OpCode};

pub struct BytecodeParseError {
    pub line: Option<u32>,
    pub kind: BytecodeParseErrorKind,
}

impl BytecodeParseError {
    pub(crate) fn new(line: Option<u32>, kind: BytecodeParseErrorKind) -> BytecodeParseError {
        BytecodeParseError { line, kind }
    }
}

pub enum BytecodeParseErrorKind {
    UnknownInstruction(u8),
    InvalidConstantIndex(u8),
    UnexpectedEndOfBytecode(OpCode),
}

impl Display for BytecodeParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeParseErrorKind::UnknownInstruction(code) => {
                write!(f, "Unknown ({})", code)
            }
            BytecodeParseErrorKind::InvalidConstantIndex(index) => {
                write!(f, "{:?} (Invalid Index: {})", OpCode::Constant, index)
            }
            BytecodeParseErrorKind::UnexpectedEndOfBytecode(op) => {
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
    pub(crate) fn read_constant(&mut self) -> Result<Instruction, BytecodeParseError> {
        self.pos += 1;
        if let Some(&constant_id) = self.chunk.code.get(self.pos - 1) {
            if let Some(&constant) = self.chunk.constants.get(constant_id as usize) {
                Ok(Instruction::Constant(constant))
            } else {
                Err(BytecodeParseError::new(
                    self.chunk.lines.get(self.pos - 1).copied(),
                    BytecodeParseErrorKind::InvalidConstantIndex(constant_id),
                ))
            }
        } else {
            Err(BytecodeParseError::new(
                self.chunk.lines.get(self.pos - 1).copied(),
                BytecodeParseErrorKind::UnexpectedEndOfBytecode(OpCode::Constant),
            ))
        }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Result<Instruction, BytecodeParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if let Some(&code) = self.chunk.code.get(self.pos - 1) {
            // read opcode
            Some(match OpCode::try_from(code) {
                Ok(op) => match op {
                    OpCode::Return => Ok(Instruction::Return),
                    OpCode::Constant => self.read_constant(),
                },
                Err(_) => Err(BytecodeParseError::new(
                    self.chunk.lines.get(self.pos - 1).copied(),
                    BytecodeParseErrorKind::UnknownInstruction(code),
                )),
            })
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
    type Item = Result<Instruction, BytecodeParseError>;
    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIterator {
            chunk: &self,
            pos: 0,
        }
    }
}
