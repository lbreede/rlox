use crate::value::Value;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<(u8, usize)>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push((byte, line));
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        (self.constants.len() - 1) as usize
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
