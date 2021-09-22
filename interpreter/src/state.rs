use crate::prelude::*;
use flagset::flags;

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

pub struct Ram([Word<sig::Unsigned>; RAM_SIZE]);

impl std::ops::Index<usize> for Ram {
    type Output = Word<sig::Unsigned>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([Word::<sig::Unsigned>::zero(); RAM_SIZE])
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

pub type Address = LongWord<sig::Unsigned>;

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
