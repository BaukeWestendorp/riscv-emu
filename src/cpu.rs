use anyhow::Context;

use crate::{
    inst::{Instruction, Opcode},
    reg::Registers,
    xlen,
};

/// Represents the RISC-V CPU.
#[derive(Debug, Clone)]
pub struct Cpu {
    /// A small amoumt of fast, general purpouse registers.
    /// Each register has a role defined by the integer register convention.
    regs: Registers,
    /// The program counter. Holds the address of the current opcode.
    pc: xlen,
    // FIXME: Move this to a memory bus.
    memory: Vec<u8>,
}

impl Cpu {
    /// Creates a new [Cpu] struct with the given memory.
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            regs: Registers::default(),
            pc: 0,
            memory,
        }
    }

    /// Starts the CPU cycle loop. It will infinitely run
    /// the 'fetch, decode, execute' cycle until
    /// the user stops the emulator explicitly,
    /// or an unrecoverable error is encountered.
    pub fn run(mut self) -> anyhow::Result<()> {
        while self.pc < self.memory.len() as xlen {
            // Hard-wire the zero register to 0.
            self.regs.set_zero(0);

            // *Fetch* the current instruction.
            let inst = self.fetch()?;

            // *Decode* the current instruction.
            let instruction = Instruction(inst);

            // *Execute* the current instruction.
            self.execute(instruction);
        }

        dbg!(self);

        Ok(())
    }

    /// Read the current instruction bytes at the program counter and add step to the next instruction.
    /// This is the first step in a CPU cycle.
    fn fetch(&mut self) -> anyhow::Result<u32> {
        let ix = self.pc as usize;
        let bytes = self.memory[ix..ix + 4]
            .try_into()
            .context("Failed to get instruction from memory")?;

        // We need to add 4 bytes to the program counter,
        // as a single instruction is 4 bytes long.
        self.pc += Instruction::BYTES as xlen;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Execute the given [Instruction].
    /// This is the third step in a CPU cycle.
    fn execute(&mut self, inst: Instruction) {
        match inst.opcode() {
            Opcode::AddI => {
                // SPEC: ADDI adds the sign-extended 12-bit immediate to register rs1.
                //       Arithmetic overflow is ignored and the result is simply the low XLEN bits of the result.
                //       ADDI rd, rs1, 0 is used to implement the MV rd, rs1 assembler pseudo-instruction.

                // First we cast the `imm` value to i32. This put's it in a representation where it will be
                // sign extended. Then cast it to i64 to make sure it's sign extended to the full possible length of xlen.
                let imm = inst.imm_11_0() as i16 as i64 as xlen;
                let value = self.regs[inst.rs1()].wrapping_add(imm);
                self.regs[inst.rd()] = value;
            }
            Opcode::Add => {
                let value = self.regs[inst.rs1()].wrapping_add(self.regs[inst.rs2()]);
                self.regs[inst.rd()] = value;
            }
            Opcode::Unknown(byte) => {
                panic!("Unknown opcode: {byte:#2x}");
            }
        }
    }
}
