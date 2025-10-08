type Value = f64;

#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::Constant as u8 => Ok(OpCode::Constant),
            x if x == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

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
