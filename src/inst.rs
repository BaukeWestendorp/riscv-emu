/// Represents a RISC-V instruction.
#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    /// Load upper immediate.
    Lui,
    /// Add upper immed to PC.
    Auipc,

    /// Jump and link.
    Jal,

    /// Branch if equal.
    Beq,
    /// Branch not equal.
    Bne,
    /// Branch if less than.
    Blt,
    /// Branch if greater than.
    Bge,
    /// Branch if less than or equal, unsigned.
    Bltu,
    /// Branch if greater than, unsigned.
    Bgeu,

    /// Jump & link register.
    Jalr,

    /// Load bytes
    Lb,
    /// Load halfword
    Lh,
    /// Load word
    Lw,
    /// Load byte unsigned.
    Lbu,
    /// Load halfword unsigned.
    Lhu,

    /// Add immediate.
    Addi,
    /// Set less than immediate.
    Slti,
    /// Set less than immediate, unsigned.
    Sltiu,
    /// XOR immediate.
    Xori,
    /// OR immediate.
    Ori,
    /// AND immediate.
    Andi,

    /// Store byte.
    Sb,
    /// Store halfword.
    Sh,
    /// Store word.
    Sw,

    /// Constant-shift left.
    Slli,
    /// Constant-shift right.
    Srli,
    /// Constant-shift right arithmetic.
    Srai,

    /// Add.
    Add,
    /// Subtract.
    Sub,
    /// Register-shift left.
    Sll,
    /// Set less than.
    Slt,
    /// Set less than unsigned.
    Sltu,
    /// XOR.
    Xor,
    /// Register-shift right (local).
    Srl,
    /// Register-shift right (arithmetic).
    Sra,
    /// OR.
    Or,
    /// AND.
    And,

    Fence,
    ECall,
    EBreak,

    /// Unknown.
    Unknown,
}
impl std::fmt::Display for InstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("<unknown>"),
            other => f.write_str(&format!("{other:?}").to_ascii_lowercase()),
        }
    }
}

bitfield::bitfield! {
    pub struct Instruction(u32);

    #[inline] pub u32, opcode, _: 6,  0;
    #[inline] pub u32, rd,     _: 11, 7;
    #[inline] pub u32, funct3, _: 14, 12;
    #[inline] pub u32, rs1,    _: 19, 15;
    #[inline] pub u32, rs2,    _: 24, 20;
    #[inline] pub u32, funct7, _: 31, 25;
}

impl Instruction {
    pub const BYTES: usize = size_of::<u32>();

    pub fn kind(&self) -> InstructionKind {
        match (self.opcode(), self.funct3(), self.funct7()) {
            (0b0110111, _, _) => InstructionKind::Lui,
            (0b0010111, _, _) => InstructionKind::Auipc,

            (0b1101111, _, _) => InstructionKind::Jal,

            (0b1100011, 0b000, _) => InstructionKind::Beq,
            (0b1100011, 0b001, _) => InstructionKind::Bne,
            (0b1100011, 0b100, _) => InstructionKind::Blt,
            (0b1100011, 0b101, _) => InstructionKind::Bge,
            (0b1100011, 0b110, _) => InstructionKind::Bltu,
            (0b1100011, 0b111, _) => InstructionKind::Bgeu,

            (0b1100111, 0b000, _) => InstructionKind::Jalr,

            (0b0000011, 0b000, _) => InstructionKind::Lb,
            (0b0000011, 0b001, _) => InstructionKind::Lh,
            (0b0000011, 0b010, _) => InstructionKind::Lw,
            (0b0000011, 0b100, _) => InstructionKind::Lbu,
            (0b0000011, 0b101, _) => InstructionKind::Lhu,

            (0b0010011, 0b000, _) => InstructionKind::Addi,
            (0b0010011, 0b010, _) => InstructionKind::Slti,
            (0b0010011, 0b011, _) => InstructionKind::Sltiu,
            (0b0010011, 0b100, _) => InstructionKind::Xori,
            (0b0010011, 0b110, _) => InstructionKind::Ori,
            (0b0010011, 0b111, _) => InstructionKind::Andi,

            (0b0100011, 0b000, _) => InstructionKind::Sb,
            (0b0100011, 0b001, _) => InstructionKind::Sh,
            (0b0100011, 0b010, _) => InstructionKind::Sw,

            (0b0010011, 0b001, 0b0000000) => InstructionKind::Slli,
            (0b0010011, 0b101, 0b0000000) => InstructionKind::Srli,
            (0b0010011, 0b101, 0b0100000) => InstructionKind::Srai,

            (0b0000000, 0b000, 0b0110011) => InstructionKind::Add,
            (0b0100000, 0b000, 0b0110011) => InstructionKind::Sub,
            (0b0000000, 0b001, 0b0110011) => InstructionKind::Sll,
            (0b0000000, 0b010, 0b0110011) => InstructionKind::Slt,
            (0b0000000, 0b011, 0b0110011) => InstructionKind::Sltu,
            (0b0000000, 0b100, 0b0110011) => InstructionKind::Xor,
            (0b0000000, 0b101, 0b0110011) => InstructionKind::Srl,
            (0b0100000, 0b101, 0b0110011) => InstructionKind::Sra,
            (0b0000000, 0b110, 0b0110011) => InstructionKind::Or,
            (0b0000000, 0b111, 0b0110011) => InstructionKind::And,

            _ => InstructionKind::Unknown,
        }
    }

    /// Immediate value for I-type instructions.
    ///
    /// (`imm[11:0]`)
    pub fn imm_i(&self) -> i32 {
        let v = self.0 & 0xfff00000 >> 20;
        v as i32
    }

    pub fn imm_s(&self) -> i32 {
        todo!();
    }

    pub fn imm_b(&self) -> i32 {
        todo!();
    }

    pub fn imm_u(&self) -> i32 {
        todo!();
    }

    /// Immediate value for J-type instructions.
    ///
    /// (`imm[20|10:1|11|19:12]`)
    pub fn imm_j(&self) -> i32 {
        // imm[20]: bit 31 in instruction
        let imm20 = ((self.0 & 0x80000000) >> 31) << 20; // Sign bit
        // imm[10:1]: bits 30-21 in instruction
        let imm10_1 = ((self.0 & 0x7fe00000) >> 21) << 1; // imm[10:1]
        // imm[11]: bit 20 in instruction
        let imm11 = ((self.0 & 0x00100000) >> 20) << 11; // imm[11]
        // imm[19:12]: bits 19-12 in instruction
        let imm19_12 = ((self.0 & 0x000ff000) >> 12) << 12; // imm[19:12]

        // Combine all parts
        let offset = imm20 | imm10_1 | imm11 | imm19_12;

        offset as i32
    }
}
