use crate::xlen;

#[derive(Debug)]
pub struct Rom<'a> {
    bytes: &'a mut [u8],
    start_addr: xlen,
    end_addr: xlen,
}

impl<'a> Rom<'a> {
    pub fn new(bytes: &'a mut [u8], start_addr: xlen, end_addr: xlen) -> Self {
        Rom {
            bytes,
            start_addr,
            end_addr,
        }
    }

    #[inline]
    pub fn read(&self, addr: xlen) -> u8 {
        self.bytes[addr as usize - self.start_addr as usize]
    }

    #[inline]
    pub fn write(&mut self, addr: xlen, value: u8) {
        self.bytes[addr as usize - self.start_addr as usize] = value;
    }

    #[inline]
    pub fn size(&self) -> xlen {
        self.end_addr - self.start_addr
    }

    #[inline]
    pub fn start_addr(&self) -> xlen {
        self.start_addr
    }

    #[inline]
    pub fn end_addr(&self) -> xlen {
        self.end_addr
    }
}
