/// Represents a RISC-V instruction.
pub enum Instruction {
    Add,
}

impl Instruction {
    /// The length of an instruction in bytes.
    pub const BYTES: u32 = u32::BITS / 8;
}
