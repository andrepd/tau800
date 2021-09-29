use std::io::Read;

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
    let buffer = buffer.to_lowercase(); // For flexibility

    Ok(())
}
