use crate::{
    Rom,
    inst::{Instruction, InstructionKind},
    ixlen,
    reg::Registers,
    uxlen,
};

/// Represents the RISC-V CPU.
#[derive(Debug)]
pub struct Cpu<'a> {
    /// A small amoumt of fast, general purpouse registers.
    /// Each register has a role defined by the integer register convention.
    regs: Registers,
    /// The program counter. Holds the address of the current opcode.
    pc: uxlen,
    /// The ROM containing the program.
    rom: &'a Rom<'a>,
}

impl<'a> Cpu<'a> {
    /// Creates a new [Cpu] struct with the given ROM.
    pub fn new(rom: &'a Rom) -> Self {
        Self { regs: Registers::new(rom.size()), pc: rom.start_addr(), rom }
    }

    /// Starts the CPU cycle loop. It will infinitely run
    /// the 'fetch, decode, execute' cycle until
    /// the user stops the emulator explicitly,
    /// or an unrecoverable error is encountered.
    pub fn run(mut self) -> anyhow::Result<()> {
        while self.pc < self.rom.end_addr() {
            // Hard-wire the zero register to 0.
            self.regs.set_zero(0);

            let instruction_addr = self.pc;

            // *Fetch* the current instruction.
            let inst = self.fetch()?;

            // *Decode* the current instruction.
            let instruction = Instruction(inst);

            // *Execute* the current instruction.
            self.execute(instruction, instruction_addr);
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
        self.pc += Instruction::BYTES as uxlen;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Execute the given [Instruction].
    /// This is the third step in a CPU cycle.
    fn execute(&mut self, inst: Instruction, addr: uxlen) {
        eprintln!("Executing instruction: {}", inst.kind());
        match inst.kind() {
            InstructionKind::Jal => {
                // SPEC: JAL stores the
                //       address of the instruction following the jump ('pc'+4) into register rd. The standard software calling
                //       convention uses 'x1' as the return address register and 'x5' as an alternate link register.

                let offset = inst.imm_j() as ixlen;
                let target_addr = (addr as ixlen).wrapping_add(offset) as uxlen;
                self.regs[inst.rd() as usize] = self.pc + Instruction::BYTES as uxlen;
                self.pc = target_addr;
            }

            InstructionKind::Beq => {
                // BEQ takes the branch if registers rs1 and rs2 are equal.

                if self.regs[inst.rs1() as usize] == self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bne => {
                // BNE takes the branch if registers rs1 and rs2 are unequal.

                if self.regs[inst.rs1() as usize] != self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Blt => {
                // BLT takes the branch if registers rs1 is less than rs2.

                if self.regs[inst.rs1() as usize] < self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bge => {
                // BGE takes the branch if registers rs1 is greater than or equal to rs2.

                if self.regs[inst.rs1() as usize] >= self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bltu => {
                // BLTU takes the branch if registers rs1 is less than rs2.

                if self.regs[inst.rs1() as usize] < self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bgeu => {
                // BGEU takes the branch if registers rs1 is greater than or equal to rs2.

                if self.regs[inst.rs1() as usize] >= self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }

            InstructionKind::Addi => {
                // SPEC: ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the
                //       result is simply the low XLEN bits of the result.

                let value =
                    (self.regs[inst.rs1() as usize] as ixlen).wrapping_add(inst.imm_i() as ixlen);
                self.regs[inst.rd() as usize] = value as uxlen;
            }

            InstructionKind::Unknown => {
                eprintln!("Encountered unknown instruction. Acting as NOP")
            }

            kind => todo!("Instruction {kind} not implemented"),
        }
    }
}
