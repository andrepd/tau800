use crate::prelude::*;
use flagset::flags;

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

pub type Ram = [UnsignedLongWord; RAM_SIZE];

impl Default for Ram {
    fn default() -> Self {
        Self(Default::default())
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
        (self.word.value() & flag) != 0
    }

    pub fn write(&self, flag: Flag, value: bool) -> () {
        let value = (value as u8) << flag;
        let new = (self.word.value() & !value) | value;
        self.word.raw_inner_mut() = value;
    }
}

pub type Register = Word<sig::Unsigned>;

pub struct Address(LongWord<sig::Unsigned>);

impl Address {
    pub fn increment(&mut self) {
        if self.0.low.value() < MAX_UNSIGNED_WORD_VALUE {
            self.0.low.raw_inner_mut() += 1;
        } else {
            self.0.low.raw_inner_mut() = 0;
            debug_assert!(self.0.high.value() < MAX_UNSIGNED_WORD_VALUE);
            self.0.high.raw_inner_mut() += 1;
        }
    }
}

pub struct Cpu {
    pub a: Register,
    pub flags: FlagWord,
    pub bh: Register,
    pub bl: Register,
    pub ch: Register,
    pub cl: Register,
    pub x: Register,
    pub sp: Register,
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
