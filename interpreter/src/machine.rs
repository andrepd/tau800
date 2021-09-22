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

    pub fn read_pc(&mut self) -> Word<sig::Unsigned> {
        let word = self.ram[self.cpu.pc.value() as usize];
        self.cpu.pc.increment();
        word
    }
}