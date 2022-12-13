use super::prelude::*;
use std::marker::PhantomData;

use self::sig::Signature;

/// The number of bits in a word.
/// (In reality words are represented by at least an `u8`, but checks are in place
///  to prevent "overflows".)
pub const WORD_SIZE: usize = 6;

pub const MAX_UNSIGNED_VALUE: u8 = (1 << WORD_SIZE) as u8 - 1;
pub const MAX_SIGNED_VALUE: i8 = (1 << (WORD_SIZE - 1)) as i8 - 1;
pub const MIN_SIGNED_VALUE: i8 = -MAX_SIGNED_VALUE - 1;

pub const MAX_UNSIGNED_DOUBLE: u16 = (1 << (2*WORD_SIZE)) as u16 - 1;
pub const MAX_SIGNED_DOUBLE: i16 = (1 << (2*WORD_SIZE - 1)) as i16 - 1;
pub const MIN_SIGNED_DOUBLE: i16 = -MAX_SIGNED_DOUBLE - 1;

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
pub struct Word<S: Signature> {
    pub value: u8,
    pub phantom: PhantomData<S>,
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

impl<S: Signature> Eq for Word<S> {}

impl<S: Signature> Word<S> {
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
impl<S: Signature> Default for Word<S> {
    fn default() -> Self {
        Self::zero()
    }
}

//

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
        debug_assert_le!(value, MAX_UNSIGNED_VALUE);
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
        /*self.value as i8*/
        if self.value >= 32 {self.value as i8 - 64} else { self.value as i8 }
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
        debug_assert_le!(MIN_SIGNED_VALUE, value);
        debug_assert_le!(value, MAX_SIGNED_VALUE);

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

//

#[derive(Debug, Clone, Copy)]
/// A "word" that actually consists of two `Words`, representing high-value bits
/// and low-value bits.
pub struct LongWord<S: Signature> {
    pub high: Word<S>,
    pub low: Word<sig::Unsigned>,
}

impl<S: Signature> PartialEq for LongWord<S> {
    fn eq(&self, other: &Self) -> bool {
        self.high == other.high && self.low == other.low
    }
}

impl<S: Signature> Eq for LongWord<S> {}

impl<S: Signature> LongWord<S> {
    pub fn from_words(high: Word<S>, low: Word<sig::Unsigned>) -> Self {
        LongWord { high, low }
    }

    pub fn zero() -> Self {
        LongWord {
            high: Word::<S>::zero(),
            low: Word::<sig::Unsigned>::zero(),
        }
    }
}

impl<S: Signature> Default for LongWord<S> {
    fn default() -> Self {
        Self {
            high: Default::default(),
            low: Default::default(),
        }
    }
}

// Implementations for unsigned long words

pub type ULongWord = LongWord<sig::Unsigned>;

impl ULongWord {
    pub fn value(self) -> u16 {
        (u8::from(self.low) as u16) + ((u8::from(self.high) as u16) << WORD_SIZE)
    }
}

impl From<u16> for ULongWord {
    fn from(x: u16) -> Self {
        debug_assert_le!(x, MAX_UNSIGNED_DOUBLE);
        let (high, low) = div_rem(x, (MAX_UNSIGNED_VALUE + 1) as u16);
        LongWord {
            high: u8::into(high as u8),
            low: u8::into(low as u8),
        }
    }
}

impl From<ULongWord> for u16 {
    fn from(word: ULongWord) -> Self {
        word.value()
    }
}

impl Into<usize> for ULongWord {
    fn into(self) -> usize {
        self.value() as usize
    }
}

// Implementations for signed long words

pub type ILongWord = LongWord<sig::Signed>;

impl ILongWord {
    pub fn value(self) -> i16 {
        (u8::from(self.low) as i16) + ((i8::from(self.high) as i16) << WORD_SIZE)
    }
}

impl From<i16> for ILongWord {
    fn from(x: i16) -> Self {
        debug_assert_le!(MIN_SIGNED_DOUBLE, x);
        debug_assert_le!(x, MAX_SIGNED_DOUBLE);
        let (high, low) = div_rem(x, (MAX_UNSIGNED_VALUE + 1) as i16);
        LongWord {
            high: i8::into(high as i8),
            low: u8::into(low as u8),
        }
    }
}

impl From<ILongWord> for i16 {
    fn from(word: ILongWord) -> Self {
        word.value()
    }
}

impl Into<isize> for ILongWord {
    fn into(self) -> isize {
        self.value() as isize
    }
}

// Operations

impl std::ops::Add<Word<sig::Unsigned>> for ULongWord {
    type Output = Self;

    fn add(self, other: Word<sig::Unsigned>) -> Self {
        let sum = u8::from(self.low) + u8::from(other);
        let (div, rem) = div_rem(sum, MAX_UNSIGNED_VALUE + 1);
        let low = Word::<sig::Unsigned>::from(rem);
        let high = Word::<sig::Unsigned>::from(u8::from(self.high) + div);
        Self { high, low }
    }
}

impl std::ops::Add<Word<sig::Signed>> for ULongWord {
    type Output = Self;

    fn add(self, other: Word<sig::Signed>) -> Self {
        let x = self.value() as i16;
        let y = other.value() as i16;
        let z = x + y;
        debug_assert!(z > 0);
        let (high, low) = div_rem(z, 1 << 6);
        /*eprintln!("add {}+{}={} {} {}", x, y, z, high, low);*/
        LongWord::from_words(Word::from(high as u8), Word::from(low as u8))
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

impl CheckedIncrement for ULongWord {
    fn try_increment(&mut self) -> Result<(), ()> {
        if self.low.try_increment().is_ok() {
            Ok(())
        } else {
            self.low = Word::zero();
            self.high.try_increment()
        }
    }
}

pub static UZERO: UWord = UWord::default();
