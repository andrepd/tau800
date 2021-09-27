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

fn write_timed_register(m: &mut Machine, x: &Timed<Register>) -> () {
    use Register::*;
    match x.op {
        A  => m.write_pc(UWord::from(0x0)),
        BH => m.write_pc(UWord::from(0x1)),
        BL => m.write_pc(UWord::from(0x2)),
        CH => m.write_pc(UWord::from(0x3)),
        CL => m.write_pc(UWord::from(0x4)),
        X  => m.write_pc(UWord::from(0x5)),
        SP => m.write_pc(UWord::from(0x7)),
    };
    m.write_pc(x.time.cast_to_unsigned());
}

/// Read a literal word from the RAM at pc.
fn read_word(m: &mut Machine) -> UWord {
    m.read_pc()
}

fn write_word(m: &mut Machine, x: &UWord) -> () {
    m.write_pc(*x)
}

/// Read a timed address from the RAM.
fn read_timed_address(m: &mut Machine) -> Timed<Address> {
    let high = m.read_pc();
    let low = m.read_pc();
    let op = Address::from_words(high, low);
    let time = m.read_pc().cast_to_signed();
    Timed { op, time }
}

fn write_timed_address(m: &mut Machine, x: &Timed<Address>) -> () {
    m.write_pc(x.op.high);
    m.write_pc(x.op.low);
    m.write_pc(x.time.cast_to_unsigned());
}

impl Operand {
    fn decode(m: &mut Machine, mode: UWord) -> Self {
        use Operand::*;
        match mode.value() {
            0x0 => Reg(read_timed_register(m)),
            0x1 => Imm(read_word(m)),
            0x2 => Abs(read_timed_address(m)),
            0x3 => Ind(read_timed_address(m)),
            0x4 => Abx(read_timed_address(m)),
            _ => unreachable!(),
        }
    }

    fn encode(m: &mut Machine, op: &Operand) -> () {
        use Operand::*;
        match op {
            Reg(x) => {
                m.write_pc(UWord::from(0x0));
                write_timed_register(m, &x);
            },
            Imm(x) => {
                m.write_pc(UWord::from(0x1));
                write_word(m, &x);
            },
            Abs(x) => {
                m.write_pc(UWord::from(0x2));
                write_timed_address(m, &x);
            },
            Ind(x) => {
                m.write_pc(UWord::from(0x3));
                write_timed_address(m, &x);
            },
            Abx(x) => {
                m.write_pc(UWord::from(0x4));
                write_timed_address(m, &x);
            },
        }
    }
}

impl Operands {
    fn decode(m: &mut Machine, mode: UWord) -> Self {
        use Operand::*;
        let src_mode = (mode.value() & 0b000111) >> 0;
        let dst_mode = (mode.value() & 0b111000) >> 3;
        let src = Operand::decode(m, UWord::from(src_mode));
        let dst = Operand::decode(m, UWord::from(dst_mode));
        Operands { src, dst }
    }

    fn encode(m: &mut Machine, op: &Operands) -> () {
        use Operand::*;
        let mut mode = 0;
        match op.src {
            Reg(_) => mode |= 0x0 << 0,
            Imm(_) => mode |= 0x1 << 0,
            Abs(_) => mode |= 0x2 << 0,
            Ind(_) => mode |= 0x3 << 0,
            Abx(_) => mode |= 0x4 << 0,
        };
        match op.dst {
            Reg(_) => mode |= 0x0 << 3,
            Imm(_) => mode |= 0x1 << 3,
            Abs(_) => mode |= 0x2 << 3,
            Ind(_) => mode |= 0x3 << 3,
            Abx(_) => mode |= 0x4 << 3,
        };
        m.write_pc(UWord::from(mode));
        match &op.src {
            Reg(x) => write_timed_register(m, &x),
            Imm(x) => write_word(m, &x),
            Abs(x) => write_timed_address(m, &x),
            Ind(x) => write_timed_address(m, &x),
            Abx(x) => write_timed_address(m, &x),
        };
        match &op.dst {
            Reg(x) => write_timed_register(m, &x),
            Imm(x) => write_word(m, &x),
            Abs(x) => write_timed_address(m, &x),
            Ind(x) => write_timed_address(m, &x),
            Abx(x) => write_timed_address(m, &x),
        };
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

    pub fn encode(m: &mut Machine, instruction: &Instruction) -> () {
        match instruction {
            Instruction::Nop => { 
                m.write_pc(UWord::from(0x00));
                m.write_pc(UWord::from(0x00));
            },
            Instruction::Clc => { 
                m.write_pc(UWord::from(0x00));
                m.write_pc(UWord::from(0x01));
            },
            Instruction::Sec => { 
                m.write_pc(UWord::from(0x00));
                m.write_pc(UWord::from(0x02));
            },
            Instruction::Ret => { 
                m.write_pc(UWord::from(0x00));
                m.write_pc(UWord::from(0xff));
            },
            Instruction::Mov(ops) => {
                m.write_pc(UWord::from(0x01));
                Operands::encode(m, ops);
            },
            Instruction::Psh(op) => {
                m.write_pc(UWord::from(0x02));
                Operand::encode(m, op);
            }
            Instruction::Pop(op) => {
                m.write_pc(UWord::from(0x03));
                Operand::encode(m, op);
            }
            Instruction::Add(ops) => {
                m.write_pc(UWord::from(0x04));
                Operands::encode(m, ops);
            },
            Instruction::Sub(ops) => {
                m.write_pc(UWord::from(0x05));
                Operands::encode(m, ops);
            },
            Instruction::Mul(ops) => {
                m.write_pc(UWord::from(0x06));
                Operands::encode(m, ops);
            },
            Instruction::Mus(ops) => {
                m.write_pc(UWord::from(0x07));
                Operands::encode(m, ops);
            },
            Instruction::Div(ops) => {
                m.write_pc(UWord::from(0x08));
                Operands::encode(m, ops);
            },
            Instruction::Dis(ops) => {
                m.write_pc(UWord::from(0x09));
                Operands::encode(m, ops);
            },
            Instruction::Mod(ops) => {
                m.write_pc(UWord::from(0x0a));
                Operands::encode(m, ops);
            },
            Instruction::Mos(ops) => {
                m.write_pc(UWord::from(0x0b));
                Operands::encode(m, ops);
            },
            Instruction::And(ops) => {
                m.write_pc(UWord::from(0x0c));
                Operands::encode(m, ops);
            },
            Instruction::Or (ops) => {
                m.write_pc(UWord::from(0x0d));
                Operands::encode(m, ops);
            },
            Instruction::Xor(ops) => {
                m.write_pc(UWord::from(0x0e));
                Operands::encode(m, ops);
            },
            Instruction::Not(op) => {
                m.write_pc(UWord::from(0x0f));
                Operand::encode(m, op);
            }
            Instruction::Lsl(op) => {
                m.write_pc(UWord::from(0x10));
                Operand::encode(m, op);
            }
            Instruction::Lsr(op) => {
                m.write_pc(UWord::from(0x11));
                Operand::encode(m, op);
            }
            Instruction::Cmp(op) => {
                m.write_pc(UWord::from(0x12));
                Operand::encode(m, op);
            }
            Instruction::Bit(op) => {
                m.write_pc(UWord::from(0x13));
                Operand::encode(m, op);
            }
            Instruction::Jmp(_) => {
                m.write_pc(UWord::from(0x14));
                unimplemented!();
            }
            Instruction::Bcc(_) => {
                m.write_pc(UWord::from(0x15));
                unimplemented!();
            }
            Instruction::Bcs(_) => {
                m.write_pc(UWord::from(0x16));
                unimplemented!();
            }
            Instruction::Bne(_) => {
                m.write_pc(UWord::from(0x17));
                unimplemented!();
            }
            Instruction::Beq(_) => {
                m.write_pc(UWord::from(0x18));
                unimplemented!();
            }
            Instruction::Bpl(_) => {
                m.write_pc(UWord::from(0x19));
                unimplemented!();
            }
            Instruction::Bmi(_) => {
                m.write_pc(UWord::from(0x1a));
                unimplemented!();
            }
            Instruction::Cal(_) => {
                m.write_pc(UWord::from(0x1d));
                unimplemented!();
            }
        }
    }
}
