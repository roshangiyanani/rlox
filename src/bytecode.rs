use std::iter::FusedIterator;

/// Bytecode instructions.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// Return from the current function.
    Return = 1
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            1 => Ok(OpCode::Return),
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
    code: Vec<u8>,
}

impl Chunk {
    pub(crate) fn new() -> Chunk {
        Chunk {
            code: Vec::new()
        }
    }

    fn iter(&self) -> ChunkIterator<'_> {
        ChunkIterator {
            chunk: self,
            pos: 0,
        }
    }

    pub(crate) fn append_op(&mut self, op: OpCode) {
        self.code.push(op.into());
    }

    pub(crate) fn dissemble(&self, name: &str) {
        println!("== {} ==", name);
        for (index, op) in self.iter().enumerate() {
            print!("{:04} ", index);
            match op {
                Ok(op) => println!("{:#?}", op),
                Err(code) => println!("Unknown ({})", code)
            }
        }
    }
}

pub struct ChunkIterator<'a> {
    chunk: &'a Chunk,
    pos: usize,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Result<OpCode, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.chunk.code.len() {
            None
        }
        else {
            let code = self.chunk.code[self.pos];
            self.pos += 1;

            Some(OpCode::try_from(code).map_err(|_| code))
        }
    }
}

impl FusedIterator for ChunkIterator<'_> {}

impl<'a> IntoIterator for &'a Chunk {
    type Item = Result<OpCode, u8>;
    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIterator {
            chunk: &self,
            pos: 0,
        }
    }
}

