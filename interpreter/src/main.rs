use std::io::Read;

mod prelude;
mod word;
mod state;
mod machine;
mod instruction;
mod interpreter;
mod assembler;
// mod universe;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.to_lowercase(); // For flexibility

    let mut state = machine::Machine::new();
    for i in assembler::assemble(buffer.as_str()) {
        eprint!("{:?}", i);
        instruction::Instruction::encode(&mut state, &i);
    }
    eprint!("\n");

    state.cpu = state::Cpu::default();

    /*loop*/ for _ in 0..100 {
        eprint!("{}\n", state);
        interpreter::step(&mut state);
    };

    Ok(())  // Sintaxe linda
}
