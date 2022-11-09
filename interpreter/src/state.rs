pub struct Registers {
    pub ip: u32,
    pub a: i32,
    pub l: i32,
}

pub struct MachineState {
    pub registers: Registers,
    pub halted:  bool,
}

impl Registers {
    pub fn new(a: i32, l: i32) -> Self {
        Self { ip: 0, a, l }
    }

    pub fn reset(&mut self, a: i32, l: i32) {
        self.ip = 0;
        self.a = a;
        self.l = l;
    }
}

impl MachineState {
    pub fn new(a: i32, l: i32) -> Self {
        Self {
            registers: Registers::new(a, l),
            halted: false,
        }
    }

    pub fn reset(&mut self, a: i32, l: i32) {
        self.registers.reset(a, l);
        self.halted = false;
    }
}
