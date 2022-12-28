pub(super) use more_asserts::*;

pub(super) use crate::word::*;
pub(super) use crate::machine::*;
pub(super) use crate::instruction::{Instruction, Op, Operand, Operands, Register};
pub(super) use crate::universe::{Universe, Mode};
pub(super) use crate::modules::ModuleCollection;

// Debug macros (como é que isto não vem standard)
#[allow(unused_macros)]

#[cfg(debug_assertions)]
macro_rules! dprintln {
    ($( $args:expr ),*) => { eprintln!( $( $args ),* ) }
}

#[cfg(not(debug_assertions))]
macro_rules! dprintln {
    ($( $args:expr ),*) => {()}
}

#[cfg(debug_assertions)]
macro_rules! dprint {
    ($( $args:expr ),*) => { eprint!( $( $args ),* ) }
}

#[cfg(not(debug_assertions))]
macro_rules! dprint {
    ($( $args:expr ),*) => {()}
}

pub(crate) use dprintln;
pub(crate) use dprint;
