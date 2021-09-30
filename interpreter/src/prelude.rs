pub(crate) use crate::word::*;
pub(crate) use crate::state::*;
pub(crate) use crate::machine::Machine;
pub(crate) use crate::instruction::{Instruction, Op, Operand, Operands, Register};
pub(crate) use crate::universe::Universe;

/// Tou sempre a ter de usar isto
pub fn div_rem<
  T: Copy + std::ops::Div<Output = T> + std::ops::Rem<Output = T>
>(a: T, b: T) -> (T, T) {
  (a / b, a % b)
}
