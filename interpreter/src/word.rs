use std::marker::PhantomData;

/// The number of bits in a word.
/// (In reality words are represented by at least an `u8`, but checks are in place
///  to prevent "overflows".)
pub const WORD_SIZE: usize = 6;

/// The maximum value an unsigned word can represent.
pub const MAX_UNSIGNED_WORD_VALUE: u8 = (1 << (WORD_SIZE + 1)) - 1;

/// The maximum absolute value a signed word can represent.
pub const MAX_SIGNED_WORD_VALUE: u8 = (1 << WORD_SIZE) - 1;

const SIGN_BIT: u8 = (1 << WORD_SIZE);

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

// Implementations for unsigned word:

impl Word<sig::Unsigned> {
    pub fn value(&self) -> u8 {
        self.value
    }
}

impl From<u8> for Word<sig::Unsigned> {
    fn from(value: u8) -> Self {
        debug_assert!(value <= MAX_UNSIGNED_WORD_VALUE);
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
        let absolute_value = (self.value & MAX_SIGNED_WORD_VALUE) as i8;
        if self.value & SIGN_BIT != 0 {
            -absolute_value
        } else {
            absolute_value
        }
    }
}

impl From<i8> for Word<sig::Signed> {
    fn from(value: i8) -> Self {
        debug_assert!((value as u8 & MAX_SIGNED_WORD_VALUE) <= MAX_SIGNED_WORD_VALUE);

        // Note that the sign bit is the `WORD_SIZE`th bit, not the last/first one.
        let value = if value < 0 {
            SIGN_BIT | (value as u8 & MAX_SIGNED_WORD_VALUE)
        } else {
            value as u8
        };

        Word {
            value: (value as u8),
            phantom: PhantomData::default(),
        }
    }
}

impl From<Word<sig::Signed>> for i8 {
    fn from(word: Word<sig::Signed>) -> Self {
        word.value()
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

impl Default for LongWord<sig::Unsigned> {
    fn default() -> Self {
        Self {
            high: Default::default(),
            low: Default::default(),
        }
    }
}

pub type UnsignedLongWord = LongWord<sig::Unsigned>;
