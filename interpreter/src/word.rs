use std::marker::PhantomData;

use crate::prelude::{Address, div_rem};

/// The number of bits in a word.
/// (In reality words are represented by at least an `u8`, but checks are in place
///  to prevent "overflows".)
pub const WORD_SIZE: usize = 6;

pub const MAX_UNSIGNED_VALUE: u8 = (1 << WORD_SIZE) as u8 - 1;
pub const MAX_SIGNED_VALUE: u8 = (1 << (WORD_SIZE - 1)) as u8 - 1;
pub const SIGN_BIT: u8 = (1 << WORD_SIZE) as u8;

pub mod sig {
    pub trait Signature {}

    #[derive(Debug, Clone, Copy)]
    pub struct Signed;
    #[derive(Debug, Clone, Copy)]
    pub struct Unsigned;

    impl Signature for Signed {}
    impl Signature for Unsigned {}
}

#[derive(Debug, Clone, Copy)]
/// A value with `WORD_SIZE` bits, that can represent a signed or unsigned bit,
/// as indicated by the `Signature` type.
pub struct Word<S>
where
    S: sig::Signature,
{
    value: u8,
    phantom: PhantomData<S>,
}

impl<S> Word<S>
where
    S: sig::Signature,
{
    pub fn zero() -> Self {
        Self {
            value: 0,
            phantom: PhantomData::default(),
        }
    }

    /// Returns a mutable reference to the raw value contained by the word.
    /// Changes to this value will not be checked for overflow of the `WORD_SIZE`.
    pub fn raw_inner_mut(&mut self) -> &mut u8 {
        &mut self.value
    }
}

/// All words are by default initialized to zero.
impl<S> Default for Word<S>
where
    S: sig::Signature,
{
    fn default() -> Self {
        Self::zero()
    }
}

pub type UWord = Word<sig::Unsigned>;
pub type IWord = Word<sig::Signed>;

// Implementations for unsigned word:

impl Word<sig::Unsigned> {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn cast_to_signed(self) -> Word<sig::Signed> {
        Word::<sig::Signed> {
            value: self.value,
            phantom: PhantomData::default(),
        }
    }
}

impl From<u8> for Word<sig::Unsigned> {
    fn from(value: u8) -> Self {
        debug_assert!(value <= MAX_UNSIGNED_VALUE);
        Word {
            value,
            phantom: PhantomData::default(),
        }
    }
}

impl From<Word<sig::Unsigned>> for u8 {
    fn from(word: Word<sig::Unsigned>) -> Self {
        word.value
    }
}

// Implementations for signed word:

impl Word<sig::Signed> {
    pub fn value(&self) -> i8 {
        self.value as i8
    }

    pub fn cast_to_unsigned(self) -> Word<sig::Unsigned> {
        Word::<sig::Unsigned> {
            value: self.value,
            phantom: PhantomData::default(),
        }
    }
}

impl From<i8> for Word<sig::Signed> {
    fn from(value: i8) -> Self {
        let negative = value < 0;
        let value = value.abs() as u8;

        debug_assert!(value <= MAX_SIGNED_VALUE);

        let value = if negative {
            ((!value & MAX_UNSIGNED_VALUE) + 1) & MAX_UNSIGNED_VALUE
        } else {
            value
        };

        Word {
            value,
            phantom: PhantomData::default(),
        }
    }
}

impl From<Word<sig::Signed>> for i8 {
    fn from(word: Word<sig::Signed>) -> Self {
        if (word.value & SIGN_BIT) != 0 {
            -((!(word.value - 1) & MAX_UNSIGNED_VALUE) as i8)
        } else {
            word.value as i8
        }
    }
}

// Long word:

#[derive(Debug, Clone, Copy)]
/// A "word" that actually consists of two `Words`, representing high-value bits
/// and low-value bits.
///
/// Although the type admits `word::Signed` and `word::Unsigned` variants, the
/// `word::Signed` variant does not implement a conversion into a value, because
/// both (high and low) words are required to have the same signature, and so the
/// sign of the value is not well defined.
pub struct LongWord<S>
where
    S: sig::Signature,
{
    pub high: Word<S>,
    pub low: Word<S>,
}

impl<S> LongWord<S>
where
    S: sig::Signature,
{
    pub fn from_words(high: Word<S>, low: Word<S>) -> Self {
        LongWord { high, low }
    }

    pub fn zero() -> Self {
        LongWord {
            high: Word::<S>::zero(),
            low: Word::<S>::zero(),
        }
    }
}

pub type ULongWord = LongWord<sig::Unsigned>;

impl LongWord<sig::Unsigned> {
    pub fn value(self) -> u16 {
        (u8::from(self.low) as u16) + ((u8::from(self.high) as u16) << WORD_SIZE)
    }
}

impl From<LongWord<sig::Unsigned>> for u16 {
    fn from(word: LongWord<sig::Unsigned>) -> Self {
        word.value()
    }
}

impl Into<usize> for LongWord<sig::Unsigned> {
    fn into(self) -> usize {
        self.value() as usize
    }
}

impl Default for LongWord<sig::Unsigned> {
    fn default() -> Self {
        Self {
            high: Default::default(),
            low: Default::default(),
        }
    }
}

impl std::ops::Add<Word<sig::Unsigned>> for LongWord<sig::Unsigned> {
    type Output = Self; 
    
    fn add(self, other: Word<sig::Unsigned>) -> Self {
        let sum = u8::from(self.low) + u8::from(other);
        let (div, rem) = div_rem(sum, MAX_UNSIGNED_VALUE);
        let low  = Word::<sig::Unsigned>::from(rem);
        let high = Word::<sig::Unsigned>::from(u8::from(self.high) + div);
        Self { high, low }
    }
}

impl std::ops::Add<Word<sig::Signed>> for LongWord<sig::Unsigned> {
    type Output = Self; 
    
    fn add(self, other: Word<sig::Signed>) -> Self {
        /*let sum = u8::from(self.low) as i8 + i8::from(other);
        let (div, rem) = div_rem(sum, MAX_UNSIGNED_VALUE as i8);
        let (div, rem) = (div as u8, rem as u8);
        let low  = Word::<sig::Unsigned>::from(rem as u8);
        let high = Word::<sig::Unsigned>::from(u8::from(self.high) + div);
        Self { high, low }*/
        unimplemented!()  // Calma isto estava tudo mal
    }
}

// Checked increment trait and implementations

pub trait CheckedIncrement {
    fn try_increment(&mut self) -> Result<(), ()>;
}

impl CheckedIncrement for Word<sig::Signed> {
    fn try_increment(&mut self) -> Result<(), ()> {
        let value = self.value();
        if value + 1 <= MAX_SIGNED_VALUE as i8 {
            *self.raw_inner_mut() = (value + 1) as u8;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl CheckedIncrement for Word<sig::Unsigned> {
    fn try_increment(&mut self) -> Result<(), ()> {
        let value = self.value();
        if value + 1 <= MAX_UNSIGNED_VALUE {
            *self.raw_inner_mut() = value + 1;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl CheckedIncrement for LongWord<sig::Unsigned> {
    fn try_increment(&mut self) -> Result<(), ()> {
        if self.low.try_increment().is_ok() {
            Ok(())
        } else {
            self.low = Word::zero();
            self.high.try_increment()
        }
    }
}
