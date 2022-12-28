use super::prelude::*;

/// Addressable registers
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

/// A time jump offset
type TimeOffset = iLong;

/// An operand T at a given time offset
#[derive(Debug, Clone)]
pub struct Timed<T> {
    pub op: T,
    /// Relative time of operation (wrt. call)
    pub time: TimeOffset,
}

/*impl<T> Timed<T> {
    pub fn new(op: T, time: iLong) -> Self {
        Timed { op, time }
    }
}*/

/// The basic addressing modes
#[derive(Debug, Clone)]
pub enum Op {
    /// Named register
    Reg(Register),
    /// Literal one-word value
    Imm(uWord),
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

/// The argument of an instruction that takes one operand
pub type Operand = Timed<Op>;

/// The argument of an instruction that takes two operands
#[derive(Debug, Clone)]
pub struct Operands {
    pub src: Operand,
    pub dst: Operand,
}

/*impl Operands {
    pub fn new(src: Operand, dst: Operand) -> Self {
        Operands { src, dst }
    }
}*/

/// The argument of an instruction that is a pc offset
pub type Offset = iWord;

//

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Operands),
    /*Xch(Operands),*/
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
    Asr(Operand),

    Inc(Operand),
    Dec(Operand),

    Cmp(Operands),
    Bit(Operands),

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
    Hcf,
}

//

fn read_register(m: &mut Machine) -> Register {
    use Register::*;
    match m.read_pc().value() {
        0x0 => A,
        /*0x1 => F,*/
        0x2 => BH,
        0x3 => BL,
        0x4 => CH,
        0x5 => CL,
        0x6 => X,
        /*0x7 => SP,*/
        _ => unreachable!(),
    }
}

fn read_word(m: &mut Machine) -> uWord {
    m.read_pc()
}

fn read_address(m: &mut Machine) -> Address {
    let lo = m.read_pc();
    let hi = m.read_pc();
    Address::from_hi_lo(hi, lo)
}

fn read_offset(m: &mut Machine) -> Offset {
    m.read_pc().as_iword()
}

fn read_time(m: &mut Machine) -> TimeOffset {
    let lo = m.read_pc();
    let hi = m.read_pc().as_iword();    
    TimeOffset::from_hi_lo(hi, lo)
}

//

fn write_register(m: &mut Machine, x: &Register) {
    use Register::*;
    match x {
        A  => m.write_pc(uWord::lit(0x0)),
        /*F  => m.write_pc(uWord::lit(0x1)),*/
        BH => m.write_pc(uWord::lit(0x2)),
        BL => m.write_pc(uWord::lit(0x3)),
        CH => m.write_pc(uWord::lit(0x4)),
        CL => m.write_pc(uWord::lit(0x5)),
        X  => m.write_pc(uWord::lit(0x6)),
        /*SP => m.write_pc(uWord::lit(0x7)),*/
    }
}

fn write_word(m: &mut Machine, x: &uWord) {
    m.write_pc(*x)
}

fn write_address(m: &mut Machine, x: &Address) {
    m.write_pc(x.lo());
    m.write_pc(x.hi());
}

fn write_offset(m: &mut Machine, x: &Offset) {
    m.write_pc(x.as_uword())
}

fn write_time(m: &mut Machine, x: &TimeOffset) {
    m.write_pc(x.lo());
    m.write_pc(x.hi().as_uword());
}

// 

impl Operand {
    fn decode(m: &mut Machine, mode: uWord) -> Self {
        use Op::*;
        let time_flag = mode.value() & 0b100000 != 0;
        let operand_flag = mode.value() & 0b011111;
        match time_flag {
            true => match operand_flag {
                0x0 => Timed { op: Reg(read_register(m)), time: read_time(m) },
                0x1 => Timed { op: Abs(read_address(m)),  time: read_time(m) },
                0x2 => Timed { op: Ind(read_address(m)),  time: read_time(m) },
                0x3 => Timed { op: Abx(read_address(m)),  time: read_time(m) },
                0x4 => Timed { op: Imm(read_word(m)),     time: iLong::ZERO  },
                _ => unreachable!(),
            }
            false => match operand_flag {
                0x0 => Timed { op: Reg(read_register(m)), time: iLong::ZERO },
                0x1 => Timed { op: Abs(read_address(m)),  time: iLong::ZERO },
                0x2 => Timed { op: Ind(read_address(m)),  time: iLong::ZERO },
                0x3 => Timed { op: Abx(read_address(m)),  time: iLong::ZERO },
                0x4 => Timed { op: Imm(read_word(m)),     time: iLong::ZERO },
                _ => unreachable!(),
            }
        }
    }

    fn encode(m: &mut Machine, x: &Operand) -> () {
        use Op::*;
        let time_flag = x.time != iLong::ZERO;
        let mode = |a,b| uWord::try_from((a as u8) | (b as u8)).unwrap();
        match &x.op {
            Reg(y) => {
                m.write_pc(mode(0x0, time_flag));
                write_register(m, &y);
                if time_flag { write_time(m, &x.time) };
            }
            Abs(y) => {
                m.write_pc(mode(0x1, time_flag));
                write_address(m, &y);
                if time_flag { write_time(m, &x.time) };
            }
            Ind(y) => {
                m.write_pc(mode(0x2, time_flag));
                write_address(m, &y);
                if time_flag { write_time(m, &x.time) };
            }
            Abx(y) => {
                m.write_pc(mode(0x3, time_flag));
                write_address(m, &y);
                if time_flag { write_time(m, &x.time) };
            }
            Imm(y) => {
                m.write_pc(mode(0x4, time_flag));
                write_word(m, &y);
            }
        };
    }
}

impl Operands {
    fn decode(m: &mut Machine, mode: uWord) -> Self {
        let time_mode = (mode.value() & 0b100000);
        let src_mode  = (mode.value() & 0b011111) % 0x5;
        let dst_mode  = (mode.value() & 0b011111) / 0x5;
        let src = Operand::decode(m, uWord::try_from(src_mode | time_mode).unwrap());
        let dst = Operand::decode(m, uWord::try_from(dst_mode | time_mode).unwrap());
        Operands { src, dst }
    }

    fn encode(m: &mut Machine, x: &Operands) -> () {
        use Op::*;
        let mut mode = 0;
        let time_flag = x.src.time != iLong::ZERO || x.dst.time != iLong::ZERO;
        if time_flag { mode |= 0b100000 };
        match x.src.op {
            Reg(_) => mode += 0x0,
            Abs(_) => mode += 0x1,
            Ind(_) => mode += 0x2,
            Abx(_) => mode += 0x3,
            Imm(_) => mode += 0x4,
        };
        match x.dst.op {
            Reg(_) => mode += 0x0 * 0x5,
            Abs(_) => mode += 0x1 * 0x5,
            Ind(_) => mode += 0x2 * 0x5,
            Abx(_) => mode += 0x3 * 0x5,
            Imm(_) => mode += 0x4 * 0x5,
        };
        m.write_pc(uWord::try_from(mode).unwrap());
        match &x.src.op {
            Reg(y) => { write_register(m, &y); if time_flag { write_time(m, &x.src.time) } }
            Abs(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.src.time) } }
            Ind(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.src.time) } }
            Abx(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.src.time) } }
            Imm(y) => write_word(m, &y),
        };
        match &x.dst.op {
            Reg(y) => { write_register(m, &y); if time_flag { write_time(m, &x.dst.time) } }
            Abs(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.dst.time) } }
            Ind(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.dst.time) } }
            Abx(y) => { write_address(m, &y);  if time_flag { write_time(m, &x.dst.time) } }
            Imm(y) => write_word(m, &y),
        };
    }
}

impl Instruction {
    pub fn decode(m: &mut Machine) -> Self {
        use Instruction::*;
        let opcode = m.read_pc();
        /*let mode = m.read_pc();*/
        match opcode.value() {
            0x00 => Nop, 
            0x01 => { let mode = m.read_pc(); Mov(Operands::decode(m, mode)) },
            /*0x02 => { let mode = m.read_pc(); Xch(Operands::decode(m, mode)) },*/
            0x03 => { let mode = m.read_pc(); Psh(Operand::decode(m, mode))  },
            0x04 => { let mode = m.read_pc(); Pop(Operand::decode(m, mode))  },
            0x10 => { let mode = m.read_pc(); Add(Operands::decode(m, mode)) },
            0x11 => { let mode = m.read_pc(); Sub(Operands::decode(m, mode)) },
            0x12 => { let mode = m.read_pc(); Mul(Operands::decode(m, mode)) },
            0x13 => { let mode = m.read_pc(); Muh(Operands::decode(m, mode)) },
            0x14 => { let mode = m.read_pc(); Mus(Operands::decode(m, mode)) },
            0x15 => { let mode = m.read_pc(); Div(Operands::decode(m, mode)) },
            0x16 => { let mode = m.read_pc(); Mod(Operands::decode(m, mode)) },
            0x17 => { let mode = m.read_pc(); And(Operands::decode(m, mode)) },
            0x18 => { let mode = m.read_pc(); Or (Operands::decode(m, mode)) },
            0x19 => { let mode = m.read_pc(); Xor(Operands::decode(m, mode)) },
            0x1a => { let mode = m.read_pc(); Not(Operand::decode(m, mode))  },
            0x1b => { let mode = m.read_pc(); Lsl(Operand::decode(m, mode))  },
            0x1c => { let mode = m.read_pc(); Lsr(Operand::decode(m, mode))  },
            0x1d => { let mode = m.read_pc(); Asr(Operand::decode(m, mode))  },
            0x1e => { let mode = m.read_pc(); Inc(Operand::decode(m, mode))  },
            0x1f => { let mode = m.read_pc(); Dec(Operand::decode(m, mode))  },
            0x20 => { let mode = m.read_pc(); Cmp(Operands::decode(m, mode)) },
            0x21 => { let mode = m.read_pc(); Bit(Operands::decode(m, mode)) },
            0x30 => Jmp(read_address(m)),
            0x31 => Bcc(read_offset(m)),
            0x32 => Bcs(read_offset(m)),
            0x33 => Bne(read_offset(m)),
            0x34 => Beq(read_offset(m)),
            0x35 => Bpl(read_offset(m)),
            0x36 => Bmi(read_offset(m)),
            0x37 => Cal(read_address(m)),
            0x38 => Clc,
            0x39 => Sec,
            0x3e => Hcf,
            0x3f => Ret,
            _ => panic!("Invalid instruction {:x} at {:x}.", opcode.value(), m.cpu.pc.value() - 1),
        }
    }

    pub fn encode(&self, m: &mut Machine) {
        use Instruction::*;
        match self {
            Nop => { m.write_pc(uWord::lit(0x00)) }
            Clc => { m.write_pc(uWord::lit(0x38)) }
            Sec => { m.write_pc(uWord::lit(0x39)) }
            Hcf => { m.write_pc(uWord::lit(0x3e)) }
            Ret => { m.write_pc(uWord::lit(0x3f)) }
            Mov(ops) => { m.write_pc(uWord::lit(0x01)); Operands::encode(m, ops)}
            /*Xch(ops) => { m.write_pc(uWord::lit(0x02)); Operands::encode(m, ops)}*/
            Psh(op) => { m.write_pc(uWord::lit(0x03)); Operand::encode(m, op)}
            Pop(op) => { m.write_pc(uWord::lit(0x04)); Operand::encode(m, op)}
            Add(ops) => { m.write_pc(uWord::lit(0x10)); Operands::encode(m, ops)}
            Sub(ops) => { m.write_pc(uWord::lit(0x11)); Operands::encode(m, ops)}
            Mul(ops) => { m.write_pc(uWord::lit(0x12)); Operands::encode(m, ops)}
            Muh(ops) => { m.write_pc(uWord::lit(0x13)); Operands::encode(m, ops)}
            Mus(ops) => { m.write_pc(uWord::lit(0x14)); Operands::encode(m, ops)}
            Div(ops) => { m.write_pc(uWord::lit(0x15)); Operands::encode(m, ops)}
            Mod(ops) => { m.write_pc(uWord::lit(0x16)); Operands::encode(m, ops)}
            And(ops) => { m.write_pc(uWord::lit(0x17)); Operands::encode(m, ops)}
            Or (ops) => { m.write_pc(uWord::lit(0x18)); Operands::encode(m, ops)}
            Xor(ops) => { m.write_pc(uWord::lit(0x19)); Operands::encode(m, ops)}
            Not(op) => { m.write_pc(uWord::lit(0x1a)); Operand::encode(m, op)}
            Lsl(op) => { m.write_pc(uWord::lit(0x1b)); Operand::encode(m, op)}
            Lsr(op) => { m.write_pc(uWord::lit(0x1c)); Operand::encode(m, op)}
            Asr(op) => { m.write_pc(uWord::lit(0x1d)); Operand::encode(m, op)}
            Inc(op) => { m.write_pc(uWord::lit(0x1e)); Operand::encode(m, op)}
            Dec(op) => { m.write_pc(uWord::lit(0x1f)); Operand::encode(m, op)}
            Cmp(op) => { m.write_pc(uWord::lit(0x20)); Operands::encode(m, op)}
            Bit(op) => { m.write_pc(uWord::lit(0x21)); Operands::encode(m, op)}
            Jmp(x) => { m.write_pc(uWord::lit(0x30)); write_address(m, x)}
            Bcc(x) => { m.write_pc(uWord::lit(0x31)); write_offset(m, x)}
            Bcs(x) => { m.write_pc(uWord::lit(0x32)); write_offset(m, x)}
            Bne(x) => { m.write_pc(uWord::lit(0x33)); write_offset(m, x)}
            Beq(x) => { m.write_pc(uWord::lit(0x34)); write_offset(m, x)}
            Bpl(x) => { m.write_pc(uWord::lit(0x35)); write_offset(m, x)}
            Bmi(x) => { m.write_pc(uWord::lit(0x36)); write_offset(m, x)}
            Cal(x) => { m.write_pc(uWord::lit(0x37)); write_address(m, x)}
        }
    }
}
