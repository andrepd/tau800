use crate::instruction::{Instruction, Operand, Operands, Register, Timed};
use crate::prelude::*;
use std::iter::Peekable;
use std::str::{Chars, Lines};

struct InstructionIterator<'i> {
    lines: Lines<'i>,
}

impl<'i> Iterator for InstructionIterator<'i> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = "";
        while line.is_empty() {
            let iter_line = self.lines.next();
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
        Some(read_instruction(line))
    }
}

fn assemble<'i>(input: &'i str) -> InstructionIterator<'i> {
    InstructionIterator {
        lines: input.lines(),
    }
}

fn read_instruction(literal: &str) -> Instruction {
    let mut words = literal.split_whitespace();
    let mnemonic = words
        .next()
        .expect("No mnemonic, even though line is not empty.");

    use Instruction::*;
    match mnemonic/*.as_str()*/ {
        "mov" => Mov(read_operands(words.next().unwrap(), words.next().unwrap())),
        _ => unimplemented!(),
    }
}

enum ReadError {
    NoMoreChars,
    UnexpectedChar,
}

type ReadResult<T> = Result<T, ReadError>;

fn read_char<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<char> {
    let next_char = chars.next();
    if next_char.is_none() {
        Err(ReadError::NoMoreChars)
    } else {
        Ok(next_char.unwrap())
    }
}

fn match_char<'s>(to_match: char, chars: &mut Peekable<Chars<'s>>) -> ReadResult<()> {
    let peek = chars.peek();
    if peek.is_none() {
        return Err(ReadError::NoMoreChars);
    }
    if *peek.unwrap() == to_match {
        chars.next();
        return Ok(());
    } else {
        return Err(ReadError::UnexpectedChar);
    }
}

fn read_register<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<Register> {
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
            Register::SP
        }
        _ => unreachable!(),
    };
    Ok(register)
}

fn read_hex_word<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<UWord> {
    let mut read_hex_char = || {
        let next = chars.peek();
        if next.is_none() {
            return Err(ReadError::NoMoreChars);
        }
        let next = next.unwrap();
        if next.is_digit(16) {
            Ok(chars.next().unwrap())
        } else {
            Err(ReadError::UnexpectedChar)
        }
    };

    // (Sorry; all I'm doing here is converting a None to an Err,
    //  and a Some to an Ok, and then unwrapping)
    let low = read_hex_char()?;
    let high = read_hex_char()?;

    let low = low.to_digit(16).unwrap() as u8;
    let high = (high.to_digit(16).unwrap() as u8) << 4;

    let value = high + low;

    Ok(UWord::from(value))
}

fn read_time<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<IWord> {
    let negative = match_char('-', chars).map(|_| true)?;
    let absolute_value = read_hex_word(chars)?.value() as i8;
    Ok(IWord::from(if negative {
        -absolute_value
    } else {
        absolute_value
    }))
}

fn read_operand<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<Operand> {
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
            match read_char(chars)? {
                '%' => {
                    let low = read_hex_word(chars)?;
                    let high = read_hex_word(chars)?;
                    let op = Address::from_words(high, low);
                    let time = read_time(chars)?;

                    Operand::Ind(Timed::new(op, time))
                },
                _ => unimplemented!(),
            }
        }
        _ => {
            let register = read_register(chars)?;
            let time = read_time(chars)?;
            Operand::Reg(Timed::new(register, time))
        }
    };
    Ok(operand)
}

fn read_operands<'s>(chars: &mut Peekable<Chars<'s>>) -> ReadResult<Operands> {
    let src = read_operand(chars)?;
    let dest = read_operand(chars)?;
    Ok(Operands::new(src, dest))
}

/*// As cenas que fazem parse de um T retornam um par (T, resto da string)
type Cont<T> = (T, &str)

fn read_char(str: &str) -> Cont<char> {
    (str.chars().nth(0).unwrap(), &str[1..])
}

fn read_hex_word(word: &str) -> Cont<UWord> {
    todo!()
}

// Aqui pus ele a aceitar uma slice do resto da linha?
fn read_time(line: &str) -> Cont<IWord> {
    todo!()
}

// Epá nem sei como indexar um char, por causa do unicode e tudo mais
fn read_register(str: &str) -> Cont<Register> {
    match str[0] {
        'a' => Register::A, &str[1..]
        'b' => { match str[1]
            'h' => Register::BH, &str[2..]
            'l' => Register::BL, &str[2..]
        },
        'c' => { 
            match str.chars().nth(1).unwrap() {    
                'h' => (Register::CH, &str[2..]),
                'l' => (Register::CL, &str[2..]),
                _ => unreachable!(),
            }
        },
        'x' => (Register::X, &str[1..]),
        's' => (Register::SP, &str[2..]),
        _ => unreachable!(),
    }
}

fn read_operand(str: &str) -> Operand {
    use Operand::*;
    let (c, str) = read_char(str);
    match c {
        '#' => {
            let (word, str) = read_hex_word(str);
            Imm(word)
        },
        '%' => {
            let (low,  str) = read_hex_word(str);
            let (high, str) = read_hex_word(str);
            let op = Address{high, low};
            match str.get(0..2) {  // TODO tá-me a dar erro aqui e n sei pq
                Some (",X") => {
                    let (time, str) = read_time(&str[2..]);
                    Abx(Timed::<Address>{op, time})
                },
                None => {
                    let (time, str) = read_time(str);
                    Abs(Timed::<Address>{op, time})
                }
            }
        },
        '(' => {
            let (c, str) = read_char(str);
            match c {
                '%' => {
                    let (low,  str) = read_hex_word(str);
                    let (high, str) = read_hex_word(str);
                    let op = Address{high, low};
                    let (time, str) = read_time(str);
                    Ind(Timed::<Address>{op, time})
                },
                _ => {
                    panic!("Indirect register not implemented, please purchase Deluxe edition of this assembler.");
                }
            }
        },
        _ => {
            let (op, str) = read_register(str);
            let (time, str) = read_time(str);
            Reg(Timed::<Register>{op, time})
        }
    }
}

fn read_operands(str1: &str, str2: &str) -> Operands {
    let src = read_operand(str1);
    let dst = read_operand(str2);
    Operands{src, dst}
}

fn read_address(str: &str) -> Address {
    todo!()
}

fn read_offset(str: &str) -> Address {
    todo!()
}
*/
