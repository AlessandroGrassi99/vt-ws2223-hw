#[derive(Debug)]
pub enum Instruction {
    HALT = 0x0,  // Stop execution
    CLRA = 0x1,  // Set content of register A to 0
    INC3A = 0x2, // Increment register A by 3
    DECA = 0x3,  // Decrement register A by 1
    SETL = 0x4,  // Copy value of register A to L
    BACK7 = 0x5, // Decrement L; if value of L is positive, jump back by 7 instructions (i.e. loop
                 // body is 6 one-byte instructions and the BACK7 itself). Otherwise fall through to next instruction
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::HALT,
            0x1 => Self::CLRA,
            0x2 => Self::INC3A,
            0x3 => Self::DECA,
            0x4 => Self::SETL,
            0x5 => Self::BACK7,
            _ => unreachable!("invalid instruction"),
        }
    }
}

pub trait Isa {
    fn halt(&mut self);
    fn clra(&mut self);
    fn inc3a(&mut self);
    fn deca(&mut self);
    fn setl(&mut self);
    fn back7(&mut self);
}
