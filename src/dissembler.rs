use crate::bytecode::core::Chunk;

impl Chunk {
    pub fn dissemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut prev_line: Option<u32> = None;
        for (metadata, parsed) in self.iter() {
            print!("{:04} ", metadata.pos);

            if prev_line == Some(metadata.line) {
                print!("   | ")
            } else {
                prev_line = Some(metadata.line);
                print!("{:4} ", metadata.line);
            }

            match parsed {
                Ok(instruction) => println!("{:?}", instruction),
                Err(e) => println!("{}", e),
            }
        }
    }
}
