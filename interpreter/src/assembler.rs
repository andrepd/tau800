use crate::prelude::*;
use std::str::Lines;
use crate::instruction::{Instruction, Operand, Operands, Register, Timed};



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

//

// As cenas que fazem parse de um T retornam um par (T, resto da string)
type Cont<'a, T> = (T, &'a str);

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
    match str.chars().nth(0).unwrap() {  // le mao
        'a' => (Register::A, &str[1..]),
        'b' => { 
            match str.chars().nth(1).unwrap() {
                'h' => (Register::BH, &str[2..]),
                'l' => (Register::BL, &str[2..]),
                _ => unreachable!(),
            }
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
