use crate::bytecode::Chunk;

impl Chunk {
    pub fn dissemble(&self, name: &str) {
        println!("== {} ==", name);
        for (index, op) in self.iter().enumerate() {
            print!("{:04} ", index);
            match op {
                Ok(op) => println!("{:?}", op),
                Err(e) => println!("{}", e.kind),
            }
        }
    }
}
