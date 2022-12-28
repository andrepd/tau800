use super::prelude::*;

/// The number of bits in a word.
pub const WORD_SIZE: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct uWord (u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct iWord (i8);

impl uWord {
    pub const fn value(self) -> u8 { self.0 }

    pub const MIN: Self = uWord(0);
    pub const MAX: Self = uWord((1 << WORD_SIZE) as u8 - 1);
    pub const ZERO: Self = uWord(0);

    pub fn as_iword(self) -> iWord { iWord(self.0 as i8) }
    pub fn sign_bit(self) -> bool { self.0 & (1 << (WORD_SIZE-1)) != 0 }

    /// Convenience function
    pub fn lit(x: u8) -> Self { 
        debug_assert!(Self::MIN.0 <= x && x <= Self::MAX.0);
        Self(x)
    }
}

impl iWord {
    pub const fn value(self) -> i8 { self.0 }

    pub const MIN: Self = iWord(-Self::MAX.0 - 1);
    pub const MAX: Self = iWord((1 << (WORD_SIZE - 1)) as i8 - 1);
    pub const ZERO: Self = iWord(0);

    pub fn as_uword(self) -> uWord { uWord(self.0 as u8) }
    pub fn sign_bit(self) -> bool { self.0 & (1 << (WORD_SIZE-1)) != 0 }
}

impl From<uWord> for u8  { fn from(x: uWord) -> Self { x.value() as u8  } }
impl From<uWord> for u16 { fn from(x: uWord) -> Self { x.value() as u16 } }
impl From<uWord> for u32 { fn from(x: uWord) -> Self { x.value() as u32 } }
impl From<uWord> for u64 { fn from(x: uWord) -> Self { x.value() as u64 } }
impl From<uWord> for usize { fn from(x: uWord) -> Self { x.value() as usize } }

impl From<iWord> for u8  { fn from(x: iWord) -> Self { x.value() as u8  } }
impl From<iWord> for u16 { fn from(x: iWord) -> Self { x.value() as u16 } }
impl From<iWord> for u32 { fn from(x: iWord) -> Self { x.value() as u32 } }
impl From<iWord> for u64 { fn from(x: iWord) -> Self { x.value() as u64 } }
impl From<iWord> for usize { fn from(x: iWord) -> Self { x.value() as usize } }

impl From<iWord> for i8  { fn from(x: iWord) -> Self { x.value() as i8  } }
impl From<iWord> for i16 { fn from(x: iWord) -> Self { x.value() as i16 } }
impl From<iWord> for i32 { fn from(x: iWord) -> Self { x.value() as i32 } }
impl From<iWord> for i64 { fn from(x: iWord) -> Self { x.value() as i64 } }
impl From<iWord> for isize { fn from(x: iWord) -> Self { x.value() as isize } }

// Non-idiomatic. "Reinterpret-cast" != "into"
/*impl Into<iWord> for uWord {
    fn into(self) -> iWord { iWord(self.0 as i8) }
}

impl Into<uWord> for iWord {
    fn into(self) -> uWord { uWord(self.0 as u8) }
}*/

impl TryFrom<u8> for uWord {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::MIN.0 <= value && value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl TryFrom<i8> for iWord {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if Self::MIN.0 <= value && value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct uLong (u16);

impl uLong {
    pub const fn value(self) -> u16 { self.0 }

    pub const MIN: Self = uLong(0);
    pub const MAX: Self = uLong((1 << (2*WORD_SIZE)) as u16 - 1);
    pub const ZERO: Self = uLong(0);

    pub fn from_hi_lo(hi: uWord, lo: uWord) -> Self {
        Self((hi.0 as u16) << WORD_SIZE | lo.0 as u16)
    }

    pub fn lo(self) -> uWord {
        uWord(self.0 as u8 & uWord::MAX.value())
    }

    pub fn hi(self) -> uWord {
        uWord((self.0 >> WORD_SIZE) as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct iLong (i16);

impl iLong {
    pub const fn value(self) -> i16 { self.0 }

    pub const MIN: Self = iLong(-Self::MAX.0 - 1);
    pub const MAX: Self = iLong((1 << (2*WORD_SIZE - 1)) as i16 - 1);
    pub const ZERO: Self = iLong(0);

    pub fn from_hi_lo(hi: iWord, lo: uWord) -> Self {
        Self((hi.0 as i16) << WORD_SIZE | lo.0 as i16)
    }

    pub fn lo(self) -> uWord {
        uWord(self.0 as u8 & uWord::MAX.value())
    }

    pub fn hi(self) -> iWord {
        iWord((self.0 >> WORD_SIZE) as i8)
    }
}

impl From<uLong> for u16 { fn from(x: uLong) -> Self { x.value() as u16 } }
impl From<uLong> for u32 { fn from(x: uLong) -> Self { x.value() as u32 } }
impl From<uLong> for u64 { fn from(x: uLong) -> Self { x.value() as u64 } }
impl From<uLong> for usize { fn from(x: uLong) -> Self { x.value() as usize } }

impl From<iLong> for i16 { fn from(x: iLong) -> Self { x.value() as i16 } }
impl From<iLong> for i32 { fn from(x: iLong) -> Self { x.value() as i32 } }
impl From<iLong> for i64 { fn from(x: iLong) -> Self { x.value() as i64 } }
impl From<iLong> for isize { fn from(x: iLong) -> Self { x.value() as isize } }

impl TryFrom<u16> for uLong {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if Self::MIN.0 <= value && value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl TryFrom<i16> for iLong {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if Self::MIN.0 <= value && value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

// Operations

impl<T: Into<i32>> std::ops::Add<T> for uWord {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let sum = self.value() as i32 + other.into();
        Self((sum as u8) << 2 >> 2)
    }
}

impl<T: Into<i32>> std::ops::Add<T> for iWord {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let sum = self.value() as i32 + other.into();
        Self((sum as i8) << 2 >> 2)
    }
}

impl<T: Into<i32>> std::ops::Add<T> for uLong {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let sum = self.value() as i32 + other.into();
        Self((sum as u16) << 4 >> 4)
    }
}

impl<T: Into<i32>> std::ops::Add<T> for iLong {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let sum = self.value() as i32 + other.into();
        Self((sum as i16) << 4 >> 4)
    }
}
