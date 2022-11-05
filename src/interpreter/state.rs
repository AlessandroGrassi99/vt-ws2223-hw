pub struct Registers {
    pub ip: u32,
    pub a: i32,
    pub l: i32,
}

pub struct MachineState {
    pub registers: Registers,
    pub halted: bool,
}

impl Registers {
    pub fn new(a: i32, l: i32) -> Self {
        Self { ip: 0, a, l }
    }
}

impl MachineState {
    pub fn new(a: i32, l: i32) -> Self {
        Self {
            registers: Registers::new(a, l),
            halted: false,
        }
    }
}
