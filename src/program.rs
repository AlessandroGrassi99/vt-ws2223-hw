pub struct Program {
    pub raw: [u8; 50000],
    pub size: i32,
}

impl Program {
    pub fn new(raw: [u8; 50000], size: i32) -> Self {
        Self { raw, size }
    }
}
