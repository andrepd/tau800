use crate::prelude::*;

fn step(state: &mut MachineState) {
    use crate::instruction::{Instruction, Offset, Operand, Operands};

    let instruction = Instruction::decode(&state.ram, &mut state.cpu.pc);
    match instruction {
        Instruction::Mov(op) => match op {
            Operands::RegReg { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::ImmReg { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::AbsReg { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::IndReg { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::RegAbs { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::RegInd { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::ImmAbs { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
            Operands::ImmInd { src, dst } => *(dst.to_mut_ref()) = src.to_ref(),
        },
    }
}

fn run(state: &mut MachineState) {
    loop {
        step(state)
    }
}
