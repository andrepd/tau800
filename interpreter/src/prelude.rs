pub(super) use super::word::*;
pub(super) use super::state::*;
pub(super) use super::machine::Machine;
pub(super) use super::instruction::{Instruction, Op, Operand, Operands, Register};
pub(super) use super::universe::{Universe, Mode};

/// Tou sempre a ter de usar isto
pub fn div_rem<
  T: Copy + std::ops::Div<Output = T> + std::ops::Rem<Output = T>
>(a: T, b: T) -> (T, T) {
  (a / b, a % b)
}
