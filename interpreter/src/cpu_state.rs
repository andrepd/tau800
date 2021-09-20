use crate::prelude::*;

#[derive(Debug)]
pub struct Flags(u8);

impl Flags {
    fn new() -> Self {
        Flags(0)
    }
}

/*impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}*/

#[derive(Debug)]
pub struct CpuState {
    pub a: Word,
    pub flags: Flags,
    pub bh: Word,
    pub bl: Word,
    pub ch: Word,
    pub cl: Word,
    pub x: Word,
    pub sp: Word,
    pub pc: Address,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            a: 0,
            bh: 0,
            bl: 0,
            ch: 0,
            cl: 0,
            x: 0,
            sp: 0,
            pc: 0,
            flags: Flags::new(),
        }
    }
}
