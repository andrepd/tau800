use crate::basics::*;



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
    &self[address]
  }
}

impl std::ops::IndexMut<Address> for RamState {
  // type Output = Word;

  fn index_mut(&mut self, address: Address) -> &mut Self::Output {
    &mut self[address]
  }
}
