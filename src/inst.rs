/// Represents a RISC-V opcode.
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Add,
    AddI,
    Unknown(u8),
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x13 => Self::AddI,
            0x33 => Self::Add,
            byte => Self::Unknown(byte),
        }
    }
}

bitfield::bitfield! {
    pub struct Instruction(u32);
    #[inline] pub u8, into Opcode, opcode, _: 6,  0;
    #[inline] pub u8, into usize,  rd,     _: 11, 7;
    #[inline] pub u8, into usize,  funct3, _: 14, 12;
    #[inline] pub u8, into usize,  rs1,    _: 19, 15;
    #[inline] pub u8, into usize,  rs2,    _: 24, 20;
    #[inline] pub u8, into usize,  funct7, _: 31, 25;

    #[inline] pub u16, imm_11_0,  _: 31, 20;

    #[inline] pub u16, imm_31_12, _: 31, 12;

    #[inline] pub u16, imm_11_5,  _: 31, 25;
    #[inline] pub u16, imm_4_0,   _: 11, 7;

    #[inline] pub u16, imm_12,    _: 31;
    #[inline] pub u16, imm_12_5,  _: 30, 25;
    #[inline] pub u16, imm_4_1,   _: 11, 7;

    #[inline] pub u16, imm_20,    _: 31;
    #[inline] pub u16, imm_10_1,  _: 30, 21;
    #[inline] pub u16, imm_11,    _: 20;
    #[inline] pub u16, imm_19_12, _: 19, 12;
}

impl Instruction {
    pub const BYTES: u32 = u32::BITS / 8;
}
