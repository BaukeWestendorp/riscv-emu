use std::ops::{Deref, DerefMut};

use crate::xlen;

/// A representation of the registers in the [Cpu][crate::Cpu].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Registers([xlen; 32]);

impl Registers {
    pub fn new(rom_size: xlen) -> Self {
        let mut this = Self([0; 32]);

        // Make sure the x0 register is set to zero.
        this.set_zero(0);
        // Set the stack pointer to the end of the ROM,
        // as it grows down into the ROM.
        this.set_sp(rom_size);

        this
    }
}

macro_rules! impl_registers {
    [$({
        ix: $ix:literal,
        r: $r:ident,
        abi: {
            get: $abi_get:ident,
            set: $abi_set:ident
        },
        desc: $desc:literal
    }),*] => {
        impl Registers {
            $(
                #[doc = $desc]
                #[doc = "\n"]
                #[doc = concat!("Get the `", stringify!($abi_get), "` register (", stringify!($r), ")")]
                #[inline]
                pub fn $abi_get(&self) -> crate::xlen {
                    self.0[$ix]
                }

                #[doc = $desc]
                #[doc = "\n"]
                #[doc = concat!("Set the `", stringify!($abi_get), "` register (", stringify!($r), ")")]
                #[inline]
                pub fn $abi_set(&mut self, $abi_get: crate::xlen) {
                    self.0[$ix] = $abi_get;
                }
            )*
        }
    };
}

impl_registers![
    { ix: 0,  r: x0,  abi: { get: zero, set: set_zero }, desc: "Hard-wired zero" },
    { ix: 1,  r: x1,  abi: { get: ra,   set: set_ra   }, desc: "Return address" },
    { ix: 2,  r: x2,  abi: { get: sp,   set: set_sp   }, desc: "Stack pointer" },
    { ix: 3,  r: x3,  abi: { get: gp,   set: set_gp   }, desc: "Global pointer" },
    { ix: 4,  r: x4,  abi: { get: tp,   set: set_tp   }, desc: "Thread pointer" },
    { ix: 5,  r: x5,  abi: { get: t0,   set: set_t0   }, desc: "Temporary 0" },
    { ix: 6,  r: x6,  abi: { get: t1,   set: set_t1   }, desc: "Temporary 1" },
    { ix: 7,  r: x7,  abi: { get: t2,   set: set_t2   }, desc: "Temporary 2" },
    { ix: 8,  r: x8,  abi: { get: s0,   set: set_s0   }, desc: "Saved register 0 / Frame pointer" },
    { ix: 9,  r: x9,  abi: { get: s1,   set: set_s1   }, desc: "Saved register 1" },
    { ix: 10, r: x10, abi: { get: a0,   set: set_a0   }, desc: "Function argument 0 / Return value 0" },
    { ix: 11, r: x11, abi: { get: a1,   set: set_a1   }, desc: "Function argument 1 / Return value 1" },
    { ix: 12, r: x12, abi: { get: a2,   set: set_a2   }, desc: "Function argument 2" },
    { ix: 13, r: x13, abi: { get: a3,   set: set_a3   }, desc: "Function argument 3" },
    { ix: 14, r: x14, abi: { get: a4,   set: set_a4   }, desc: "Function argument 4" },
    { ix: 15, r: x15, abi: { get: a5,   set: set_a5   }, desc: "Function argument 5" },
    { ix: 16, r: x16, abi: { get: a6,   set: set_a6   }, desc: "Function argument 6" },
    { ix: 17, r: x17, abi: { get: a7,   set: set_a7   }, desc: "Function argument 7" },
    { ix: 18, r: x18, abi: { get: s2,   set: set_s2   }, desc: "Saved register 2" },
    { ix: 19, r: x19, abi: { get: s3,   set: set_s3   }, desc: "Saved register 3" },
    { ix: 20, r: x20, abi: { get: s4,   set: set_s4   }, desc: "Saved register 4" },
    { ix: 21, r: x21, abi: { get: s5,   set: set_s5   }, desc: "Saved register 5" },
    { ix: 22, r: x22, abi: { get: s6,   set: set_s6   }, desc: "Saved register 6" },
    { ix: 23, r: x23, abi: { get: s7,   set: set_s7   }, desc: "Saved register 7" },
    { ix: 24, r: x24, abi: { get: s8,   set: set_s8   }, desc: "Saved register 8" },
    { ix: 25, r: x25, abi: { get: s9,   set: set_s9   }, desc: "Saved register 9" },
    { ix: 26, r: x26, abi: { get: s10,  set: set_s10  }, desc: "Saved register 10" },
    { ix: 27, r: x27, abi: { get: s11,  set: set_s11  }, desc: "Saved register 11" },
    { ix: 28, r: x28, abi: { get: t3,   set: set_t3   }, desc: "Temporary 3" },
    { ix: 29, r: x29, abi: { get: t4,   set: set_t4   }, desc: "Temporary 4" },
    { ix: 30, r: x30, abi: { get: t5,   set: set_t5   }, desc: "Temporary 5" },
    { ix: 31, r: x31, abi: { get: t6,   set: set_t6   }, desc: "Temporary 6" }
];

impl Deref for Registers {
    type Target = [xlen; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Registers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
