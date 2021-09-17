// use crate::cpu_state::CpuState;
use crate::cpu_state::{self, CpuState};
use crate::ram_state::{self, RamState};

#[derive(Debug)]
pub struct MachineState {
  cpu: cpu_state::CpuState,
  ram: ram_state::RamState,
}

impl MachineState {
  pub fn new() -> Self {
    MachineState{ cpu: CpuState::new(), ram: RamState::new() }
  }
}
