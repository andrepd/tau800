use crate::prelude::*;



#[derive(Debug)]
pub struct MachineState {
  pub cpu: CpuState,  // Preciso por pub em tudo o que é merda, whyyyy
  pub ram: RamState,
}

impl MachineState {
  pub fn new() -> Self {
    MachineState{ cpu: CpuState::new(), ram: RamState::new() }
  }

  /// Read word at pc and increment pc
  pub fn read_pc(&mut self) -> Word {
    let word = self.ram[self.cpu.pc];
    self.cpu.pc += 1;
    word
  }
}


