use std::fmt::{Binary, Display};

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::value::Value;

/// Bytecode instructions.
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    /// Return from the current function.
    Return = 1,
    /// Load a constant, whose index is the next byte.
    Constant,
    /// Negate the value at the top of the stack.
    Negate,
    /// Binary Op, applied to the two values at the top of the stack.
    Add,
    /// Binary Op, applied to the two values at the top of the stack.
    Subtract,
    /// Binary Op, applied to the two values at the top of the stack.
    Multiply,
    /// Binary Op, applied to the two values at the top of the stack.
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOp::Add => '+',
            BinaryOp::Subtract => '-',
            BinaryOp::Multiply => '*',
            BinaryOp::Divide => '/',
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Return,
    Constant(Value),
    Negate,
    BinaryOp(BinaryOp),
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
            Instruction::Negate => self.add_op(OpCode::Negate, line),
            Instruction::BinaryOp(BinaryOp::Add) => self.add_op(OpCode::Add, line),
            Instruction::BinaryOp(BinaryOp::Subtract) => self.add_op(OpCode::Subtract, line),
            Instruction::BinaryOp(BinaryOp::Multiply) => self.add_op(OpCode::Multiply, line),
            Instruction::BinaryOp(BinaryOp::Divide) => self.add_op(OpCode::Divide, line),
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
    use super::*;
    use crate::value::Value;

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
