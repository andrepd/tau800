use crate::prelude::*;
use flagset::flags;

// CPU

flags! {
    pub enum Flag: u8 {
        /// Negative: set if value is negative
        N,
        /// Overflow: set if signed arithmetic overflows
        V,
        /// Zero: set if value is zero
        Z, 
        /// Carry: set if unsigned overflows the register
        C,
    }
}

impl From<Flag> for u8 {
    fn from(flag: Flag) -> Self {
        flag.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FlagWord {
    word: UWord,
}

impl Default for FlagWord {
    fn default() -> Self {
        Self {
            word: Default::default(),
        }
    }
}

impl FlagWord {
    pub fn read(&self, flag: Flag) -> bool {
        (self.word.value() & u8::from(flag)) != 0
    }

    pub fn write(&mut self, flag: Flag, value: bool) -> () {
        let value = (value as u8) << u8::from(flag);
        let new = (self.word.value() & !value) | value;
        *self.word.raw_inner_mut() = new;
    }
}

pub type Address = ULongWord;

pub struct Cpu {
    pub a: UWord,
    pub flags: FlagWord,
    pub bh: UWord,
    pub bl: UWord,
    pub ch: UWord,
    pub cl: UWord,
    pub x: UWord,
    pub sp: Address,
    pub pc: Address,
}

impl Default for Cpu {
    fn default() -> Self {
        let pc = Address::from(0x80 as u16);
        Self {
            a: Default::default(),
            flags: Default::default(),
            bh: Default::default(),
            bl: Default::default(),
            ch: Default::default(),
            cl: Default::default(),
            x: Default::default(),
            sp: Default::default(),
            pc: pc,
        }
    }
}

// RAM

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

pub struct Ram([UWord; RAM_SIZE]);

impl std::ops::Index<usize> for Ram {
    type Output = UWord;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Index<Address> for Ram {
    type Output = UWord;

    fn index(&self, index: Address) -> &Self::Output {
        &self.0[index.value() as usize]
    }
}

impl std::ops::IndexMut<usize> for Ram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::IndexMut<Address> for Ram {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        &mut self.0[index.value() as usize]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([Word::<sig::Unsigned>::zero(); RAM_SIZE])
    }
}