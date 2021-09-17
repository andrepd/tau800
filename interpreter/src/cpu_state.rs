use crate::prelude::*;

#[derive(Debug)]
pub struct Flags (
  u8
);

impl Flags {
  fn new() -> Self {
    Flags(0)
  }
}

/*impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}*/



#[derive(Debug)]
pub struct CpuState {
  a: Word,
  b: Word,
  c: Word,
  d: Word,
  sp: Word,
  pc: Address,
  flags: Flags,
}

impl CpuState {
  pub fn new() -> Self { 
    Self { a: 0, b: 0, c: 0, d: 0, sp: 0, pc: 0, flags: Flags::new() } 
  }
}
