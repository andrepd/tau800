pub(crate) use crate::prelude::*;
use crate::modules::{ClockModule, DisplayModule, DiskModule, ModuleCollection};

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

    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.to_lowercase(); // For flexibility

    // IO
    let mut io_modules = {
        let clock_module = ClockModule;
        let display_module = DisplayModule::new();
        if let Some(fname) = std::env::args().nth(1) {
            let disk_module = DiskModule::new(fname).unwrap();
            ModuleCollection::new(vec![
                Box::new(clock_module), 
                Box::new(display_module),
                Box::new(disk_module),
            ])
        } else {
            ModuleCollection::new(vec![
                Box::new(clock_module), 
                Box::new(display_module),
            ])
        }
    };

    // Compilation

    let mut universe = Universe::new();
    assembler::assemble_into(universe.now_mut(), buffer.as_str());

    // For printing the punch cards: write the compiled program in binary to a file
    // Do this if the PUNCHCARD env. variable is set.
    if let Ok(_) = std::env::var("PUNCHCARD") {
        for word in universe.now().ram.0.iter() {
            println!("{:06b}" ,word.value());
        }
        return Ok(()); // Exit(0)
    }

    // Emulation

    io_modules.run(&mut universe);

    for t in 0.. {
        // Run IO modules
        // @André: Não sei quais as consequências de não correr isto na fase de
        //         resolução, se pode causar inconsistência.
        // @Mike: Boa pergunta
        // Mudei para dentro do step_micro no interpreter
        /*io_modules.run(&mut universe);*/

        {
            // Step the machine (auto loop)
            let (machine, instruction) = interpreter::step(&mut universe, &mut io_modules).expect("Consistency failure.");

            println!("t = {}", t);
            println!("instruction: {:?}", instruction);
            println!("{}", machine);

            if let Instruction::Hcf = instruction { println!("Execution ended."); break };

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
