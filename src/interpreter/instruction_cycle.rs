pub trait InstructionCycle {
    type InstructionType;

    /// It takes the next instruction from PC
    fn fetch(&self) -> u8;

    /// It decodes the instruction to understand the
    /// Instruction and the variables
    fn decode(&self, op_code: u8) -> Self::InstructionType;

    /// It executes the instruction and updates registers and memory
    fn execute(&mut self, op_code: Self::InstructionType);

    /// Abstraction of what happens for every instruction in the machine_state
    fn step(&mut self);
}
