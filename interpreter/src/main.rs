use crate::{
    instruction::Instruction,
    modules::{ClockModule, DisplayModule, Module, ModuleCollection},
};
use core::panic;
use std::io::Read;
use universe::Universe;

mod assembler;
mod instruction;
mod interpreter;
mod machine;
mod modules;
mod prelude;
mod state;
mod universe;
mod word;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.to_lowercase(); // For flexibility

    // IO
    let mut io_modules =
        ModuleCollection::new(vec![Box::new(ClockModule), Box::new(DisplayModule::new())]);

    // Emulation

    let mut universe = Universe::new();
    assembler::assemble_into(universe.now_mut(), buffer.as_str());
    io_modules.run(&mut universe);

    println!("{}", universe.now());
    for _ in 0..100 {
        // Run IO modules
        // @André: Não sei quais as consequências de não correr isto na fase de
        //         resolução, se pode causar inconsistência.
        io_modules.run(&mut universe);

        // Step the machine
        {
            interpreter::step(&mut universe);
            let mut iterations = 0;
            while universe.target.is_some() {
                // In resolution
                interpreter::step(&mut universe);

                iterations += 1;
                if iterations > 100 {
                    panic!("Consistency failure.");
                }
            }
            println!("{}", universe.now());
        }
    }

    Ok(())
}
