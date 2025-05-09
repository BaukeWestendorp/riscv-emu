pub mod cpu;
pub mod inst;
pub mod reg;
pub mod rom;

/// The unsigned width of an x register in bits (either u32 or u64).
#[allow(non_camel_case_types)]
pub type uxlen = u32;

/// The signed width of an x register in bits (either i32 or i64).
#[allow(non_camel_case_types)]
pub type ixlen = i32;
