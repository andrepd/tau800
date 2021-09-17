use crate::prelude::*;

#[derive(Debug)]
pub struct MachineState {
  cpu: CpuState,
  ram: RamState,
}

impl MachineState {
  pub fn new() -> Self {
    MachineState{ cpu: CpuState::new(), ram: RamState::new() }
  }
}
