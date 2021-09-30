use core::panic;
use std::io::Read;

use universe::Universe;

use crate::instruction::Instruction;

mod prelude;
mod word;
mod state;
mod machine;
mod instruction;
mod interpreter;
mod assembler;
mod universe;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.to_lowercase();  // For flexibility

    /*let mut universe = Universe::bootstrap(&buffer);
    println!("{:}", universe.now());
    
    for _i in 0..100 {
        universe = universe.step().unwrap();
        println!("{:}", universe.now());
    }*/

    let mut universe = Universe::new();
    assembler::assemble_into(universe.now_mut(), buffer.as_str());
    
    println!("{}", universe.now());
    for _ in 0..100 {
        interpreter::step(&mut universe);
        while universe.target.is_some() { // In resolution
            interpreter::step(&mut universe);
        }
        println!("{}", universe.now());
        
    };

    Ok(())
}
