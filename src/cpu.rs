use crate::{inst::Instruction, reg::Registers};

/// Represents the RISC-V CPU.
#[derive(Debug, Clone, Default)]
pub struct Cpu {
    /// A small amoumt of fast, general purpouse registers.
    /// Each register has a role defined by the integer register convention.
    registers: Registers,
    /// The program counter. Holds the address of the current instruction.
    pc: u64,
    // FIXME: Move this to a memory bus.
    ram: Vec<u8>,
}

impl Cpu {
    /// Starts the CPU cycle loop. It will infinitely run
    /// the 'fetch, decode, execute' cycle until
    /// the user stops the emulator explicitly,
    /// or an unrecoverable error is encountered.
    pub fn run(mut self) {
        while self.pc < self.ram.len() as u64 {
            // *Fetch* and *decode* the current instruction.
            let inst = self.fetch();

            // We need to add 4 bytes to the program counter,
            // as a single instruction is 4 bytes long.
            self.pc += Instruction::BYTES as u64;

            // *Execute* the current instruction.
            self.execute(inst);
        }
    }

    fn fetch(&self) -> Instruction {
        Instruction::Add
    }

    fn execute(&mut self, inst: Instruction) {}
}
