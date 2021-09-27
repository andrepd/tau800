use crate::prelude::*;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::instruction::Instruction;

struct InstructionIterator {

}

fn assemble(input: &mut BufReader<String>) -> InstructionIterator {
    todo!()
}

/*
// Quero que isto leia uma palavra da puta do iterator e o retorne, fds
fn read_mnemonic<It: Iterator>(words: It) -> String 
    where <It as Iterator>::Item : &str
{
    words.next().unwrap()/*.to_lowercase();*/
}

fn read_operand()(words: It) -> 

fn read_line(line: io::Result<String>) -> Instruction {
    let line: String = line.unwrap();
    let line = 
        match line.split_once(";") {
            Some ((prefix, suffix)) => String::from(prefix),
            None => line,
        };
    let words = line.split_whitespace();

    match read_mnemonic(words) with {
        "mov" => {
            let operands = read_operands(words);
        }
    }
}

fn assemble(input: &mut BufReader) -> Iterator<Instruction> {
    
}
 */