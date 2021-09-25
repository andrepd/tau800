use crate::prelude::*;

#[derive(Debug)]
pub enum Register {
    A,
    // F,
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
    pub op: T,
    /// Relative time of operation (w.r.t. call)
    pub time: IWord,
}

#[derive(Debug)]
pub enum Operand {
    /// Named register
    Reg(Timed<Register>),
    /// Literal one-word value
    Imm(UWord),
    /*/// Literal two-word value
    Iml(LongUWord),*/
    /// Absolute address (byte order: lo hi)
    Abs(Timed<Address>),
    /// Absolute address + X register
    Abx(Timed<Address>),
    /// Indirect access (address is value at that address, lo hi order)
    Ind(Timed<Address>),
    /*/// Indirect access (address is value at that register, lo hi order)
    Inr(Timed<Register>),*/
}

// You can match on structs in Rust. If you want to check for, for example, ImmReg,
// you can accomplish this with
//
//      match ops {
//          Operands { src: Imm(src), reg: Reg(reg) } => ...,
//          _ => ...
//      }
pub struct Operands {
    pub src: Operand,
    pub dst: Operand,
}

pub type Offset = IWord;

pub enum Instruction {
    Mov(Operands),
    Psh(Operand),
    Pop(Operand),

    Add(Operands),
    Sub(Operands),
    Mul(Operands),
    Mus(Operands),
    Div(Operands),
    Dis(Operands),
    Mod(Operands),
    Mos(Operands),
    
    And(Operands),
    Or (Operands),
    Xor(Operands),
    Not(Operand),

    Lsl(Operand),
    Lsr(Operand),

    Cmp(Operand),
    Bit(Operand),

    Jmp(Address),

    Bcc(Offset),
    Bcs(Offset),
    Bne(Offset),
    Beq(Offset),
    Bpl(Offset),
    Bmi(Offset),

    Clc,
    Sec,

    Cal(Address),
    Ret,

    Nop,
}

/// Read a timed register from the RAM.
fn read_timed_register(m: &mut Machine) -> Timed<Register> {
    use Register::*;

    let op = match m.read_pc().value() {
        0x0 => A,
        // 0x1 => F,
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
fn read_word(m: &mut Machine) -> UWord {
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
    fn decode(m: &mut Machine, mode: UWord) -> Self {
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
    fn decode(m: &mut Machine, mode: UWord) -> Self {
        use Operand::*;
        match mode.value() {
            0x0 => Operands {
                src: Reg(read_timed_register(m)),
                dst: Reg(read_timed_register(m)),
            },
            0x1 => Operands {
                src: Imm(read_word(m)),
                dst: Reg(read_timed_register(m)),
            },
            0x2 => Operands {
                src: Abs(read_timed_address(m)),
                dst: Reg(read_timed_register(m)),
            },
            0x3 => Operands {
                src: Ind(read_timed_address(m)),
                dst: Reg(read_timed_register(m)),
            },
            0x4 => Operands {
                src: Reg(read_timed_register(m)),
                dst: Abs(read_timed_address(m)),
            },
            0x5 => Operands {
                src: Reg(read_timed_register(m)),
                dst: Ind(read_timed_address(m)),
            },
            0x6 => Operands {
                src: Imm(read_word(m)),
                dst: Abs(read_timed_address(m)),
            },
            0x7 => Operands {
                src: Imm(read_word(m)),
                dst: Ind(read_timed_address(m)),
            },
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    pub fn decode(m: &mut Machine) -> Self {
        let opcode = m.read_pc();
        let mode   = m.read_pc();
        match opcode.value() {
            0x00 => match mode.value() {
                0x00 => Instruction::Nop,
                0x01 => Instruction::Clc,
                0x02 => Instruction::Sec,
                0xff => Instruction::Ret,
                _ => unreachable!(),
            },
            0x01 => Instruction::Mov(Operands::decode(m, mode)),
            0x02 => Instruction::Psh(Operand::decode(m, mode)),
            0x03 => Instruction::Pop(Operand::decode(m, mode)),
            0x04 => Instruction::Add(Operands::decode(m, mode)),
            0x05 => Instruction::Sub(Operands::decode(m, mode)),
            0x06 => Instruction::Mul(Operands::decode(m, mode)),
            0x07 => Instruction::Mus(Operands::decode(m, mode)),
            0x08 => Instruction::Div(Operands::decode(m, mode)),
            0x09 => Instruction::Dis(Operands::decode(m, mode)),
            0x0a => Instruction::Mod(Operands::decode(m, mode)),
            0x0b => Instruction::Mos(Operands::decode(m, mode)),
            0x0c => Instruction::And(Operands::decode(m, mode)),
            0x0d => Instruction::Or (Operands::decode(m, mode)),
            0x0e => Instruction::Xor(Operands::decode(m, mode)),
            0x0f => Instruction::Not(Operand::decode(m, mode)),
            0x10 => Instruction::Lsl(Operand::decode(m, mode)),
            0x11 => Instruction::Lsr(Operand::decode(m, mode)),
            0x12 => Instruction::Cmp(Operand::decode(m, mode)),
            0x13 => Instruction::Bit(Operand::decode(m, mode)),
            0x14 => Instruction::Jmp(unimplemented!()),
            0x15 => Instruction::Bcc(unimplemented!()),
            0x16 => Instruction::Bcs(unimplemented!()),
            0x17 => Instruction::Bne(unimplemented!()),
            0x18 => Instruction::Beq(unimplemented!()),
            0x19 => Instruction::Bpl(unimplemented!()),
            0x1a => Instruction::Bmi(unimplemented!()),
            0x1d => Instruction::Cal(unimplemented!()),
            _ => unreachable!(),
        }
    }
}
