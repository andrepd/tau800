use crate::instruction::{Instruction, Operand, Operands, Register, Timed};
use crate::prelude::*;
use std::iter::Peekable;
use std::str::{CharIndices, Chars, Lines};

struct LineIterator<'i> {
    lines: Lines<'i>,
    line_idx: usize,
}

impl<'i> Iterator for LineIterator<'i> {
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
            line = line.trim();
        }
        Some(read_instruction(line, self.line_idx))
    }
}

fn assemble<'i>(input: &'i str) -> LineIterator<'i> {
    LineIterator {
        lines: input.lines(),
        line_idx: 0,
    }
}

fn read_instruction(literal: &str, line_idx: usize) -> Instruction {
    let mut iter = WindowSource::new(literal).window();
    
    let mnemonic = { iter.take_while(|c| !c.is_whitespace()); iter.collect() };

    let operands = match read_operands(&mut iter) {
        Ok(operands) => operands,
        Err(err) => match err {
            ReadError::NoMoreChars => panic!("Unexpected EOF"),
            ReadError::UnexpectedChar(col) => {
                panic!("Unexpected character at {}:{}", line_idx, col)
            }
        },
    };
}

#[derive(Debug)]
enum ReadError {
    NoMoreChars,
    UnexpectedChar(usize),
}

type ReadResult<T> = Result<T, ReadError>;

struct WindowSource<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
}

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
            start: self.start,
            last: self.start,
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

    fn peek(&self) -> Option<&char> {
        self.parent.chars.peek().map(|(_, c)| c)
    }

    fn forget(&mut self) -> () {
        self.start = self.last + 1;
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

    fn take_while<F: Fn(&char) -> bool>(mut self, pred: F) -> Self {
        let mut next = Some(' '); // doesn't matter the char here
        while next.is_some() && self.peek().is_some() && pred(self.peek().unwrap()) {
            next = self.next()
        }
        self
    }
}

trait OptionalRead<T> {
    fn optional(self) -> ReadResult<Option<T>>;
}

impl<T> OptionalRead<T> for ReadResult<T> {
    fn optional(self) -> ReadResult<Option<T>> {
        match self {
            Ok(x) => Ok(Some(x)),
            Err(err) => match err {
                ReadError::NoMoreChars => Err(err),
                ReadError::UnexpectedChar(_) => Ok(None),
            },
        }
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

fn eat_whitespace<'k, 's>(chars: &mut SlidingWindow<'k, 's>) -> SlidingWindow<'k, 's> {
    chars.take_while(|c| c.is_whitespace())
}

fn read_register(chars: &mut SlidingWindow) -> ReadResult<Register> {
    let register = match read_char(chars)? {
        'a' => Register::A,
        'b' => match read_char(chars)? {
            'h' => Register::BH,
            'l' => Register::BL,
            _ => unreachable!(),
        },
        'c' => match read_char(chars)? {
            'h' => Register::CH,
            'l' => Register::CL,
            _ => unreachable!(),
        },
        'x' => Register::X,
        's' => {
            debug_assert!(read_char(chars)? == 'p');
            unimplemented!("SP is not readable")
            //Register::SP
        }
        _ => unreachable!(),
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

    let low = read_hex_char()?;
    let high = read_hex_char()?;

    let low = low.to_digit(16).unwrap() as u8;
    let high = (high.to_digit(16).unwrap() as u8) << 4;

    let value = high + low;

    Ok(UWord::from(value))
}

fn read_decimal(chars: &mut SlidingWindow) -> ReadResult<UWord> {
    let subwindow = chars.window_from_here();
    let value = subwindow
        .take_while(|c| c.is_digit(10))
        .collect()
        .map_or(Err(ReadError::NoMoreChars), |s| Ok(s))?;
    if value.is_empty() {
        return Err(ReadError::UnexpectedChar(chars.pos()));
    }
    let value = value.parse::<u8>().unwrap();
    Ok(Word::from(value))
}

fn read_time(chars: &mut SlidingWindow) -> ReadResult<IWord> {
    match match_char('@', chars).optional()? {
        None => Ok(IWord::zero()),
        Some(_) => {
            let negative = match_char('-', chars).map(|_| true)?;
            let absolute_value = read_decimal(chars)?.value() as i8;
            Ok(IWord::from(if negative {
                -absolute_value
            } else {
                match_char('+', chars).optional()?;
                absolute_value
            }))
        }
    }
}

fn read_operand(chars: &mut SlidingWindow) -> ReadResult<Operand> {
    let operand = match read_char(chars)? {
        '#' => {
            let word = read_hex_word(chars)?;
            Operand::Imm(word)
        }
        '%' => {
            let low = read_hex_word(chars)?;
            let high = read_hex_word(chars)?;
            let op = Address::from_words(high, low);

            let next = chars.peek();
            if next.is_some() && *next.unwrap() == ',' {
                match_char(',', chars)?;
                match_char('X', chars)?;

                let time = read_time(chars)?;
                Operand::Abx(Timed::new(op, time))
            } else {
                let time = read_time(chars)?;
                Operand::Abs(Timed::new(op, time))
            }
        }
        '(' => {
            let operand = match read_char(chars)? {
                '%' => {
                    let low = read_hex_word(chars)?;
                    let high = read_hex_word(chars)?;
                    let op = Address::from_words(high, low);
                    let time = read_time(chars)?;

                    Operand::Ind(Timed::new(op, time))
                }
                _ => unimplemented!(),
            };
            match_char(')', chars)?;
            operand
        }
        _ => {
            let register = read_register(chars)?;
            let time = read_time(chars)?;
            Operand::Reg(Timed::new(register, time))
        }
    };
    Ok(operand)
}

fn read_operands(chars: &mut SlidingWindow) -> ReadResult<Operands> {
    let src = read_operand(chars)?;
    let dest = read_operand(chars)?;
    Ok(Operands::new(src, dest))
}
