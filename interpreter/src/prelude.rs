pub(super) use more_asserts::*;

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



// Debug macros (como é que isto não vem standard)
#[allow(unused_macros)]

#[cfg(debug_assertions)]
macro_rules! dprintln {
    ($( $args:expr ),*) => { eprintln!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! dprintln {
    ($( $args:expr ),*) => {()}
}

#[cfg(debug_assertions)]
macro_rules! dprint {
    ($( $args:expr ),*) => { eprint!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! dprint {
    ($( $args:expr ),*) => {()}
}

pub(crate) use dprintln;
pub(crate) use dprint;
