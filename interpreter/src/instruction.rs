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
    // SP,
}

#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(Debug)]
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
        // 0x7 => SP,
        _ => unreachable!(),
    };

    let time = m.read_pc().cast_to_signed();
    Timed { op, time }
}

fn write_timed_register(m: &mut Machine, x: &Timed<Register>) -> () {
    use Register::*;
    match x.op {
        A => m.write_pc(UWord::from(0x0)),
        // F => m.write_pc(UWord::from(0x1)),
        BH => m.write_pc(UWord::from(0x2)),
        BL => m.write_pc(UWord::from(0x3)),
        CH => m.write_pc(UWord::from(0x4)),
        CL => m.write_pc(UWord::from(0x5)),
        X => m.write_pc(UWord::from(0x6)),
        // SP => m.write_pc(UWord::from(0x7)),
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
    let low = m.read_pc();
    let high = m.read_pc();
    let op = Address { low, high };
    let time = m.read_pc().cast_to_signed();
    Timed { op, time }
}

fn write_timed_address(m: &mut Machine, x: &Timed<Address>) -> () {
    m.write_pc(x.op.low);
    m.write_pc(x.op.high);
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
            }
            Imm(x) => {
                m.write_pc(UWord::from(0x1));
                write_word(m, &x);
            }
            Abs(x) => {
                m.write_pc(UWord::from(0x2));
                write_timed_address(m, &x);
            }
            Ind(x) => {
                m.write_pc(UWord::from(0x3));
                write_timed_address(m, &x);
            }
            Abx(x) => {
                m.write_pc(UWord::from(0x4));
                write_timed_address(m, &x);
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
    m.write_pc(dbg!(x.cast_to_unsigned()))
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
