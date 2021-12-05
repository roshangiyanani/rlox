use crate::{
    bytecode::{
        core::{Chunk, Instruction},
        parser::BytecodeParseError,
    },
    value::Value,
};

pub struct VM {
    pub stack: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpreterError {
    ParseError(BytecodeParseError),
    EmptyStack,
}

impl From<BytecodeParseError> for InterpreterError {
    fn from(e: BytecodeParseError) -> Self {
        InterpreterError::ParseError(e)
    }
}

enum ControlFlow {
    Continue,
    Break,
}

impl VM {
    pub fn new() -> VM {
        VM { stack: Vec::new() }
    }

    fn stack_pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop().ok_or(InterpreterError::EmptyStack)
    }

    fn interpret(&mut self, instruction: Instruction) -> Result<ControlFlow, InterpreterError> {
        match instruction {
            Instruction::Return => {
                println!("{:?}", self.stack_pop()?);
                Ok(ControlFlow::Break)
            }
            Instruction::Constant(value) => {
                self.stack.push(value);
                Ok(ControlFlow::Continue)
            }
            Instruction::Negate => {
                let value = self.stack_pop()?;
                self.stack.push(-value);
                Ok(ControlFlow::Continue)
            }
        }
    }

    pub fn interpret_chunk(chunk: Chunk) -> Result<(), InterpreterError> {
        let mut vm = VM::new();

        for (metadata, parsed) in chunk.iter() {
            log::trace!("stack: {:?}", vm.stack);
            match parsed {
                Ok(instruction) => {
                    log::debug!("{:04} {:4} {:?}", metadata.pos, metadata.line, instruction)
                }
                Err(e) => {
                    log::debug!("{:04} {:4} {:?}", metadata.pos, metadata.line, e)
                }
            };

            let instruction = parsed?;
            match vm.interpret(instruction)? {
                ControlFlow::Break => break,
                ControlFlow::Continue => (),
            }
        }

        Ok(())
    }
}
