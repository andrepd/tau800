use crate::prelude::*;



#[derive(Debug)]
pub struct Timed<T> {
  op: T,
  time: SWord,  // Should be signed
}

#[derive(Debug)]
pub enum Register {
  A, F, BH, BL, CH, CL, X, SP, 
}

/// Reg: named register
/// Imm: literal value
/// Abs: absolute address (byte order: hi lo)
/// Ind: indirect address (address is value at that address, hi lo order)
#[derive(Debug)]
pub enum Operand {
  Reg (Timed<Register>),
  Imm (Word),
  Abs (Timed<Address>),
  Ind (Timed<Address>), 
  // Ind {hi: Word, lo: Word}, 
}

#[derive(Debug)]
pub enum Operands {
  RegReg {src: Timed<Register>, dst: Timed<Register>}, 

  ImmReg {src: Word,            dst: Timed<Register>}, 
  AbsReg {src: Timed<Address>,  dst: Timed<Register>}, 
  IndReg {src: Timed<Address>,  dst: Timed<Register>}, 

  RegAbs {src: Timed<Register>, dst: Timed<Address> }, 
  RegInd {src: Timed<Register>, dst: Timed<Address> }, 

  ImmAbs {src: Word,            dst: Timed<Address> }, 
  ImmInd {src: Word,            dst: Timed<Address> }, 
}

#[derive(Debug)]
pub struct Offset (
  SWord
);  // Rust é péssimo. Porque é que aqui é preciso `;` mas se tiver named fields já não é?

#[derive(Debug)]
pub enum Instruction {
  Mov (Operands), 

  Add (Operands), 
  Sub (Operands), 
  Mul (Operands), 

  Cmp (Operands), 
  
  Jmp (Operand),

  Beq (Offset),
  Bne (Offset),
  Bpl (Offset),
  Bmi (Offset),

  Cal (Operand),
  Ret,

  Nop, 
}



fn decode_reg(m: &mut MachineState) -> Timed<Register> {
  let op = 
    match m.read_pc() {
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
  let time = m.read_pc() as SWord;
  Timed { op, time }
}

fn decode_word(m: &mut MachineState) -> Word {
  let word = m.read_pc();
  word
}

fn decode_addr(m: &mut MachineState) -> Timed<Address> {
  let hi: Word = m.read_pc();
  let lo: Word = m.read_pc();
  let op: Address = ((hi as Address) << 8) + (lo as Address);
  let time = m.read_pc() as SWord;
  Timed { op, time }
}

impl Operand {
  fn decode(m: &mut MachineState, mode: Word) -> Self {
    use Operand::*;
    match mode {
      0x0 => Reg (decode_reg (m)),
      0x1 => Imm (decode_word(m)),
      0x2 => Abs (decode_addr(m)),
      0x3 => Ind (decode_addr(m)), 
      _ => unreachable!(), 
    }
  }
}

impl Operands {
  fn decode(m: &mut MachineState, mode: Word) -> Self {
    use Operands::*;
    match mode {
      0x0 => RegReg { src: decode_reg (m), dst: decode_reg (m) }, 
      0x1 => ImmReg { src: decode_word(m), dst: decode_reg (m) }, 
      0x2 => AbsReg { src: decode_addr(m), dst: decode_reg (m) }, 
      0x3 => IndReg { src: decode_addr(m), dst: decode_reg (m) }, 
      0x4 => RegAbs { src: decode_reg (m), dst: decode_addr(m) }, 
      0x5 => RegInd { src: decode_reg (m), dst: decode_addr(m) }, 
      0x6 => ImmAbs { src: decode_word(m), dst: decode_addr(m) }, 
      0x7 => ImmInd { src: decode_word(m), dst: decode_addr(m) }, 
      _ => unreachable!(), 
    }
  }
}

impl Offset {
  fn decode(m: &mut MachineState) -> Self {
    let word = m.read_pc();
    Offset(word as SWord)
  }
}

impl Instruction {
  /// Decode the next instruction at memory[pc].
  /// Encoding scheme: 5 bits for instruction, 3 bits for addressing mode
  fn decode(m: &mut MachineState) -> Self {
    use Instruction::*;  // Fds eu ter de dizer use `Foo::*` dentro de `impl Foo` também é de génio
    // One-word instructions
    /*let word = memory[*pc];
    let (opcode, addressing) = (word >> 3, word & ((1<<3) - 1));*/
    // Two-word instructions
    let opcode = m.read_pc();
    let addressing = m.read_pc();
    match opcode {  
      0x001 => Mov (Operands::decode(m, addressing)), 
      0x002 => Add (Operands::decode(m, addressing)), 
      0x003 => Sub (Operands::decode(m, addressing)), 
      0x004 => Cmp (Operands::decode(m, addressing)), 
      0x005 => Jmp (Operand ::decode(m, addressing)), 
      0x006 => Beq (Offset  ::decode(m)),
      0x007 => Bne (Offset  ::decode(m)),
      0x008 => Bpl (Offset  ::decode(m)),
      0x009 => Bmi (Offset  ::decode(m)),
      0x00a => Cal (Operand ::decode(m, addressing)),
      0x00b => Ret,
      0x00c => Nop,
      _ => panic!("Illegal opcode: {:?}", opcode),  
    }
  }
}
