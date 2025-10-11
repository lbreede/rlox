use crate::value::Value;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<(u8, u16)>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u16) {
        self.code.push((byte, line));
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
