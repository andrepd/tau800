use std::io::Read;

use universe::Universe;

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
    let mut universe = Universe::bootstrap(&buffer);

    for _i in 0..100 {
        universe = universe.step().unwrap();
        println!("{:}", universe.now());
    }

    Ok(())
}
