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
        write!(f, "Cpu: a={:02x} NVZC={}{}{}{} b={:02x}{:02x} c={:02x}{:02x} x={:02x} sp={:02x}{:02x} pc={:02x}{:02x}\n", 
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
        ).unwrap();
        /*write!(f, "Mem:\n").unwrap();*/
        write!(f, "Mem: ").unwrap();
        for j in 0..64 { write!(f, "{:02x} ", j).unwrap() };
        write!(f, "\n");
        for i in 0..64 {
            /*write!(f, "{:4x} | ", i*64).unwrap();*/
            write!(f, "{:02x} | ", i).unwrap();
            for j in 0..64 {
                if self.cpu.pc.value() == i*64+j {
                    write!(f, "\x08^").unwrap()
                }
                let val = self.ram[(i*64+j) as usize].value();
                if val != 0 {
                    write!(f, "{:02x} ", val).unwrap()
                } else {
                    write!(f, "   ").unwrap()
                }
            }
            write!(f, "\n").unwrap();
        };
        Ok(())
    }
}