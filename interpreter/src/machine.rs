use crate::prelude::*;

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

    /// Read the next value in ram, as indicated by the Program Counter (PC) in
    /// CPU, and increment the PC.
    pub fn read_pc(&mut self) -> UWord {
        let word = self.ram[self.cpu.pc];
        self.cpu.pc.try_increment().expect("Overflowed program counter.");
        word
    }

    /// Write a word at PC and increment the PC.
    pub fn write_pc(&mut self, word: UWord) -> () {
        self.ram[self.cpu.pc] = word;
        self.cpu.pc.try_increment().expect("Overflowed program counter.");
        ()
    }

    /// Read a word from stack and increment the sp. 
    pub fn read_sp(&mut self) -> UWord {
        let word = self.ram[self.cpu.sp];
        self.cpu.sp = self.cpu.sp + IWord::from(1);  // Ugly af
        word
    }

    /// Write a word to stack and decrement the sp. 
    pub fn write_sp(&mut self, word: UWord) -> () {
        self.ram[self.cpu.sp] = word;
        self.cpu.sp = self.cpu.sp + IWord::from(-1);  // Ugly af
    }
}