use crate::prelude::*;
use flagset::flags;

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

pub struct Ram([UnsignedLongWord; RAM_SIZE]);

impl std::ops::Index<usize> for Ram {
    type Output = UnsignedLongWord;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([UnsignedLongWord::zero(); RAM_SIZE])
    }
}

flags! {
    pub enum Flag: u8 {
        // TODO: Actual flags (max 6)
        A,
        B,
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
    word: Word<sig::Unsigned>,
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
        *self.word.raw_inner_mut() = value;
    }
}

#[derive(Debug)]
pub struct Address(LongWord<sig::Unsigned>);

impl Address {
    pub fn from_words(high: Word<sig::Unsigned>, low: Word<sig::Unsigned>) -> Self {
        Address(LongWord::<sig::Unsigned>::from_words(high, low))
    }

    pub fn increment(&mut self) {
        if self.0.low.value() < MAX_UNSIGNED_WORD_VALUE {
            *self.0.low.raw_inner_mut() += 1;
        } else {
            *self.0.low.raw_inner_mut() = 0;
            debug_assert!(self.0.high.value() < MAX_UNSIGNED_WORD_VALUE);
            *self.0.high.raw_inner_mut() += 1;
        }
    }

    pub fn value(&self) -> u16 {
        self.0.value()
    }
}

impl Default for Address {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Cpu {
    pub a: Word<sig::Unsigned>,
    pub flags: FlagWord,
    pub bh: Word<sig::Unsigned>,
    pub bl: Word<sig::Unsigned>,
    pub ch: Word<sig::Unsigned>,
    pub cl: Word<sig::Unsigned>,
    pub x: Word<sig::Unsigned>,
    pub sp: Word<sig::Unsigned>,
    pub pc: Address,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: Default::default(),
            flags: Default::default(),
            bh: Default::default(),
            bl: Default::default(),
            ch: Default::default(),
            cl: Default::default(),
            x: Default::default(),
            sp: Default::default(),
            pc: Default::default(),
        }
    }
}
