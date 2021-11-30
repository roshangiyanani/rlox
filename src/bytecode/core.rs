use crate::value::Value;

/// Bytecode instructions.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// Return from the current function.
    Return = 1,
    Constant = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Return,
    Constant(Value),
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            1 => Ok(OpCode::Return),
            2 => Ok(OpCode::Constant),
            _ => Err(()),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub lines: Vec<u32>,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub(crate) fn new() -> Chunk {
        Chunk {
            lines: Vec::new(),
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub(crate) fn add_instruction(&mut self, line: u32, instruction: Instruction) {
        match instruction {
            Instruction::Return => self.add_op(OpCode::Return, line),
            Instruction::Constant(value) => {
                let constant_id = self.add_constant(value);
                self.add_op(OpCode::Constant, line);
                self.add_raw(constant_id, line);
            }
        }
    }

    pub(crate) fn add_instructions(&mut self, instructions: &[(u32, Instruction)]) {
        instructions
            .iter()
            .for_each(|(line, instruction)| self.add_instruction(*line, *instruction));
    }

    fn add_op(&mut self, op: OpCode, line: u32) {
        self.add_raw(op.into(), line);
    }

    fn add_raw(&mut self, code: u8, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    fn add_constant(&mut self, constant: Value) -> u8 {
        let index = self.constants.len();
        self.constants.push(constant);
        u8::try_from(index).expect("too many constants")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bytecode::{core::OpCode, Instruction},
        value::Value,
    };

    use super::Chunk;

    #[test]
    fn add_return() {
        let mut chunk = Chunk::new();
        chunk.add_instruction(1, Instruction::Return);

        let expected = Chunk {
            code: vec![OpCode::Return.into()],
            constants: vec![],
            lines: vec![1],
        };
        assert_eq!(chunk, expected);
    }

    #[test]
    fn add_constant() {
        let mut chunk = Chunk::new();
        chunk.add_instructions(&[
            (1, Instruction::Constant(Value(3.0))),
            (2, Instruction::Constant(Value(1.0))),
        ]);

        let expected = Chunk {
            code: vec![
                OpCode::Constant.into(),
                0, // constant index
                OpCode::Constant.into(),
                1,
            ],
            constants: vec![Value(3.0), Value(1.0)],
            lines: vec![1, 1, 2, 2],
        };
        assert_eq!(chunk, expected);
    }
}
