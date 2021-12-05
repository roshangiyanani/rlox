use std::fmt::Display;
use std::iter::FusedIterator;

use crate::bytecode::core::{Chunk, Instruction, OpCode};

use super::core::BinaryOp;

pub struct InstructionMetadata {
    pub line: u32,
    pub pos: usize,
}

impl InstructionMetadata {
    fn new(pos: usize, line: u32) -> InstructionMetadata {
        InstructionMetadata { pos, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub struct BytecodeParser<'a> {
    pub(crate) chunk: &'a Chunk,
    pub(crate) pos: usize,
}

impl<'a> BytecodeParser<'a> {
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

impl<'a> Iterator for BytecodeParser<'a> {
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
                    OpCode::Negate => Ok(Instruction::Negate),
                    OpCode::Add => Ok(Instruction::BinaryOp(BinaryOp::Add)),
                    OpCode::Subtract => Ok(Instruction::BinaryOp(BinaryOp::Subtract)),
                    OpCode::Multiply => Ok(Instruction::BinaryOp(BinaryOp::Multiply)),
                    OpCode::Divide => Ok(Instruction::BinaryOp(BinaryOp::Divide)),
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

impl FusedIterator for BytecodeParser<'_> {}

impl Chunk {
    pub fn iter(&self) -> BytecodeParser<'_> {
        BytecodeParser {
            chunk: self,
            pos: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = (InstructionMetadata, Result<Instruction, BytecodeParseError>);
    type IntoIter = BytecodeParser<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BytecodeParser {
            chunk: &self,
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn normal() {
        let mut chunk = Chunk::new();

        let instructions = vec![
            (1, Instruction::Constant(Value(3.0))),
            (2, Instruction::Return),
            (3, Instruction::Constant(Value(2.0))),
            (3, Instruction::Return),
        ];
        chunk.add_instructions(&instructions);

        let parsed: Vec<_> = chunk
            .iter()
            .map(|(metadata, parsed)| {
                (
                    metadata.line,
                    parsed.expect("unable to parse test instruction stream"),
                )
            })
            .collect();

        assert_eq!(instructions, parsed);
    }

    #[test]
    fn missing_constant() {
        let chunk = Chunk {
            code: vec![
                OpCode::Return.into(),
                OpCode::Constant.into(),
                0,
                OpCode::Return.into(),
            ],
            lines: vec![1, 2, 2, 3, 3],
            constants: vec![],
        };

        let result: Vec<_> = chunk.iter().map(|(_, parsed)| parsed).collect();

        assert_eq!(
            result,
            vec![
                Ok(Instruction::Return),
                Err(BytecodeParseError::InvalidConstantIndex(0)),
                Ok(Instruction::Return),
            ],
        )
    }

    #[test]
    fn early_eof() {
        let chunk = Chunk {
            code: vec![
                OpCode::Return.into(),
                OpCode::Constant.into(),
                // no constant index
            ],
            lines: vec![1, 1, 1],
            constants: vec![],
        };

        let result: Vec<_> = chunk.iter().map(|(_, parsed)| parsed).collect();

        assert_eq!(
            result,
            vec![
                Ok(Instruction::Return),
                Err(BytecodeParseError::UnexpectedEndOfBytecode(
                    OpCode::Constant
                )),
            ]
        )
    }
}
