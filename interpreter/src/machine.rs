use crate::prelude::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlagWord (uWord);

impl FlagWord {
    pub fn read(&self, flag: Flag) -> bool {
        let mask = u8::from(flag);
        u8::from(self.0) & mask != 0
    }

    pub fn write(&mut self, flag: Flag, value: bool) -> () {
        let mask = u8::from(flag);
        let new = if value {
            u8::from(self.0) | mask
        } else {
            u8::from(self.0) & !mask
        };
        self.0 = new.try_into().unwrap()
    }
}

pub type Address = uLong;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
pub struct Cpu {
    pub a: uWord,
    pub flags: FlagWord,
    pub bh: uWord,
    pub bl: uWord,
    pub ch: uWord,
    pub cl: uWord,
    pub x: uWord,
    pub sp: Address,
    pub pc: Address,
}

impl Default for Cpu {
    fn default() -> Self {
        let pc = Address::from_hi_lo(uWord::lit(0x02), uWord::lit(0x00));
        let sp = Address::from_hi_lo(uWord::lit(0x01), uWord::lit(0x3f));
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

const RAM_SIZE: usize = Address::MAX.value() as usize + 1;

#[derive(Clone, Eq)]
pub struct Ram(pub Vec<uWord>);

// To save space, RAM is represented by a vector with size ≤ RAM_SIZE. Memory  
// past the size of the vector is assumed to be 0.
impl PartialEq for Ram {
    fn eq(&self, other: &Self) -> bool {
        let (min, max) = if self.0.len() < other.0.len() {
            (&self.0, &other.0)
        } else {
            (&other.0, &self.0)
        };
        min[..min.len()] == max[..min.len()]
        && max[min.len()..max.len()].iter().all(|x| x == &uWord::ZERO)
    }
}

impl std::fmt::Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ram (omitted)").finish()
    }
}

impl std::ops::Index<usize> for Ram {
    type Output = uWord;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert_lt!(index, RAM_SIZE);
        if index >= self.0.len() { return &uWord::ZERO };
        &self.0[index]
    }
}

impl std::ops::Index<Address> for Ram {
    type Output = uWord;

    fn index(&self, index: Address) -> &Self::Output {
        &self[usize::from(index)]
    }
}

impl std::ops::IndexMut<usize> for Ram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert_lt!(index, RAM_SIZE);
        if index >= self.0.len() { self.0.resize(index+1, uWord::ZERO) };
        &mut self.0[index]
    }
}

impl std::ops::IndexMut<Address> for Ram {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        &mut self[usize::from(index)]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Ram {
    type Output = [uWord];

    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        debug_assert_le!(range.end, self.0.len()); //TODO
        &self.0[range]
    }
}

impl std::ops::Index<std::ops::Range<Address>> for Ram {
    type Output = [uWord];

    fn index(&self, range: std::ops::Range<Address>) -> &Self::Output {
        &self[usize::from(range.start)..usize::from(range.end)]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Ram {
    fn index_mut(&mut self, range: std::ops::Range<usize>) -> &mut Self::Output {
        debug_assert_le!(range.end, RAM_SIZE);
        if range.end >= self.0.len() { self.0.resize(range.end, uWord::ZERO) };
        &mut self.0[range]
    }
}

impl std::ops::IndexMut<std::ops::Range<Address>> for Ram {
    fn index_mut(&mut self, range: std::ops::Range<Address>) -> &mut Self::Output {
        &mut self[usize::from(range.start)..usize::from(range.end)]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self(vec![uWord::ZERO; 64*2])
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

    fn increment_pc(&mut self) {
        self.cpu.pc = self.cpu.pc + 1_i8
    }

    /// Read the next value in ram, as indicated by the Program Counter (PC) in
    /// CPU, and increment the PC.
    pub fn read_pc(&mut self) -> uWord {
        let word = self.ram[self.cpu.pc];
        self.increment_pc();
        word
    }

    /// Write a word at PC and increment the PC.
    pub fn write_pc(&mut self, word: uWord) {
        self.ram[self.cpu.pc] = word;
        self.increment_pc();

    }

    /// Read a word from stack and increment the sp. 
    pub fn read_sp(&mut self) -> uWord {
        self.cpu.sp = self.cpu.sp + 1_i8;
        let word = self.ram[self.cpu.sp];
        word
    }

    /// Write a word to stack and decrement the sp. 
    pub fn write_sp(&mut self, word: uWord) {
        self.ram[self.cpu.sp] = word;
        self.cpu.sp = self.cpu.sp + (-1_i8);
    }

    pub fn reset_cpu(&mut self) {
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
            self.cpu.sp.hi().value(), self.cpu.sp.lo().value(),
            self.cpu.pc.hi().value(), self.cpu.pc.lo().value(),
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
