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
    /*unsafe { interpreter::ZERO = word::UWord::from(0) };*/

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.to_lowercase(); // For flexibility

    // IO
    let mut clock_module = ClockModule;
    let mut display_module = DisplayModule::new();
    let mut io_modules =
        ModuleCollection::new(vec![Box::new(&mut clock_module), Box::new(&mut display_module)]);

    // Emulation

    let mut universe = Universe::new();
    assembler::assemble_into(universe.now_mut(), buffer.as_str());
    io_modules.run(&mut universe);

    for t in 0.. {
        // Run IO modules
        // @André: Não sei quais as consequências de não correr isto na fase de
        //         resolução, se pode causar inconsistência.
        // @Mike: Boa pergunta
        io_modules.run(&mut universe);

        {
            // Step the machine (manual loop)
            /*interpreter::step_micro(&mut universe);
            let mut iterations = 0;
            while !universe.is_consistent() {
                println!("{}", universe.now());
                // In resolution
                interpreter::step_micro(&mut universe);
                // println!("time resolution {}", universe.now());

                iterations += 1;
                if iterations > 100*10 {
                    panic!("Consistency failure.");
                }
            }
            let machine = universe.now();*/

            // Step the machine (auto loop)
            let (machine, instruction) = interpreter::step(&mut universe).expect("Consistency failure.");

            println!("t = {}", t);
            println!("instruction: {:?}", instruction);
            println!("{}", machine);

            match instruction { Instruction::Hcf => { println!("Execution ended."); break }, _ => () }

            println!("Display:");
            let words = &machine.ram.0[0x14..=0x1a];
            for d in 0usize..4 {
                // println!("{}", d);
                let mask = (1 << d) as u8;
                let f = |x,c| { if words[x as usize - 1].value() & mask != 0 {c} else {' '} };
                println!(" {} ", f(2,'—'));
                println!("{}{}{}", f(1,'|'), f(7,'_'), f(3,'|'));
                println!("{}{}{}", f(4,'|'), f(5,'_'), f(6,'|'));
                println!();
            }
        }
    };

    Ok(())
}
