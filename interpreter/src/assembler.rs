use crate::instruction::{Instruction, Operand, Operands, Register};
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

    match mnemonic {
        _ => unimplemented!(),
    }
}

enum ReadError {
    NoMoreChars,
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

fn match_str<'s>(to_match: &str, chars: &mut Peekable<Chars<'s>>) -> ReadResult<bool> {
    for char in to_match.chars() {
        let peek = chars.peek();
        if peek.is_none() {
            return Err(ReadError::NoMoreChars);
        }
        if *peek.unwrap() == char {
            chars.next();
        } else {
            return Ok(false);
        }
    }
    Ok(true)
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
    todo!()
}

/*// As cenas que fazem parse de um T retornam um par (T, resto da string)
type Cont<T> = (T, &str)

fn read_char(str: &str) -> Cont<char> {
    str[0], &str[1..]
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
        'c' => { match str[1]
            'h' => Register::CH, &str[2..]
            'l' => Register::CL, &str[2..]
        },
        'x' => Register::X, &str[1..]
        's' => Register::SP, &str[2..]
    }
}

fn read_operand(str: &str) -> Operand {
    let c, str = read_char(str);
    match c {
        '#' => {
            let word, str = read_hex_word(str);
            Imm(word)
        },
        '%' => {
            let low,  str = read_hex_word(str);
            let high, str = read_hex_word(str);
            let op = Address{high, low};
            match str.get(0..2) {
                Some (",X") => {
                    let time, str = read_time(&str[2..]);
                    Abx({op, time})
                },
                None => {
                    let time, str = read_time(str);
                    Abs({op, time})
                }
            }
        },
        '(' => {
            let c, str = read_char(str);
            match c {
                '%' => {
                    let low,  str = read_hex_word(str);
                    let high, str = read_hex_word(str);
                    let op = Address{high, low};
                    let time, str = read_time(str);
                    Ind({op, time})
                },
                _ => {
                    panic!("Indirect register not implemented, please purchase Deluxe edition of this assembler.");
                }
            }
            let word = read_hex_word(str);
        },
        _ => {
            let op, str = read_register(str);
            let time = read_time(str);
            Reg({op, time})
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
