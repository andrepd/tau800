use crate::prelude::*;

#[derive(Debug)]
pub struct Timed<T> {
    op: T,
    /// Relative time of operation (w.r.t. call)
    time: Word<sig::Signed>,
}

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

#[derive(Debug)]
pub enum Operands {
    RegReg {
        src: Timed<Register>,
        dst: Timed<Register>,
    },
    ImmReg {
        src: Word<sig::Unsigned>,
        dst: Timed<Register>,
    },
    AbsReg {
        src: Timed<Address>,
        dst: Timed<Register>,
    },
    IndReg {
        src: Timed<Address>,
        dst: Timed<Register>,
    },
    RegAbs {
        src: Timed<Register>,
        dst: Timed<Address>,
    },
    RegInd {
        src: Timed<Register>,
        dst: Timed<Address>,
    },
    ImmAbs {
        src: Word<sig::Unsigned>,
        dst: Timed<Address>,
    },
    ImmInd {
        src: Word<sig::Unsigned>,
        dst: Timed<Address>,
    },
}

#[derive(Debug)]
pub struct Offset(Word<sig::Signed>);

#[derive(Debug)]
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

fn decode_reg(m: &mut Machine) -> Timed<Register> {
    let op = match m.read_pc().value() {
        0x0 => Register::A,
        0x1 => Register::F,
        0x2 => Register::BH,
        0x3 => Register::BL,
        0x4 => Register::CH,
        0x5 => Register::CL,
        0x6 => Register::X,
        0x7 => Register::SP,
        _ => unreachable!(),
    };
    let time = Word::from(m.read_pc().value() as i8);
    Timed { op, time }
}

fn decode_word(m: &mut Machine) -> Word<sig::Unsigned> {
    m.read_pc()
}

fn decode_addr(m: &mut Machine) -> Timed<Address> {
    let high: Word = m.read_pc();
    let low: Word = m.read_pc();
    let op: Address = Address::from_words(high, low);
    let time = Word::<sig::Signed>::from(m.read_pc());
    Timed { op, time }
}

impl Operand {
    fn decode(m: &mut Machine, mode: Word<sig::Unsigned>) -> Self {
        use Operand::*;
        match mode {
            0x0 => Reg(decode_reg(m)),
            0x1 => Imm(decode_word(m)),
            0x2 => Abs(decode_addr(m)),
            0x3 => Ind(decode_addr(m)),
            _ => unreachable!(),
        }
    }
}

impl Operands {
    fn decode(m: &mut Machine, mode: Word<sig::Unsigned>) -> Self {
        use Operands::*;
        match mode {
            0x0 => RegReg {
                src: decode_reg(m),
                dst: decode_reg(m),
            },
            0x1 => ImmReg {
                src: decode_word(m),
                dst: decode_reg(m),
            },
            0x2 => AbsReg {
                src: decode_addr(m),
                dst: decode_reg(m),
            },
            0x3 => IndReg {
                src: decode_addr(m),
                dst: decode_reg(m),
            },
            0x4 => RegAbs {
                src: decode_reg(m),
                dst: decode_addr(m),
            },
            0x5 => RegInd {
                src: decode_reg(m),
                dst: decode_addr(m),
            },
            0x6 => ImmAbs {
                src: decode_word(m),
                dst: decode_addr(m),
            },
            0x7 => ImmInd {
                src: decode_word(m),
                dst: decode_addr(m),
            },
            _ => unreachable!(),
        }
    }
}

impl Offset {
    fn decode(m: &mut Machine) -> Self {
        let word = Word::<sig::Signed>::from(m.read_pc() as i8);
        Offset(word as Word)
    }
}

impl Instruction {
    /// Decode the next instruction at memory[pc].
    /// Encoding scheme: 5 bits for instruction, 3 bits for addressing mode
    fn decode(m: &mut Machine) -> Self {
        // One-word instructions
        /*let word = memory[*pc];
        let (opcode, addressing) = (word >> 3, word & ((1<<3) - 1));*/
        // Two-word instructions
        let opcode = m.read_pc();
        let addressing = m.read_pc();
        match opcode {
            0x001 => Instruction::Mov(Operands::decode(m, addressing)),
            0x002 => Instruction::Add(Operands::decode(m, addressing)),
            0x003 => Instruction::Sub(Operands::decode(m, addressing)),
            0x004 => Instruction::Cmp(Operands::decode(m, addressing)),
            0x005 => Instruction::Jmp(Operand::decode(m, addressing)),
            0x006 => Instruction::Beq(Offset::decode(m)),
            0x007 => Instruction::Bne(Offset::decode(m)),
            0x008 => Instruction::Bpl(Offset::decode(m)),
            0x009 => Instruction::Bmi(Offset::decode(m)),
            0x00a => Instruction::Cal(Operand::decode(m, addressing)),
            0x00b => Instruction::Ret,
            0x00c => Instruction::Nop,
            _ => panic!("Illegal opcode: {:?}", opcode),
        }
    }
}
