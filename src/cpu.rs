use crate::{
    Rom,
    inst::{Instruction, InstructionKind},
    reg::Registers,
    xlen,
};

/// Represents the RISC-V CPU.
#[derive(Debug)]
pub struct Cpu<'a> {
    /// A small amoumt of fast, general purpouse registers.
    /// Each register has a role defined by the integer register convention.
    regs: Registers,
    /// The program counter. Holds the address of the current opcode.
    pc: xlen,
    // FIXME: Move this to a memory bus.
    rom: &'a Rom<'a>,
}

impl<'a> Cpu<'a> {
    /// Creates a new [Cpu] struct with the given ROM.
    pub fn new(rom: &'a Rom) -> Self {
        Self {
            regs: Registers::new(rom.size()),
            pc: rom.start_addr(),
            rom,
        }
    }

    /// Starts the CPU cycle loop. It will infinitely run
    /// the 'fetch, decode, execute' cycle until
    /// the user stops the emulator explicitly,
    /// or an unrecoverable error is encountered.
    pub fn run(mut self) -> anyhow::Result<()> {
        while self.pc < self.rom.end_addr() {
            // Hard-wire the zero register to 0.
            self.regs.set_zero(0);

            // *Fetch* the current instruction.
            let inst = self.fetch()?;

            // *Decode* the current instruction.
            let instruction = Instruction(inst);

            // *Execute* the current instruction.
            self.execute(instruction);
        }

        Ok(())
    }

    /// Read the current instruction bytes at the program counter and add step to the next instruction.
    /// This is the first step in a CPU cycle.
    fn fetch(&mut self) -> anyhow::Result<u32> {
        let bytes = [
            self.rom.read(self.pc),
            self.rom.read(self.pc + 1),
            self.rom.read(self.pc + 2),
            self.rom.read(self.pc + 3),
        ];

        // We need to add 4 bytes to the program counter,
        // as a single instruction is 4 bytes long.
        self.pc += Instruction::BYTES as xlen;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Execute the given [Instruction].
    /// This is the third step in a CPU cycle.
    fn execute(&mut self, inst: Instruction) {
        match inst.kind() {
            InstructionKind::Addi => {}
            InstructionKind::Unknown(inst) => panic!("Unknown instruction: {inst:#x?}"),
            kind => todo!("Instruction {kind:?} not implemented"),
        }
    }
}
