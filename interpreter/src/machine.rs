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

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cpu: a={:x} NVZC={}{}{}{} b={:x}{:x} c={:x}{:x} x={:x} sp={:x}{:x} pc={:x}{:x}\n", 
            self.cpu.a.value(), 
            (self.cpu.flags.read(crate::state::Flag::N) as u8),
            (self.cpu.flags.read(crate::state::Flag::V) as u8),
            (self.cpu.flags.read(crate::state::Flag::Z) as u8),
            (self.cpu.flags.read(crate::state::Flag::C) as u8),
            self.cpu.bh.value(), self.cpu.bl.value(),
            self.cpu.ch.value(), self.cpu.cl.value(),
            self.cpu.x.value(),
            self.cpu.sp.high.value(), self.cpu.sp.low.value(),
            self.cpu.pc.high.value(), self.cpu.pc.low.value(),
        );
        write!(f, "Mem:\n");
        for i in 0..64 {
            write!(f, "{:x} | ", i*64);
            for j in 0..64 {
                write!(f, "{:x} ", self.ram[i*64+j].value());
            }
            write!(f, "\n");
        };
        write!(f, "")  // Sem esta linha dá erro lel
    }
}