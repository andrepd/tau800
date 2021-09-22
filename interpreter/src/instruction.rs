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

/// Read a timed register from the RAM.
fn read_timed_register(m: &mut Machine) -> Timed<Register> {
    use Register::*;

    let op = match m.read_pc().value() {
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
    Timed { op, time }
}

/// Read a literal word from the RAM.
fn read_word(m: &mut Machine) -> Word<sig::Unsigned> {
    m.read_pc()
}

/// Read a timed address from the RAM.
fn read_timed_address(m: &mut Machine) -> Timed<Address> {
    let high = m.read_pc();
    let low = m.read_pc();
    let op = Address::from_words(high, low);
    let time = m.read_pc().cast_to_signed();
    Timed { op, time }
}

impl Operand {
    fn decode(m: &mut Machine, mode: Word<sig::Unsigned>) -> Self {
        use Operand::*;
        match mode.value() {
            0x0 => Reg(read_timed_register(m)),
            0x1 => Imm(read_word(m)),
            0x2 => Abs(read_timed_address(m)),
            0x3 => Ind(read_timed_address(m)),
            _ => unreachable!(),
        }
    }
}

impl Operands {
    fn decode(m: &mut Machine, mode: Word<sig::Unsigned>) -> Self {
        use Operand::*;

        match mode.value() {
            0x0 => Operands {
                src: Reg(read_timed_register(m)),
                dest: Reg(read_timed_register(m)),
            },
            0x1 => Operands  {
                src: Imm(read_word(m)),
                dest: Reg(read_timed_register(m)),
            },
            0x2 => Operands  {
                src: Abs(read_timed_address(m)),
                dest: Reg(read_timed_register(m)),
            },
            0x3 => Operands  {
                src: Ind(read_timed_address(m)),
                dest: Reg(read_timed_register(m)),
            },
            0x4 => Operands  {
                src: Reg(read_timed_register(m)),
                dest: Abs(read_timed_address(m)),
            },
            0x5 => Operands  {
                src: Reg(read_timed_register(m)),
                dest: Ind(read_timed_address(m)),
            },
            0x6 => Operands  {
                src: Imm(read_word(m)),
                dest: Abs(read_timed_address(m)),
            },
            0x7 => Operands  {
                src: Imm(read_word(m)),
                dest: Ind(read_timed_address(m)),
            },
            _ => unreachable!(),
        }
    }
}