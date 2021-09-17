/*use std::ops::Add;*/

/*use std::{mem, ops::Add};*/

use crate::{basics::*, ram_state::RamState};



#[derive(Debug)]
pub struct Timed<T> {
  op: T,
  time: SWord,  // Should be signed
}

#[derive(Debug)]
pub enum Register {
  A, B, C, D
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

/*pub type Offset = Word;  // As signed*/
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



fn decode_reg(memory: &RamState, pc: &mut Address) -> Timed<Register> {
  let op = 
    match memory[*pc] {
      0x0 => Register::A, 
      0x1 => Register::B, 
      0x2 => Register::C, 
      0x3 => Register::D, 
      _ => unreachable!(), 
    };
  *pc += 1;
  let time = memory[*pc] as SWord;
  *pc += 1;
  Timed { op, time }
}

fn decode_word(memory: &RamState, pc: &mut Address) -> Word {
  let word = memory[*pc];
  *pc += 1;
  word
}

fn decode_addr(memory: &RamState, pc: &mut Address) -> Timed<Address> {
  let hi: Word = memory[*pc];
  *pc += 1;
  let lo: Word = memory[*pc];
  *pc += 1;
  let op: Address = ((hi as Address) << 8) + (lo as Address);
  let time = memory[*pc] as SWord;
  *pc += 1;
  Timed { op, time }
}

impl Operand {
  fn decode(memory: &RamState, pc: &mut Address, mode: Word) -> Self {
    use Operand::*;
    match mode {
      0x0 => Reg (decode_reg (memory, pc)),
      0x1 => Imm (decode_word(memory, pc)),
      0x2 => Abs (decode_addr(memory, pc)),
      0x3 => Ind (decode_addr(memory, pc)), 
      _ => unreachable!(), 
    }
  }
}

impl Operands {
  fn decode(memory: &RamState, pc: &mut Address, mode: Word) -> Self {
    use Operands::*;
    match mode {
      0x0 => RegReg { src: decode_reg (memory, pc), dst: decode_reg (memory, pc) }, 
      0x1 => ImmReg { src: decode_word(memory, pc), dst: decode_reg (memory, pc) }, 
      0x2 => AbsReg { src: decode_addr(memory, pc), dst: decode_reg (memory, pc) }, 
      0x3 => IndReg { src: decode_addr(memory, pc), dst: decode_reg (memory, pc) }, 
      0x4 => RegAbs { src: decode_reg (memory, pc), dst: decode_addr(memory, pc) }, 
      0x5 => RegInd { src: decode_reg (memory, pc), dst: decode_addr(memory, pc) }, 
      0x6 => ImmAbs { src: decode_word(memory, pc), dst: decode_addr(memory, pc) }, 
      0x7 => ImmInd { src: decode_word(memory, pc), dst: decode_addr(memory, pc) }, 
      _ => unreachable!(), 
    }
  }
}

impl Offset {
  fn decode(memory: &RamState, pc: &mut Address) -> Self {
    let word = memory[*pc];
    *pc += 1;
    Offset(word as SWord)
  }
}

impl Instruction {
  /// Decode the next instruction at memory[pc].
  /// Encoding scheme: 5 bits for instruction, 3 bits for addressing mode (or 4/4?)
  fn decode(memory: &RamState, pc: &mut Address) -> Self {
    use Instruction::*;  // Fds eu ter de dizer use `Foo::*` dentro de `impl Foo` também é de génio
    let word = memory[*pc];
    let (opcode, addressing) = (word >> 3, word & ((1<<3) - 1));
    *pc += 1;
    match opcode {  
      0x001 => Mov (Operands::decode(memory, pc, addressing)), 
      0x002 => Add (Operands::decode(memory, pc, addressing)), 
      0x003 => Sub (Operands::decode(memory, pc, addressing)), 
      0x004 => Cmp (Operands::decode(memory, pc, addressing)), 
      0x005 => Jmp (Operand ::decode(memory, pc, addressing)), 
      0x006 => Beq (Offset  ::decode(memory, pc)),
      0x007 => Bne (Offset  ::decode(memory, pc)),
      0x008 => Bpl (Offset  ::decode(memory, pc)),
      0x009 => Bmi (Offset  ::decode(memory, pc)),
      0x00a => Cal (Operand ::decode(memory, pc, addressing)),
      0x00b => Ret,
      0x00c => Nop,
      _ => panic!("Illegal opcode: {:?}", opcode),  
    }
  }
}
