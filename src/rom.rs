use crate::uxlen;

#[derive(Debug)]
pub struct Rom<'a> {
    bytes: &'a mut [u8],
    start_addr: uxlen,
    end_addr: uxlen,
}

impl<'a> Rom<'a> {
    pub fn new(bytes: &'a mut [u8], start_addr: uxlen, end_addr: uxlen) -> Self {
        Rom {
            bytes,
            start_addr,
            end_addr,
        }
    }

    #[inline]
    pub fn read(&self, addr: uxlen) -> u8 {
        self.bytes[addr as usize - self.start_addr as usize]
    }

    #[inline]
    pub fn write(&mut self, addr: uxlen, value: u8) {
        self.bytes[addr as usize - self.start_addr as usize] = value;
    }

    #[inline]
    pub fn size(&self) -> uxlen {
        self.end_addr - self.start_addr
    }

    #[inline]
    pub fn start_addr(&self) -> uxlen {
        self.start_addr
    }

    #[inline]
    pub fn end_addr(&self) -> uxlen {
        self.end_addr
    }
}
