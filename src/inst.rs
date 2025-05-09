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

    /// Sign-extended immediate value for I-type instructions.
    ///
    /// (`imm[11:0]`)
    pub fn imm_i(&self) -> i32 {
        let imm = self.0 >> 20;
        sign_extend(imm, 12)
    }

    /// Sign-extended immediate value for S-type instructions.
    ///
    /// (`imm[11:5|4:0]`)
    #[rustfmt::skip]
    pub fn imm_s(&self) -> i32 {
        let imm11_5 = ((self.0 & 0b10000000000000000000000000000000) >> 31) << 12; // imm[11:5]
        let imm4_0 =  ((self.0 & 0b01111110000000000000000000000000) >> 25) << 5; // imm[4:0]
        let imm = imm11_5 | imm4_0;
        sign_extend(imm, 12)
    }

    /// Sign-extended immediate value for B-type instructions.
    ///
    /// (`imm[12|10:5|4:1|11]`)
    #[rustfmt::skip]
    pub fn imm_b(&self) -> i32 {
        let imm12 =   ((self.0 & 0b10000000000000000000000000000000) >> 31) << 12; // imm[12]
        let imm10_5 = ((self.0 & 0b01111110000000000000000000000000) >> 25) << 5;  // imm[10:5]
        let imm4_1 =  ((self.0 & 0b00000000000000000000111100000000) >> 8 ) << 1;  // imm[4:1]
        let imm11 =   ((self.0 & 0b00000000000000000000000010000000) >> 7 ) << 11; // imm[11]
        let imm = imm12 | imm11 | imm10_5 | imm4_1;
        sign_extend(imm, 12)
    }

    /// Sign-extended immediate value for U-type instructions.
    ///
    /// (`imm[31:12]`)
    pub fn imm_u(&self) -> i32 {
        let imm = self.0 >> 12;
        sign_extend(imm, 20)
    }

    /// Sign-extended immediate value for J-type instructions.
    ///
    /// (`imm[20|10:1|11|19:12]`)
    #[rustfmt::skip]
    pub fn imm_j(&self) -> i32 {
        let imm20 =    ((self.0 & 0b10000000000000000000000000000000) >> 31) << 20; // imm[20]
        let imm10_1 =  ((self.0 & 0b01111111111000000000000000000000) >> 21) << 1;  // imm[10:1]
        let imm11 =    ((self.0 & 0b00000000000100000000000000000000) >> 20) << 11; // imm[11]
        let imm19_12 = ((self.0 & 0b00000000000011111111000000000000) >> 12) << 12; // imm[19:12]
        let imm = imm20 | imm11 | imm10_1 | imm19_12;
        sign_extend(imm, 20)
    }
}

impl std::fmt::Debug for Instruction {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = self.kind();
        use InstructionKind as I;
        match kind {
            I::Lui     => write!(f, "lui   x{}, {:#x}",      self.rd(),  self.imm_u()),
            I::Auipc   => write!(f, "auipc x{}, {:#x}",      self.rd(),  self.imm_u()),
            I::Jal     => write!(f, "jal   x{}, {:#x}",      self.rd(),  self.imm_j()),
            I::Beq     => write!(f, "beq   x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Bne     => write!(f, "bne   x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Blt     => write!(f, "blt   x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Bge     => write!(f, "bge   x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Bltu    => write!(f, "bltu  x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Bgeu    => write!(f, "bgeu  x{}, x{}, {:#x}", self.rs1(), self.rs2(),   self.imm_b()),
            I::Jalr    => write!(f, "jalr  x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Lb      => write!(f, "lb    x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Lh      => write!(f, "lh    x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Lw      => write!(f, "lw    x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Lbu     => write!(f, "lbu   x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Lhu     => write!(f, "lhu   x{}, ${}({})",    self.rd(),  self.imm_i(), self.rs1()),
            I::Addi    => write!(f, "addi  x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Slti    => write!(f, "slti  x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Sltiu   => write!(f, "sltiu x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Xori    => write!(f, "xori  x{}, x{}, {:#x}", self.rd(),  self.rs1(),   self.imm_i()),
            I::Ori     => write!(f, "ori   x{}, x{}, {:#x}", self.rd(),  self.rs1(),   self.imm_i()),
            I::Andi    => write!(f, "andi  x{}, x{}, {:#x}", self.rd(),  self.rs1(),   self.imm_i()),
            I::Sb      => write!(f, "sb    x{}, ${}({})",    self.rs2(), self.imm_s(), self.rs1()),
            I::Sh      => write!(f, "sh    x{}, ${}({})",    self.rs2(), self.imm_s(), self.rs1()),
            I::Sw      => write!(f, "sw    x{}, ${}({})",    self.rs2(), self.imm_s(), self.rs1()),
            I::Slli    => write!(f, "slli  x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Srli    => write!(f, "srli  x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Srai    => write!(f, "srai  x{}, x{}, {}",    self.rd(),  self.rs1(),   self.imm_i()),
            I::Add     => write!(f, "add   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Sub     => write!(f, "sub   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Sll     => write!(f, "sll   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Slt     => write!(f, "slt   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Sltu    => write!(f, "sltu  x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Xor     => write!(f, "xor   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Srl     => write!(f, "srl   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Sra     => write!(f, "sra   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Or      => write!(f, "or    x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::And     => write!(f, "and   x{}, x{}, x{}",   self.rd(),  self.rs1(),   self.rs2()),
            I::Fence   => write!(f, "fence"),
            I::ECall   => write!(f, "ecall"),
            I::EBreak  => write!(f, "ebreak"),
            I::Unknown => write!(f, "<unknown instruction>"),
        }
    }
}

/// Helper function to sign-extend a value after n bits.
fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((value as i32) << shift) >> shift
}
