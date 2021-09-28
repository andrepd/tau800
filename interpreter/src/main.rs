use std::io::Read;

mod prelude;
mod word;
mod state;
mod machine;
mod instruction;
mod interpreter;
mod assembler;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut state = machine::Machine::new();
    for i in assembler::assemble(buffer.as_str()) {
        eprint!("{:?}", i);
        instruction::Instruction::encode(&mut state, &i);
    }
    state.cpu = state::Cpu::default();

    loop {
        eprint!("{}", state);
        interpreter::step(&mut state);
    }
}
