mod instruction_cycle;
mod isa;
pub mod program;
mod state;

pub use instruction_cycle::InstructionCycle;
use isa::{Instruction, Isa};
pub use program::Program;
use state::MachineState;

pub struct Interpreter {
    program: Program,
    state: MachineState,
}

impl InstructionCycle for Interpreter {
    type InstructionType = Instruction;

    fn fetch(&self) -> u8 {
        self.program.raw[self.state.registers.ip as usize]
    }

    fn decode(&self, op_code: u8) -> Self::InstructionType {
        Instruction::from(op_code)
    }

    fn execute(&mut self, op_code: Self::InstructionType) {
        use Instruction::*;
        match op_code {
            HALT => self.halt(),
            CLRA => self.clra(),
            INC3A => self.inc3a(),
            DECA => self.deca(),
            SETL => self.setl(),
            BACK7 => self.back7(),
        };

        self.state.registers.ip += 1;
    }

    fn step(&mut self) {
        let op_code = self.fetch();
        let op_code = self.decode(op_code);

        self.execute(op_code)
    }
}

impl Isa for Interpreter {
    fn halt(&mut self) {
        self.state.halted = true;
    }

    fn clra(&mut self) {
        self.state.registers.a = 0;
    }

    fn inc3a(&mut self) {
        self.state.registers.a = self.state.registers.a.wrapping_add(3);
    }

    fn deca(&mut self) {
        self.state.registers.a = self.state.registers.a.wrapping_sub(1);
    }

    fn setl(&mut self) {
        self.state.registers.l = self.state.registers.a;
    }

    fn back7(&mut self) {
        self.state.registers.l = self.state.registers.l.wrapping_sub(1);
        if self.state.registers.l >= 0 {
            self.state.registers.ip -= 8;
        }
    }
}

impl Interpreter {
    pub fn new(program: Program, a: i32, l: i32) -> Self {
        Self {
            program,
            state: MachineState::new(a, l),
        }
    }

    pub fn halted(&self) -> bool {
        self.state.halted
    }
}
