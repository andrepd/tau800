use std::slice::SliceIndex;

use super::prelude::*;

// CPU

pub enum Flag {
    /// Negative: set if value is negative
    N = 1 << 0,
    /// Overflow: set if signed arithmetic overflows
    V = 1 << 1,
    /// Zero: set if value is zero
    Z = 1 << 2,
    /// Carry: set if unsigned overflows the register
    C = 1 << 3,
}

impl From<Flag> for u8 {
    fn from(flag: Flag) -> Self {
        flag as u8
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq)]
pub struct FlagWord {
    pub word: UWord,
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
        let mask = u8::from(flag);
        self.word.value() & mask != 0
    }

    pub fn write(&mut self, flag: Flag, value: bool) -> () {
        let mask = u8::from(flag);
        let new = if value {
            self.word.value() | mask
        } else {
            self.word.value() & !mask
        };
        *self.word.raw_inner_mut() = new
    }
}

pub type Address = ULongWord;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
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
        let pc = Address {
            high: UWord::from(0x02),
            low: UWord::from(0x00),
        };
        let sp = Address {
            high: UWord::from(0x01),
            low: UWord::from(0x3f),
        };
        Self {
            a: Default::default(),
            flags: Default::default(),
            bh: Default::default(),
            bl: Default::default(),
            ch: Default::default(),
            cl: Default::default(),
            x: Default::default(),
            sp,
            pc,
        }
    }
}

// RAM

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

#[derive(Clone)]
pub struct Ram(pub Vec<UWord>);

impl std::fmt::Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ram (omitted)").finish()
    }
}

impl PartialEq for Ram {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0 == other.0
        /*self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)*/
    }
}

impl std::ops::Index<usize> for Ram {
    type Output = UWord;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(0 <= index && index < RAM_SIZE);
        if index >= self.0.len() { return &UZERO };
        &self.0[index]
    }
}

impl std::ops::Index<Address> for Ram {
    type Output = UWord;

    fn index(&self, index: Address) -> &Self::Output {
        &self[index.value() as usize]
    }
}

impl std::ops::IndexMut<usize> for Ram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(0 <= index && index < RAM_SIZE);
        if index >= self.0.len() { self.0.resize(index+1, UZERO) };
        &mut self.0[index]
    }
}

impl std::ops::IndexMut<Address> for Ram {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        &mut self[index.value() as usize]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self(Vec::with_capacity(64 * 4))
    }
}