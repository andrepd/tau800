use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Register {
    A,
    // F,
    BH,
    BL,
    CH,
    CL,
    X,
    // SP,
}

#[derive(Debug, Clone)]
/// An object referencing an operand at a time different from the call
pub struct Timed<T> {
    pub op: T,
    /// Relative time of operation (w.r.t. call)
    pub time: IWord,
}

impl<T> Timed<T> {
    pub fn new(op: T, time: IWord) -> Self {
        Timed { op, time }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    /// Named register
    Reg(Register),
    /// Literal one-word value
    Imm(UWord),
    /*/// Literal two-word value
    Iml(LongUWord),*/
    /// Absolute address (byte order: lo hi)
    Abs(Address),
    /// Absolute address + X register
    Abx(Address),
    /// Indirect access (address is value at that address, lo hi order)
    Ind(Address),
    /*/// Indirect access (address is value at that register, lo hi order)
    Inr(Timed<Register>),*/
}

pub type Operand = Timed<Op>;

// You can match on structs in Rust. If you want to check for, for example, ImmReg,
// you can accomplish this with
//
//      match ops {
//          Operands { src: Imm(src), reg: Reg(reg) } => ...,
//          _ => ...
//      }
#[derive(Debug, Clone)]
pub struct Operands {
    pub src: Operand,
    pub dst: Operand,
}

impl Operands {
    pub fn new(src: Operand, dst: Operand) -> Self {
        Operands { src, dst }
    }
}

pub type Offset = IWord;

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Operands),
    Psh(Operand),
    Pop(Operand),

    Add(Operands),
    Sub(Operands),
    Mul(Operands),
    Muh(Operands),
    Mus(Operands),
    Div(Operands),
    /*Dis(Operands),*/
    Mod(Operands),
    /*Mos(Operands),*/

    And(Operands),
    Or(Operands),
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
fn read_register(m: &mut Machine) -> Register {
    use Register::*;
    match m.read_pc().value() {
        0x0 => A,
        // 0x1 => F,
        0x2 => BH,
        0x3 => BL,
        0x4 => CH,
        0x5 => CL,
        0x6 => X,
        // 0x7 => SP,
        _ => unreachable!(),
    }
}

/// Read a literal word from the RAM at pc.
fn read_word(m: &mut Machine) -> UWord {
    m.read_pc()
}

/// Read a timed address from the RAM.
fn read_address(m: &mut Machine) -> Address {
    let low = m.read_pc();
    let high = m.read_pc();
    Address { low, high }
}

fn read_time(m: &mut Machine) -> IWord {
    m.read_pc().cast_to_signed()
}

fn write_register(m: &mut Machine, x: &Register) -> () {
    use Op::*;
    use Register::*;
    match x {
        A => m.write_pc(UWord::from(0x0)),
        // F => m.write_pc(UWord::from(0x1)),
        BH => m.write_pc(UWord::from(0x2)),
        BL => m.write_pc(UWord::from(0x3)),
        CH => m.write_pc(UWord::from(0x4)),
        CL => m.write_pc(UWord::from(0x5)),
        X => m.write_pc(UWord::from(0x6)),
        // SP => m.write_pc(UWord::from(0x7)),
    }
}

fn write_word(m: &mut Machine, x: &UWord) -> () {
    m.write_pc(*x)
}

fn write_address(m: &mut Machine, x: &Address) -> () {
    m.write_pc(x.low);
    m.write_pc(x.high);
}

fn write_time(m: &mut Machine, x: &IWord) -> () {
    m.write_pc(x.cast_to_unsigned());
}

impl Operand {
    fn decode(m: &mut Machine, mode: UWord) -> Self {
        use Op::*;
        match mode.value() {
            0x0 => Timed { op: Reg(read_register(m)), time: read_time(m) },
            0x1 => Timed { op: Imm(read_word(m)),     time: 0.into()     },
            0x2 => Timed { op: Abs(read_address(m)),  time: read_time(m) },
            0x3 => Timed { op: Ind(read_address(m)),  time: read_time(m) },
            0x4 => Timed { op: Abx(read_address(m)),  time: read_time(m) },
            _ => unreachable!(),
        }
    }

    fn encode(m: &mut Machine, x: &Operand) -> () {
        use Op::*;
        match &x.op {
            Reg(y) => {
                m.write_pc(UWord::from(0x0));
                write_register(m, &y);
                write_time(m, &x.time);
            }
            Imm(y) => {
                m.write_pc(UWord::from(0x1));
                write_word(m, &y);
            }
            Abs(y) => {
                m.write_pc(UWord::from(0x2));
                write_address(m, &y);
                write_time(m, &x.time);
            }
            Ind(y) => {
                m.write_pc(UWord::from(0x3));
                write_address(m, &y);
                write_time(m, &x.time);
            }
            Abx(y) => {
                m.write_pc(UWord::from(0x4));
                write_address(m, &y);
                write_time(m, &x.time);
            }
        }
    }
}

impl Operands {
    fn decode(m: &mut Machine, mode: UWord) -> Self {
        let src_mode = (mode.value() & 0b000111) >> 0;
        let dst_mode = (mode.value() & 0b111000) >> 3;
        let src = Operand::decode(m, UWord::from(src_mode));
        let dst = Operand::decode(m, UWord::from(dst_mode));
        Operands { src, dst }
    }

    fn encode(m: &mut Machine, x: &Operands) -> () {
        use Op::*;
        let mut mode = 0;
        match x.src.op {
            Reg(_) => mode |= 0x0 << 0,
            Imm(_) => mode |= 0x1 << 0,
            Abs(_) => mode |= 0x2 << 0,
            Ind(_) => mode |= 0x3 << 0,
            Abx(_) => mode |= 0x4 << 0,
        };
        match x.dst.op {
            Reg(_) => mode |= 0x0 << 3,
            Imm(_) => mode |= 0x1 << 3,
            Abs(_) => mode |= 0x2 << 3,
            Ind(_) => mode |= 0x3 << 3,
            Abx(_) => mode |= 0x4 << 3,
        };
        m.write_pc(UWord::from(mode));
        match &x.src.op {
            Reg(y) => {write_register(m, &y); write_time(m, &x.src.time)},
            Imm(y) => write_word(m, &y),
            Abs(y) => {write_address(m, &y);  write_time(m, &x.src.time)},
            Ind(y) => {write_address(m, &y);  write_time(m, &x.src.time)},
            Abx(y) => {write_address(m, &y);  write_time(m, &x.src.time)},
        };
        match &x.dst.op {
            Reg(y) => {write_register(m, &y); write_time(m, &x.dst.time)},
            Imm(y) => write_word(m, &y),
            Abs(y) => {write_address(m, &y);  write_time(m, &x.dst.time)},
            Ind(y) => {write_address(m, &y);  write_time(m, &x.dst.time)},
            Abx(y) => {write_address(m, &y);  write_time(m, &x.dst.time)},
        };
    }
}

fn address_decode(m: &mut Machine) -> Address {
    let low = m.read_pc();
    let high = m.read_pc();
    Address { low, high }
}

fn offset_decode(m: &mut Machine) -> Offset {
    m.read_pc().cast_to_signed()
}

fn address_encode(m: &mut Machine, x: &Address) {
    m.write_pc(x.low);
    m.write_pc(x.high);
}

fn offset_encode(m: &mut Machine, x: &Offset) {
    m.write_pc(x.cast_to_unsigned())
}

impl Instruction {
    pub fn decode(m: &mut Machine) -> Self {
        use Instruction::*;
        let opcode = m.read_pc();
        /*let mode = m.read_pc();*/
        match opcode.value() {
            0x00 => Nop, 
            0x01 => { let mode = m.read_pc(); Mov(Operands::decode(m, mode)) },
            0x02 => { let mode = m.read_pc(); Psh(Operand::decode(m, mode))  },
            0x03 => { let mode = m.read_pc(); Pop(Operand::decode(m, mode))  },
            0x04 => { let mode = m.read_pc(); Add(Operands::decode(m, mode)) },
            0x05 => { let mode = m.read_pc(); Sub(Operands::decode(m, mode)) },
            0x06 => { let mode = m.read_pc(); Mul(Operands::decode(m, mode)) },
            0x20 => { let mode = m.read_pc(); Muh(Operands::decode(m, mode)) },
            0x07 => { let mode = m.read_pc(); Mus(Operands::decode(m, mode)) },
            0x08 => { let mode = m.read_pc(); Div(Operands::decode(m, mode)) },
            0x0a => { let mode = m.read_pc(); Mod(Operands::decode(m, mode)) },
            0x0c => { let mode = m.read_pc(); And(Operands::decode(m, mode)) },
            0x0d => { let mode = m.read_pc(); Or (Operands::decode(m, mode)) },
            0x0e => { let mode = m.read_pc(); Xor(Operands::decode(m, mode)) },
            0x0f => { let mode = m.read_pc(); Not(Operand::decode(m, mode))  },
            0x10 => { let mode = m.read_pc(); Lsl(Operand::decode(m, mode))  },
            0x11 => { let mode = m.read_pc(); Lsr(Operand::decode(m, mode))  },
            0x12 => { let mode = m.read_pc(); Cmp(Operand::decode(m, mode))  },
            0x13 => { let mode = m.read_pc(); Bit(Operand::decode(m, mode))  },
            0x14 => Jmp(address_decode(m)),
            0x15 => Bcc(offset_decode(m)),
            0x16 => Bcs(offset_decode(m)),
            0x17 => Bne(offset_decode(m)),
            0x18 => Beq(offset_decode(m)),
            0x19 => Bpl(offset_decode(m)),
            0x1a => Bmi(offset_decode(m)),
            0x1d => Cal(address_decode(m)),
            0x21 => Clc,
            0x22 => Sec,
            0x3f => Ret,
            _ => unreachable!(),
        }
    }

    pub fn encode(m: &mut Machine, instruction: &Instruction) -> () {
        use Instruction::*;
        match instruction {
            Nop => {
                m.write_pc(UWord::from(0x00));
                /*m.write_pc(UWord::from(0x00));*/
            }
            Clc => {
                m.write_pc(UWord::from(0x21));
            }
            Sec => {
                m.write_pc(UWord::from(0x22));
            }
            Ret => {
                m.write_pc(UWord::from(0x3f));
            }
            Mov(ops) => {
                m.write_pc(UWord::from(0x01));
                Operands::encode(m, ops);
            }
            Psh(op) => {
                m.write_pc(UWord::from(0x02));
                Operand::encode(m, op);
            }
            Pop(op) => {
                m.write_pc(UWord::from(0x03));
                Operand::encode(m, op);
            }
            Add(ops) => {
                m.write_pc(UWord::from(0x04));
                Operands::encode(m, ops);
            }
            Sub(ops) => {
                m.write_pc(UWord::from(0x05));
                Operands::encode(m, ops);
            }
            Mul(ops) => {
                m.write_pc(UWord::from(0x06));
                Operands::encode(m, ops);
            }
            Muh(ops) => {
                m.write_pc(UWord::from(0x20));
                Operands::encode(m, ops);
            }
            Mus(ops) => {
                m.write_pc(UWord::from(0x07));
                Operands::encode(m, ops);
            }
            Div(ops) => {
                m.write_pc(UWord::from(0x08));
                Operands::encode(m, ops);
            }
            Mod(ops) => {
                m.write_pc(UWord::from(0x0a));
                Operands::encode(m, ops);
            }
            And(ops) => {
                m.write_pc(UWord::from(0x0c));
                Operands::encode(m, ops);
            }
            Or(ops) => {
                m.write_pc(UWord::from(0x0d));
                Operands::encode(m, ops);
            }
            Xor(ops) => {
                m.write_pc(UWord::from(0x0e));
                Operands::encode(m, ops);
            }
            Not(op) => {
                m.write_pc(UWord::from(0x0f));
                Operand::encode(m, op);
            }
            Lsl(op) => {
                m.write_pc(UWord::from(0x10));
                Operand::encode(m, op);
            }
            Lsr(op) => {
                m.write_pc(UWord::from(0x11));
                Operand::encode(m, op);
            }
            Cmp(op) => {
                m.write_pc(UWord::from(0x12));
                Operand::encode(m, op);
            }
            Bit(op) => {
                m.write_pc(UWord::from(0x13));
                Operand::encode(m, op);
            }
            Jmp(x) => {
                m.write_pc(UWord::from(0x14));
                address_encode(m, x);
            }
            Bcc(x) => {
                m.write_pc(UWord::from(0x15));
                offset_encode(m, x);
            }
            Bcs(x) => {
                m.write_pc(UWord::from(0x16));
                offset_encode(m, x);
            }
            Bne(x) => {
                m.write_pc(UWord::from(0x17));
                offset_encode(m, x);
            }
            Beq(x) => {
                m.write_pc(UWord::from(0x18));
                offset_encode(m, x);
            }
            Bpl(x) => {
                m.write_pc(UWord::from(0x19));
                offset_encode(m, x);
            }
            Bmi(x) => {
                m.write_pc(UWord::from(0x1a));
                offset_encode(m, x);
            }
            Cal(x) => {
                m.write_pc(UWord::from(0x1d));
                address_encode(m, x);
            }
        }
    }
}
