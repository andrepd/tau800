use radix_fmt::radix;

use super::instruction::{Instruction, Op, Operand, Operands, Register, Timed};
use super::modules::{Module, ModuleCollection};
use super::prelude::*;
use std::iter::Peekable;
use std::str::{CharIndices, Lines};

pub struct InstructionIterator<'i> {
    lines: Lines<'i>,
    line_idx: usize,
}

impl<'i> Iterator for InstructionIterator<'i> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = "";
        while line.is_empty() {
            let iter_line = self.lines.next();
            self.line_idx += 1;
            if iter_line.is_none() {
                return None;
            }
            line = iter_line.unwrap();
            line = match line.split_once(";") {
                Some((prefix, _suffix)) => prefix,
                None => line,
            };
            line = match line.split_once(":") {
                Some((prefix, _suffix)) => prefix,
                None => line,
            };
            line = line.trim();
        }
        Some(read_instruction(line, self.line_idx))
    }
}

pub fn assemble<'i>(input: &'i str) -> InstructionIterator<'i> {
    InstructionIterator {
        lines: input.lines(),
        line_idx: 0,
    }
}

pub fn assemble_into(m: &mut Machine, input: &str) {
    for i in assemble(input) {
        Instruction::encode(m, &i);
    }

    m.cpu.pc = Address::from(0x80);
}

fn read_instruction(literal: &str, line_idx: usize) -> Instruction {
    let mut source = WindowSource::new(literal);
    let mut iter = source.window();

    let mnemonic = {
        iter.take_while(|c| !c.is_whitespace())
            .collect()
            .expect("Could not find a mnemonic.")
    };

    eat_whitespace(&mut iter);

    let instruction = match mnemonic {
        "mov" => Instruction::Mov(read_operands(&mut iter).or_panic(line_idx)),
        "psh" => Instruction::Psh(read_operand(&mut iter).or_panic(line_idx)),
        "pop" => Instruction::Pop(read_operand(&mut iter).or_panic(line_idx)),
        "add" => Instruction::Add(read_operands(&mut iter).or_panic(line_idx)),
        "sub" => Instruction::Sub(read_operands(&mut iter).or_panic(line_idx)),
        "mul" => Instruction::Mul(read_operands(&mut iter).or_panic(line_idx)),
        "muh" => Instruction::Muh(read_operands(&mut iter).or_panic(line_idx)),
        "mus" => Instruction::Mus(read_operands(&mut iter).or_panic(line_idx)),
        "div" => Instruction::Div(read_operands(&mut iter).or_panic(line_idx)),
        "mod" => Instruction::Mod(read_operands(&mut iter).or_panic(line_idx)),
        "and" => Instruction::And(read_operands(&mut iter).or_panic(line_idx)),
        "or" => Instruction::Or(read_operands(&mut iter).or_panic(line_idx)),
        "xor" => Instruction::Xor(read_operands(&mut iter).or_panic(line_idx)),
        "not" => Instruction::Not(read_operand(&mut iter).or_panic(line_idx)),
        "lsl" => Instruction::Lsl(read_operand(&mut iter).or_panic(line_idx)),
        "lsr" => Instruction::Lsr(read_operand(&mut iter).or_panic(line_idx)),
        "inc" => Instruction::Inc(read_operand(&mut iter).or_panic(line_idx)),
        "dec" => Instruction::Dec(read_operand(&mut iter).or_panic(line_idx)),
        "cmp" => Instruction::Cmp(read_operands(&mut iter).or_panic(line_idx)),
        "bit" => Instruction::Bit(read_operands(&mut iter).or_panic(line_idx)),
        "jmp" => Instruction::Jmp(read_address(&mut iter).or_panic(line_idx)),
        "bcc" => Instruction::Bcc(read_decimal(&mut iter).or_panic(line_idx)),
        "bcs" => Instruction::Bcs(read_decimal(&mut iter).or_panic(line_idx)),
        "bne" => Instruction::Bne(read_decimal(&mut iter).or_panic(line_idx)),
        "beq" => Instruction::Beq(read_decimal(&mut iter).or_panic(line_idx)),
        "bpl" => Instruction::Bpl(read_decimal(&mut iter).or_panic(line_idx)),
        "bmi" => Instruction::Bmi(read_decimal(&mut iter).or_panic(line_idx)),
        "clc" => Instruction::Clc,
        "sec" => Instruction::Sec,
        "cal" => Instruction::Cal(read_address(&mut iter).or_panic(line_idx)),
        "ret" => Instruction::Ret,
        "nop" => Instruction::Nop,
        "hcf" => Instruction::Hcf,
        _ => panic!("Invalid mnemonic {}", mnemonic),
    };

    instruction
}

#[derive(Debug)]
enum ReadError {
    NoMoreChars,
    UnexpectedChar(usize),
}

type ReadResult<T> = Result<T, ReadError>;

trait ReadOrPanic<T> {
    fn or_panic(self, line: usize) -> T;
}

impl<T> ReadOrPanic<T> for ReadResult<T> {
    fn or_panic(self, line: usize) -> T {
        match self {
            Ok(value) => value,
            Err(err) => match err {
                ReadError::NoMoreChars => panic!("Unexpected EOF."),
                ReadError::UnexpectedChar(col) => {
                    panic!("Unexpected character at {}:{}", line, col)
                }
            },
        }
    }
}

#[derive(Debug)]
struct WindowSource<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
}

#[derive(Debug)]
struct SlidingWindow<'k, 's: 'k> {
    parent: &'k mut WindowSource<'s>,
    start: usize,
    last: usize,
}

impl<'s> WindowSource<'s> {
    fn new(source: &'s str) -> Self {
        WindowSource {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    fn window(&mut self) -> SlidingWindow<'_, 's> {
        SlidingWindow {
            parent: self,
            start: 0,
            last: 0,
        }
    }
}

impl<'k, 's> SlidingWindow<'k, 's> {
    fn window_from_here(&mut self) -> SlidingWindow<'_, 's> {
        SlidingWindow {
            parent: self.parent,
            start: self.last,
            last: self.last,
        }
    }

    fn oob(&self, idx: usize) -> bool {
        idx >= self.parent.source.len()
    }

    fn pos(&self) -> usize {
        self.last
    }

    fn next(&mut self) -> Option<char> {
        if let Some((next_idx, next_chr)) = self.parent.chars.next() {
            self.last = next_idx;
            Some(next_chr)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.parent.chars.peek().map(|(_, c)| c)
    }

    fn collect(&mut self) -> Option<&'s str> {
        if self.oob(self.start) {
            return None;
        }
        let slice = &self.parent.source[self.start..=self.last];
        self.start = self.last + 1;
        self.last = self.start;
        Some(slice)
    }

    fn take_while<F: Fn(&char) -> bool>(&mut self, pred: F) -> &mut Self {
        while self.peek().is_some() && pred(self.peek().unwrap()) {
            self.next();
        }
        self
    }
}

trait OptionalRead<T> {
    fn optional(self) -> Option<T>;
}

impl<T> OptionalRead<T> for ReadResult<T> {
    fn optional(self) -> Option<T> {
        self.map_or(None, |x| Some(x))
    }
}

fn read_char(chars: &mut SlidingWindow) -> ReadResult<char> {
    let next_char = chars.next();
    if next_char.is_none() {
        Err(ReadError::NoMoreChars)
    } else {
        Ok(next_char.unwrap())
    }
}

fn match_char(to_match: char, chars: &mut SlidingWindow) -> ReadResult<()> {
    let peek = chars.peek();
    if peek.is_none() {
        return Err(ReadError::NoMoreChars);
    }
    if *peek.unwrap() == to_match {
        chars.next();
        return Ok(());
    } else {
        return Err(ReadError::UnexpectedChar(chars.pos()));
    }
}

fn eat_whitespace<'r, 'k, 's>(
    chars: &'r mut SlidingWindow<'k, 's>,
) -> &'r mut SlidingWindow<'k, 's> {
    chars.take_while(|c| c.is_whitespace())
}

fn read_register(chars: &mut SlidingWindow) -> ReadResult<Register> {
    let register = match read_char(chars)? {
        'a' => Register::A,
        'b' => match read_char(chars)? {
            'h' => Register::BH,
            'l' => Register::BL,
            _ => return Err(ReadError::UnexpectedChar(chars.pos())),
        },
        'c' => match read_char(chars)? {
            'h' => Register::CH,
            'l' => Register::CL,
            _ => return Err(ReadError::UnexpectedChar(chars.pos())),
        },
        'x' => Register::X,
        's' => {
            debug_assert!(read_char(chars)? == 'p');
            unimplemented!("SP is not addressable")
            //Register::SP
        }
        _ => return Err(ReadError::UnexpectedChar(chars.pos())),
    };
    Ok(register)
}

fn read_hex_word(chars: &mut SlidingWindow) -> ReadResult<UWord> {
    let mut read_hex_char = || {
        let next = chars.peek();
        if next.is_none() {
            return Err(ReadError::NoMoreChars);
        }
        let next = next.unwrap();
        if next.is_digit(16) {
            Ok(chars.next().unwrap())
        } else {
            Err(ReadError::UnexpectedChar(chars.pos()))
        }
    };

    // É high-word/low-word, mas não high char low char!
    // e.g. $abcd = 0xab + 2^6 × 0xcd
    let high = read_hex_char()?;
    let low = read_hex_char()?;

    let high = (high.to_digit(16).unwrap() as u8) << 4;
    let low = low.to_digit(16).unwrap() as u8;

    let value = high + low;

    Ok(UWord::from(value))
}

fn read_address(chars: &mut SlidingWindow) -> ReadResult<Address> {
    let low = read_hex_word(chars)?;
    let high = read_hex_word(chars)?;
    Ok(Address { low, high })
}

// Can be signed
fn read_decimal(chars: &mut SlidingWindow) -> ReadResult<IWord> {
    let mut subwindow = chars.window_from_here();
    let value = subwindow
        .take_while(|c| c.is_digit(10) || *c == '+' || *c == '-')
        .collect()
        .map_or(Err(ReadError::NoMoreChars), |s| Ok(s))?;
    if value.is_empty() {
        return Err(ReadError::UnexpectedChar(chars.pos()));
    }
    let value = value.trim().parse::<i8>().unwrap();
    Ok(IWord::from(value))
}

fn read_time(chars: &mut SlidingWindow) -> ReadResult<IWord> {
    match match_char('@', chars).optional() {
        None => Ok(IWord::zero()),
        Some(_) => {
            let _ = match_char('-', chars);
            let _ = match_char('+', chars);
            let value = read_decimal(chars)?;
            eprintln!("qux {:?} {:?}", value, IWord::from(value));
            Ok(IWord::from(value))
        }
    }
}

fn read_operand(chars: &mut SlidingWindow) -> ReadResult<Operand> {
    let next = chars.peek().map_or(Err(ReadError::NoMoreChars), |x| Ok(x));
    let operand = match next? {
        '#' => {
            match_char('#', chars)?;
            let word = read_hex_word(chars)?;
            Timed::new(Op::Imm(word), 0.into())
        }
        '%' => {
            match_char('%', chars)?;
            let low = read_hex_word(chars)?;
            let high = read_hex_word(chars)?;
            let op = Address { low, high };

            
            let next = chars.peek();
            if next.is_some() && *next.unwrap() == ',' {
                match_char(',', chars)?;
                match_char('x', chars)?;

                let time = read_time(chars)?;
                Timed::new(Op::Abx(op), time)
            } else {
                
                let time = read_time(chars)?;
                Timed::new(Op::Abs(op), time)
            }
        }
        '(' => {
            match_char('(', chars)?;
            let operand = match read_char(chars)? {
                '%' => {
                    let low = read_hex_word(chars)?;
                    let high = read_hex_word(chars)?;
                    let op = Address { low, high };
                    let time = read_time(chars)?;

                    Timed::new(Op::Ind(op), time)
                }
                _ => unreachable!(), /*{
                                         let register = read_register(chars)?;
                                         let time = read_time(chars)?;
                                         Operand::Reg(Timed::new(register, time))
                                     }*/
            };
            match_char(')', chars)?;
            operand
        }
        _ => {
            let register = read_register(chars)?;
            let time = read_time(chars)?;
            Timed::new(Op::Reg(register), time)
        }
    };
    Ok(operand)
}

fn read_operands(chars: &mut SlidingWindow) -> ReadResult<Operands> {
    let src = read_operand(chars)?;
    eat_whitespace(chars);
    let dest = read_operand(chars)?;
    Ok(Operands::new(src, dest))
}

pub fn mnemonic(cmd: Instruction) -> String {
    let mnemonic_op = |op: Op| -> String {
        match op {
            Op::Reg(r) => match r {
                Register::A => "a",
                Register::BH => "bh",
                Register::BL => "bl",
                Register::CH => "ch",
                Register::CL => "cl",
                Register::X => "x",
            }
            .to_string(),
            Op::Imm(v) => format!("#{}", radix(v.value(), 10)),
            Op::Abs(v) => format!("%{}", radix(v.value(), 10)),
            Op::Abx(v) => format!("%{},x", radix(v.value(), 10)),
            Op::Ind(v) => format!("(%{})", radix(v.value(), 10)),
        }
    };
    let mnemonic_timed_op = |op: Operand| -> String {
        let time = op.time.value();
        let op = op.op;
        if time == 0 {
            mnemonic_op(op)
        } else {
            if time > 0 {
                format!("{}@+{}", mnemonic_op(op), radix(time, 10))
            } else {
                format!("{}@-{}", mnemonic_op(op), radix(time.abs(), 10))
            }
        }
    };

    match cmd {
        Instruction::Mov(Operands { src, dst }) => {
            format!("mov {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Psh(op) => format!("psh {}", mnemonic_timed_op(op)),
        Instruction::Pop(op) => format!("pop {}", mnemonic_timed_op(op)),
        Instruction::Add(Operands { src, dst }) => {
            format!("add {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Sub(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Mul(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Muh(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Mus(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Div(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Mod(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::And(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Or(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Xor(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Not(op) => format!("not {}", mnemonic_timed_op(op)),
        Instruction::Lsl(op) => format!("lsl {}", mnemonic_timed_op(op)),
        Instruction::Lsr(op) => format!("lsr {}", mnemonic_timed_op(op)),
        Instruction::Inc(op) => format!("inc {}", mnemonic_timed_op(op)),
        Instruction::Dec(op) => format!("dec {}", mnemonic_timed_op(op)),
        Instruction::Cmp(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Bit(Operands { src, dst }) => {
            format!(" {} {}", mnemonic_timed_op(src), mnemonic_timed_op(dst))
        }
        Instruction::Jmp(op) => format!("jmp %{}", radix(op.value(), 16)),
        Instruction::Bcc(op) => format!("bcc #{}", radix(op.value() as u8, 16)),
        Instruction::Bcs(op) => format!("bcs #{}", radix(op.value() as u8, 16)),
        Instruction::Bne(op) => format!("bne #{}", radix(op.value() as u8, 16)),
        Instruction::Beq(op) => format!("beq #{}", radix(op.value() as u8, 16)),
        Instruction::Bpl(op) => format!("bpl #{}", radix(op.value() as u8, 16)),
        Instruction::Bmi(op) => format!("bmi #{}", radix(op.value() as u8, 16)),
        Instruction::Clc => "clc".to_string(),
        Instruction::Sec => "sec".to_string(),
        Instruction::Cal(op) => format!("cal #{}", radix(op.value(), 16)),
        Instruction::Ret => "ret".to_string(),
        Instruction::Nop => "nop".to_string(),
        Instruction::Hcf => "hcf".to_string(),
    }
}
