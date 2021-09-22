use crate::prelude::*;

#[derive(Debug)]
pub enum Register {
    A,
    F,
    BH,
    BL,
    CH,
    CL,
    X,
    SP,
}

#[derive(Debug)]
/// An object referencing an operand at a time different from the call
pub struct Timed<T> {
    op: T,
    /// Relative time of operation (w.r.t. call)
    time: Word<sig::Signed>,
}

#[derive(Debug)]
pub enum Operand {
    /// Named register
    Reg(Timed<Register>),
    /// Literal value
    Imm(Word<sig::Unsigned>),
    /// Absolute address (byte order: hi, lo)
    Abs(Timed<Address>),
    /// Indirect access (address is value at that address, hi lo order)
    Ind(Timed<Address>),
}

// You can match on structs in Rust. If you want to check for, for example, ImmReg,
// you can accomplish this with
//
//      match ops {
//          Operands { src: Imm(src), reg: Reg(reg) } => ...,
//          _ => ...
//      }
pub struct Operands {
    src: Operand,
    dest: Operand,
}

type Offset = Word<sig::Signed>;

pub enum Instruction {
    Mov(Operands),

    Add(Operands),
    Sub(Operands),
    Mul(Operands),

    Cmp(Operands),

    Jmp(Operand),

    Beq(Offset),
    Bne(Offset),
    Bpl(Offset),
    Bmi(Offset),

    Cal(Operand),
    Ret,

    Nop,
}

/// Reads and decodes the byte representation of a timed register, i.e., in `Word`s,
/// (Register opcode, Time).
fn read_timed_register(m: &mut Machine) -> Timed<Register> {
    use Register::*;

    let register= match m.read_pc().value() {
        0x0 => A,
        0x1 => F,
        0x2 => BH,
        0x3 => BL,
        0x4 => CH,
        0x5 => CL,
        0x6 => X,
        0x7 => SP,
        _ => unreachable!(),
    };

    let time = m.read_pc().cast_to_signed();
}