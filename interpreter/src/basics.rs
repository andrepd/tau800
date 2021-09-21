use std::{marker::PhantomData, ops::Add};

const WORD_SIZE: usize = 6;
const MAX_UNSIGNED_VALUE: u8 = 1 << WORD_SIZE;

pub mod word {
    pub trait Signature {}

    pub type Signed = i8;
    pub type Unsigned = u8;

    impl Signature for Signed {}
    impl Signature for Unsigned {}
}

#[derive(Debug, Clone, Copy)]
pub struct Word<X>
where
    X: word::Signature,
{
    value: u8,
    phantom: PhantomData<X>,
}

impl Word<word::Unsigned> {
    pub fn value(&self) -> u8 {
        self.value
    }
}

impl Word<word::Signed> {
    pub fn value(&self) -> i8 {
        self.value as i8
    }
}

impl From<u8> for Word<word::Unsigned> {
    fn from(value: u8) -> Self {
        debug_assert!(value < MAX_UNSIGNED_VALUE);
        Word {
            value,
            phantom: PhantomData::default(),
        }
    }
}

impl From<Word<word::Unsigned>> for u8 {
    fn from(word: Word<word::Unsigned>) -> Self {
        word.value
    }
}

impl From<i8> for Word<word::Signed> {
    fn from(value: i8) -> Self {
        debug_assert!((value as u8) < MAX_UNSIGNED_VALUE);
        Word {
            value: (value as u8),
            phantom: PhantomData::default(),
        }
    }
}

impl From<Word<word::Signed>> for i8 {
    fn from(word: Word<word::Signed>) -> Self {
        word.value as i8
    }
}

impl<X> Default for Word<X>
where
    X: word::Signature + Default,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
            phantom: PhantomData::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LongWord<S>
where
    S: word::Signature,
{
    high: Word<S>,
    low: Word<S>,
}

impl LongWord<word::Unsigned> {
    pub fn from_words(high: Word<word::Unsigned>, low: Word<word::Unsigned>) -> Self {
        Address { high, low }
    }
}

impl Into<u16> for LongWord<word::Unsigned> {
    fn into(self) -> u16 {
        (u8::from(self.low) as u16) + ((u8::from(self.high) as u16) << WORD_SIZE)
    }
}

impl Default for LongWord<word::Unsigned> {
    fn default() -> Self {
        Self {
            high: Default::default(),
            low: Default::default(),
        }
    }
}

pub type UnsignedLongWord = LongWord<word::Unsigned>;
pub type Address = LongWord<word::Unsigned>;
