use crate::prelude::*;

pub struct Machine {
    ram: Ram,
    cpu: Cpu,
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
    pub fn read_pc(&mut self) -> LongWord<sig::Unsigned> {
        let word = self.ram[self.cpu.pc.value() as usize];
        self.cpu.pc.increment();
        word
    }
}