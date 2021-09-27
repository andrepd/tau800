use std::str::Lines;
use crate::instruction::{Instruction, Operand, Operands, Register};

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

//

fn read_operand(word: &str) -> Operand {
    match word[0] {
        '#' => {
            let word = read_hex_word(&word[1..3]);
            Imm(word)
        },
        '%' => {
            let low = read_hex_word(&word[1..3]);
            let high = read_hex_word(&word[3..5]);
            let op = Address{high, low};
            match word.get(5..7) {
                Some (",X") => {
                    let time = try_read_time(word[7..]);
                    Abx({op, time})
                },
                None => {
                    let time = try_read_time(word[5..]);
                    Abs({op, time})
                }
            }
        },
        '(' => {
            match word[1] {
                '%' => {
                    let low = read_hex_word(&word[2..4]);
                    let high = read_hex_word(&word[4..6]);
                    let time = try_read_time(word[7..]);
                    Ind({op, time})
                },
                _ => {
                    panic!("Indirect register not implemented, please purchase Deluxe edition of this assembler.");
                }
            }
            let word = read_hex_word(&word[1..3]);
        },
        _ => {
            let op = read_register(word[0]);
            let time = try_read_time(word[1..]);
            Reg({op, time})
        }
    }
}

// Epá nem sei como indexar um char, por causa do unicode e tudo mais
fn read_register(word: &str) -> Register {
    match word[0] {
        'a' => Register::A,
        'b' => { match word[1] 
            'h' => Register::BH,
            'l' => Register::BL,
        },
        'c' => { match word[1]
            'h' => Register::CH,
            'l' => Register::CL,
        },
        'x' => Register::X,
        's' => Register::SP,
    }
}

fn read_operands(word1: &str, word2: &str) -> Operands {
    todo!()
}

fn read_hex_word(word: &str) -> UWord {
    todo!()
}

// Aqui pus ele a aceitar uma slice do resto da linha?
fn try_read_time(line: &str) -> IWord {
    todo!()
}
