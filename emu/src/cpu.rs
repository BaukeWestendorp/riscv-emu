use std::cell::Cell;

use crate::{
    inst::{Instruction, InstructionKind},
    ixlen,
    reg::Registers,
    rom::Rom,
    uxlen,
};

type HandleECall = dyn Fn(&Cpu);

/// Represents the RISC-V CPU.
pub struct Cpu<'rom> {
    /// A small amoumt of fast, general purpouse registers.
    /// Each register has a role defined by the integer register convention.
    regs: Registers,
    /// The program counter. Holds the address of the current opcode.
    pc: uxlen,
    /// The ROM containing the program.
    rom: &'rom Rom<'rom>,

    /// Whether or not the CPU is currently running.
    running: Cell<bool>,

    /// A callback function to run when the CPU encounters an ECALL instruction.
    handle_ecall: Option<Box<HandleECall>>,

    /// Whether to print information about the current instruction for each cycle.
    verbose: bool,
}

impl<'rom> Cpu<'rom> {
    /// Creates a new [Cpu] struct with the given ROM.
    pub fn new(rom: &'rom Rom, verbose: bool) -> Self {
        Self {
            regs: Registers::new(rom.size()),
            pc: rom.start_addr(),
            rom,
            running: Cell::new(false),
            handle_ecall: None,
            verbose,
        }
    }

    pub fn on_ecall(mut self, f: Box<HandleECall>) -> Self {
        self.handle_ecall = Some(f);
        self
    }

    pub fn registers(&self) -> &Registers {
        &self.regs
    }

    pub fn pc(&self) -> uxlen {
        self.pc
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }

    pub fn running(&self) -> bool {
        self.running.get()
    }

    /// Starts the CPU cycle loop. It will infinitely run
    /// the 'fetch, decode, execute' cycle until
    /// the user stops the emulator explicitly,
    /// or an unrecoverable error is encountered.
    pub fn run(mut self) -> anyhow::Result<()> {
        self.running.set(true);

        while self.pc < self.rom.end_addr() && self.running() {
            // Hard-wire the zero register to 0.
            self.regs.set_zero(0);

            let instruction_addr = self.pc;

            // *Fetch* the current instruction.
            let inst = self.fetch()?;

            // FIXME: This is a temporary solution to stop test programs from running after finishing.
            if inst == 0xC0001073 {
                break;
            }

            // *Decode* the current instruction.
            let instruction = self.decode(inst);

            // *Execute* the current instruction.
            self.execute(instruction, instruction_addr);

            // We need to add 4 bytes to the program counter,
            // as a single instruction is 4 bytes long.
            self.pc += Instruction::BYTES as uxlen;
        }

        Ok(())
    }

    pub fn abort(&self) {
        self.running.set(false);
    }

    /// Decodes the u32 we just fetched into an [Instruction].
    fn decode(&self, inst: uxlen) -> Instruction {
        Instruction(inst.to_le())
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

        Ok(u32::from_le_bytes(bytes))
    }

    /// Execute the given [Instruction].
    /// This is the third step in a CPU cycle.
    fn execute(&mut self, inst: Instruction, addr: uxlen) {
        if self.verbose {
            eprintln!("${:08x?}: ({:#010x?}) {:?}", self.pc, inst.0, inst);
        }

        match inst.kind() {
            InstructionKind::Lui => {
                // SPEC: LUI (load upper immediate) is used to build 32-bit constants and uses the U-type format. LUI places
                //       the 32-bit U-immediate value into the destination register rd, filling in the lowest 12 bits with zeros.
                let value = inst.imm_u() & 0x7ffff000;

                // SPEC: The 32-bit result is sign-extended to 64 bits.
                let value = value as i64;

                self.regs[inst.rd() as usize] = value as uxlen;
            }
            InstructionKind::Auipc => {
                // SPEC: AUIPC (add upper immediate to pc) is used to build pc-relative addresses and uses the U-type format.
                //       AUIPC forms a 32-bit offset from the U-immediate, filling in the lowest 12 bits with zeros,
                let offset = (inst.imm_u() & 0x7ffff000) as ixlen;

                // SPEC: sign-extends the result to 64 bits,
                let offset = offset as i64;

                // SPEC: adds this offset to the address of the AUIPC instruction,
                let target_addr = (addr as ixlen).wrapping_add(offset as ixlen) as uxlen;

                // SPEC: then places the result in register rd.
                self.regs[inst.rd() as usize] = target_addr;
            }

            InstructionKind::Jal => {
                // SPEC: The jump and link (JAL) instruction uses the J-type format, where the J-immediate encodes a signed
                //       offset in multiples of 2 bytes.
                // NOTE: This is because RISC-V instructions are always aligned on 2-byte (16-bit) or 4-byte (32-bit) boundaries.
                let byte_offset = inst.imm_j() * 2;

                // SPEC: The offset is sign-extended and added to the address of
                //       the jump instruction to form the jump target address.
                //       Jumps can therefore target a ±1 MiB range.
                let target_addr = (addr as ixlen).wrapping_add(byte_offset) as uxlen;

                // SPEC: JAL stores the address of the instruction following the jump ('pc'+4) into register rd.
                self.regs[inst.rd() as usize] = self.pc + Instruction::BYTES as uxlen;
                self.pc = target_addr;
            }

            // SPEC: All branch instructions use the B-type instruction format. The 12-bit B-immediate encodes signed
            //       offsets in multiples of 2 bytes. The offset is sign-extended and added to the address of the branch
            //       instruction to give the target address. The conditional branch range is ±4 KiB.
            //
            //       Branch instructions compare two registers.
            //
            // FIXME: The conditional branch instructions will generate an instruction-address-misaligned exception if the
            //        target address is not aligned to a four-byte boundary and the branch condition evaluates to true. If the
            //        branch condition evaluates to false, the instruction-address-misaligned exception will not be raised
            InstructionKind::Beq => {
                // SPEC: BEQ takes the branch if registers rs1 and rs2 are equal.

                if self.regs[inst.rs1() as usize] == self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bne => {
                // SPEC: BNE takes the branch if registers rs1 and rs2 are unequal.

                if self.regs[inst.rs1() as usize] != self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Blt => {
                // SPEC: BLT takes the branch if registers rs1 is less than rs2.

                if self.regs[inst.rs1() as usize] < self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bge => {
                // SPEC: BGE takes the branch if registers rs1 is greater than or equal to rs2.

                if self.regs[inst.rs1() as usize] >= self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bltu => {
                // SPEC: BLTU takes the branch if registers rs1 is less than rs2.

                if self.regs[inst.rs1() as usize] < self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }
            InstructionKind::Bgeu => {
                // SPEC: BGEU takes the branch if registers rs1 is greater than or equal to rs2.

                if self.regs[inst.rs1() as usize] >= self.regs[inst.rs2() as usize] {
                    let target_addr = self.pc.wrapping_add(inst.imm_b() as uxlen);
                    self.pc = target_addr;
                }
            }

            InstructionKind::Jalr => todo!("JALR instruction not implemented"),

            InstructionKind::Lb => todo!("LB instruction not implemented"),
            InstructionKind::Lh => todo!("LBU instruction not implemented"),
            InstructionKind::Lw => todo!("LW instruction not implemented"),
            InstructionKind::Lbu => todo!("LBU instruction not implemented"),
            InstructionKind::Lhu => todo!("LHU instruction not implemented"),

            InstructionKind::Addi => {
                // SPEC: ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the
                //       result is simply the low XLEN bits of the result.

                let imm = inst.imm_i() as ixlen;
                let rs1 = self.regs[inst.rs1() as usize] as ixlen;
                let value = rs1.wrapping_add(imm);
                self.regs[inst.rd() as usize] = value as uxlen;
            }

            InstructionKind::Slti => todo!("SLTI instruction not implemented"),
            InstructionKind::Sltiu => todo!("SLTIU instruction not implemented"),
            InstructionKind::Xori => {
                // SPEC: XORI is a logical operations that perform bitwise XOR on register rs1 and
                //       the sign-extended 12-bit immediate and place the result in rd.

                let rs1 = self.regs[inst.rs1() as usize] as ixlen;
                let imm = inst.imm_i() as ixlen;
                self.regs[inst.rd() as usize] = (rs1 ^ imm) as uxlen;
            }
            InstructionKind::Ori => {
                // SPEC: ORI is a logical operations that perform bitwise OR on register rs1 and
                //       the sign-extended 12-bit immediate and place the result in rd.

                let rs1 = self.regs[inst.rs1() as usize] as ixlen;
                let imm = inst.imm_i() as ixlen;
                self.regs[inst.rd() as usize] = (rs1 | imm) as uxlen;
            }
            InstructionKind::Andi => {
                // SPEC: ANDI is a logical operations that perform bitwise AND on register rs1 and
                //       the sign-extended 12-bit immediate and place the result in rd.

                let rs1 = self.regs[inst.rs1() as usize] as ixlen;
                let imm = inst.imm_i() as ixlen;
                self.regs[inst.rd() as usize] = (rs1 & imm) as uxlen;
            }

            InstructionKind::Sb => todo!("SB instruction not implemented"),
            InstructionKind::Sh => todo!("SH instruction not implemented"),
            InstructionKind::Sw => todo!("SW instruction not implemented"),

            InstructionKind::Slli => {
                // SPEC: Shifts by a constant are encoded as a specialization of the I-type format.
                //       The operand to be shifted is in rs1, and the shift amount is encoded in
                //       the lower 5 bits of the I-immediate field. The right shift type is
                //       encoded in bit 30.

                // SPEC: SLLI is a logical left shift (zeros are shifted into the lower bits);
                let shamt = inst.imm_i() & 0b11111;
                let value = self.regs[inst.rs1() as usize] << shamt;
                self.regs[inst.rd() as usize] = value;
            }
            InstructionKind::Srli => {
                // SPEC: SRLI is a logical right shift (zeros are shifted into the upper bits);
                todo!("SRLI instruction not implemented");
            }
            InstructionKind::Srai => {
                // SPEC: SRAI is an arithmetic right shift (the original sign bit is copied into the vacated upper bits).
                todo!("SRAI instruction not implemented");
            }

            InstructionKind::Add => todo!("ADD instruction not implemented"),
            InstructionKind::Sub => todo!("SUB instruction not implemented"),
            InstructionKind::Sll => todo!("SLL instruction not implemented"),
            InstructionKind::Slt => todo!("SLT instruction not implemented"),
            InstructionKind::Sltu => todo!("SLTU instruction not implemented"),
            InstructionKind::Xor => todo!("XOR instruction not implemented"),
            InstructionKind::Srl => todo!("SRL instruction not implemented"),
            InstructionKind::Sra => todo!("SRA instruction not implemented"),
            InstructionKind::Or => todo!("OR instruction not implemented"),
            InstructionKind::And => todo!("AND instruction not implemented"),

            InstructionKind::Fence => {}
            InstructionKind::ECall => {
                self.handle_ecall.as_ref().map(|f| f(self));
            }
            InstructionKind::EBreak => {}

            InstructionKind::Unknown => {}
        }
    }
}
