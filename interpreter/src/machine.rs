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
    pub fn read_pc(&mut self) -> Word<sig::Unsigned> {
        let word = self.ram[self.cpu.pc.value() as usize];
        self.cpu.pc.try_increment().expect("Overflowed program counter.");
        word
    }
}