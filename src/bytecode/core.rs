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

    pub(crate) fn add_instruction(&mut self, instruction: Instruction, line: u32) {
        match instruction {
            Instruction::Return => self.add_op(OpCode::Return, line),
            Instruction::Constant(value) => {
                let constant_id = self.add_constant(value);
                self.add_op(OpCode::Constant, line);
                self.add_raw(constant_id, line);
            }
        }
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
