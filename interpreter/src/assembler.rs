use std::str::Lines;
use crate::instruction::Instruction;

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
