use crate::prelude::*;



// pub type RamState = std::vec::Vec;
#[derive(Debug)]
pub struct RamState (
  Vec<Word>
);

impl RamState {
  pub fn new() -> Self {
    RamState(vec![0; RAM_SIZE])  // Rust é feio como a merda
  }
}

impl std::ops::Index<Address> for RamState {
  type Output = Word;

  fn index(&self, address: Address) -> &Self::Output {
    debug_assert!((address as usize) < RAM_SIZE);
    let value = &self[address];
    debug_assert!((*value as usize) < MAX_WORD);
    value
  }
}

impl std::ops::IndexMut<Address> for RamState {
  // type Output = Word;

  fn index_mut(&mut self, address: Address) -> &mut Self::Output {
    debug_assert!((address as usize) < RAM_SIZE);
    let value = &mut self[address];
    debug_assert!((*value as usize) < MAX_WORD);
    value
  }
}
