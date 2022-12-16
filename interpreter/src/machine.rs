use super::prelude::*;

use super::{modules::Module, prelude::*};

// CPU //

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
        Self { word: Default::default() }
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

// RAM //

const RAM_SIZE: usize = 1 << (2 * WORD_SIZE);

#[derive(Clone)]
#[derive(PartialEq, Eq)]  //TODO: subtle bug (because of resizing)
pub struct Ram(pub Vec<UWord>);

impl std::fmt::Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ram (omitted)").finish()
    }
}

impl std::ops::Index<usize> for Ram {
    type Output = UWord;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert_lt!(index, RAM_SIZE);
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
        debug_assert_lt!(index, RAM_SIZE);
        if index >= self.0.len() { self.0.resize(index+1, UZERO) };
        &mut self.0[index]
    }
}

impl std::ops::IndexMut<Address> for Ram {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        &mut self[index.value() as usize]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Ram {
    type Output = [UWord];

    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        debug_assert_le!(range.end, self.0.len()); //TODO
        &self.0[range]
    }
}

impl std::ops::Index<std::ops::Range<Address>> for Ram {
    type Output = [UWord];

    fn index(&self, range: std::ops::Range<Address>) -> &Self::Output {
        &self[(range.start.value() as usize)..(range.end.value() as usize)]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Ram {
    fn index_mut(&mut self, range: std::ops::Range<usize>) -> &mut Self::Output {
        debug_assert_le!(range.end, RAM_SIZE);
        if range.end >= self.0.len() { self.0.resize(range.end, UZERO) };
        &mut self.0[range]
    }
}

impl std::ops::IndexMut<std::ops::Range<Address>> for Ram {
    fn index_mut(&mut self, range: std::ops::Range<Address>) -> &mut Self::Output {
        &mut self[(range.start.value() as usize)..(range.end.value() as usize)]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self(Vec::with_capacity(64 * 4))
    }
}

// Machine //

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub struct Machine {
    pub ram: Ram,
    pub cpu: Cpu,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            ram: Ram::default(),
            cpu: Cpu::default(),
        }
    }

    fn increment_pc_or_overflow(&mut self) {
        // self.cpu.pc.try_increment().expect("Overflowed program counter.");
        // Don't panic on overflow of PC because the futures on the timeline will
        // panic when close to the maximum.
        // Instead, overflow the PC
        if self.cpu.pc.try_increment().is_err() {
            self.cpu.pc = ULongWord { low: UWord::zero(), high: UWord::zero() };
        }
    }

    /// Read the next value in ram, as indicated by the Program Counter (PC) in
    /// CPU, and increment the PC.
    pub fn read_pc(&mut self) -> UWord {
        let word = self.ram[self.cpu.pc];
        self.increment_pc_or_overflow();
        word
    }

    /// Write a word at PC and increment the PC.
    pub fn write_pc(&mut self, word: UWord) {
        self.ram[self.cpu.pc] = word;
        self.increment_pc_or_overflow();

    }

    /// Read a word from stack and increment the sp. 
    pub fn read_sp(&mut self) -> UWord {
        self.cpu.sp = self.cpu.sp + IWord::from(1);  // Ugly af
        let word = self.ram[self.cpu.sp];
        word
    }

    /// Write a word to stack and decrement the sp. 
    pub fn write_sp(&mut self, word: UWord) -> () {
        self.ram[self.cpu.sp] = word;
        self.cpu.sp = self.cpu.sp + IWord::from(-1);  // Ugly af
    }

    pub fn reset_cpu(&mut self) -> () {
        self.cpu = Cpu::default();
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cpu: a={:02x} NVZC={}{}{}{} b={:02x}{:02x} c={:02x}{:02x} x={:02x} sp={:02x}{:02x} pc={:02x}{:02x}\n", 
            self.cpu.a.value(), 
            (self.cpu.flags.read(Flag::N) as u8),
            (self.cpu.flags.read(Flag::V) as u8),
            (self.cpu.flags.read(Flag::Z) as u8),
            (self.cpu.flags.read(Flag::C) as u8),
            self.cpu.bh.value(), self.cpu.bl.value(),
            self.cpu.ch.value(), self.cpu.cl.value(),
            self.cpu.x.value(),
            self.cpu.sp.high.value(), self.cpu.sp.low.value(),
            self.cpu.pc.high.value(), self.cpu.pc.low.value(),
        ).unwrap();
        write!(f, "Mem: ").unwrap();
        for j in 0..64 { write!(f, "{:02x} ", j).unwrap() };
        write!(f, "\n").unwrap();
        for i in 0..64 {
            let mut do_print = false;
            for j in 0..64 {
                if self.ram[(i*64+j) as usize].value() != 0 {
                    do_print = true;
                    break
                }
            }
            if i >= 4 && !do_print { continue }

            /*write!(f, "{:4x} | ", i*64).unwrap();*/
            write!(f, "{:02x} | ", i).unwrap();
            // if self.ram[i*64 .. i*65].all(|x| x == 0)
            for j in 0..64 {
                if self.cpu.pc.value() == i*64+j {
                    write!(f, "\x08^").unwrap()
                }
                let val = self.ram[(i*64+j) as usize].value();
                if val != 0 {
                    write!(f, "{:02x} ", val).unwrap()
                } else {
                    write!(f, "__ ").unwrap()
                }
            }
            write!(f, "\n").unwrap();
        };
        Ok(())
    }
}
