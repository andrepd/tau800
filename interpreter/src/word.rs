use crate::prelude::*;
use std::marker::PhantomData;

use self::sig::Signature;

/// The number of bits in a word.
/// (In reality words are represented by at least an `u8`, but checks are in place
///  to prevent "overflows".)
pub const WORD_SIZE: usize = 6;

pub const MAX_UNSIGNED_VALUE: u8 = (1 << WORD_SIZE) as u8 - 1;
pub const MAX_SIGNED_VALUE: i8 = (1 << (WORD_SIZE - 1)) as i8 - 1;
pub const MIN_SIGNED_VALUE: i8 = -MAX_SIGNED_VALUE - 1;
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

#[derive(Clone, Copy)]
/// A value with `WORD_SIZE` bits, that can represent a signed or unsigned bit,
/// as indicated by the `Signature` type.
pub struct Word<S>
where
    S: sig::Signature,
{
    value: u8,
    phantom: PhantomData<S>,
}

impl<S: Signature> std::fmt::Debug for Word<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Word").field("value", &self.value).finish()
    }
}

impl<S: Signature> PartialEq for Word<S> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
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
        debug_assert!(
            value <= MAX_UNSIGNED_VALUE,
            "Value ${:x} is > ${:x}",
            value,
            MAX_UNSIGNED_VALUE
        );
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
        debug_assert!(
            MIN_SIGNED_VALUE <= value && value <= MAX_SIGNED_VALUE,
            "Value {} isn't in {} – {}",
            value,
            MIN_SIGNED_VALUE,
            MAX_SIGNED_VALUE
        );

        let negative = value < 0;
        let abs = value.abs() as u8;

        let value = if negative {
            ((!abs & MAX_UNSIGNED_VALUE) + 1) & MAX_UNSIGNED_VALUE
        } else {
            abs
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

impl<S: Signature> PartialEq for LongWord<S> {
    fn eq(&self, other: &Self) -> bool {
        self.high == other.high && self.low == other.low
    }
}

pub type ULongWord = LongWord<sig::Unsigned>;

impl LongWord<sig::Unsigned> {
    pub fn value(self) -> u16 {
        (u8::from(self.low) as u16) + ((u8::from(self.high) as u16) << WORD_SIZE)
    }
}

impl From<u16> for LongWord<sig::Unsigned> {
    fn from(x: u16) -> Self {
        debug_assert!(x < 1 << (2 * WORD_SIZE));
        let (high, low) = div_rem(x, (MAX_UNSIGNED_VALUE + 1) as u16);
        LongWord {
            high: u8::into(high as u8),
            low: u8::into(low as u8),
        }
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
        let (div, rem) = div_rem(sum, MAX_UNSIGNED_VALUE + 1);
        let low = Word::<sig::Unsigned>::from(rem);
        let high = Word::<sig::Unsigned>::from(u8::from(self.high) + div);
        Self { high, low }
    }
}

impl std::ops::Add<Word<sig::Signed>> for LongWord<sig::Unsigned> {
    type Output = Self;

    fn add(self, other: Word<sig::Signed>) -> Self {
        let sum = if (other.value & SIGN_BIT) != 0 {
            let absolute_value = !(other.value - 1);
            let long_twos_complement = (!(absolute_value as u16) & (1 << (2 * WORD_SIZE - 1))) + 1;
            long_twos_complement + self.value() as u16
        } else {
            other.value as u16 + self.value() as u16
        };

        let high = (sum >> WORD_SIZE) as u8 & MAX_UNSIGNED_VALUE;
        let low = sum as u8 & MAX_UNSIGNED_VALUE;

        LongWord::from_words(Word::from(high), Word::from(low))
    }
}

// Checked increment trait and implementations

pub trait CheckedIncrement {
    fn try_increment(&mut self) -> Result<(), ()>;
}

impl CheckedIncrement for Word<sig::Signed> {
    fn try_increment(&mut self) -> Result<(), ()> {
        let value = self.value();
        if value + 1 <= MAX_SIGNED_VALUE {
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
