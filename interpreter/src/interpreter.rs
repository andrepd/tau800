use crate::instruction::{Instruction, Op, Operand, Operands, Register};
use crate::prelude::*;
use crate::state::Flag;
use crate::universe::Universe;

//

fn operand_to_ref_inner<'a>(state: &'a Machine, op: &'a Op) -> &'a UWord {
    use Op::*;
    use Register::*;

    match op {
        Reg(A) => &state.cpu.a,
        Reg(BH) => &state.cpu.bh,
        Reg(BL) => &state.cpu.bl,
        Reg(CH) => &state.cpu.ch,
        Reg(CL) => &state.cpu.cl,
        Reg(X) => &state.cpu.x,
        Imm(op) => &op,
        Abs(op) => &state.ram[*op],
        Abx(op) => &state.ram[*op + state.cpu.x],
        Ind(op) => {
            let low = state.ram[*op];
            let high = state.ram[u16::from(*op) as usize + 1];
            let address = LongWord::<sig::Unsigned> { low, high };
            &state.ram[u16::from(address) as usize]
        }
    }
}

fn operand_to_mut_ref_inner<'a>(state: &'a mut Machine, op: &'a Op) -> &'a mut UWord {
    use Op::*;
    use Register::*;

    match op {
        Reg(A) => &mut state.cpu.a,
        Reg(BH) => &mut state.cpu.bh,
        Reg(BL) => &mut state.cpu.bl,
        Reg(CH) => &mut state.cpu.ch,
        Reg(CL) => &mut state.cpu.cl,
        Reg(X) => &mut state.cpu.x,
        Imm(_) => panic!("Illegal: attempted to write to immediate."),
        Abs(op) => &mut state.ram[*op],
        Abx(op) => &mut state.ram[*op + state.cpu.x],
        Ind(op) => {
            let low = state.ram[*op];
            let high = state.ram[u16::from(*op) as usize + 1];
            let address = LongWord::<sig::Unsigned> { low, high };
            &mut state.ram[u16::from(address) as usize]
        }
    }
}

//

fn operand_to_ref<'a>(universe: &'a mut Universe, operand: &'a Operand) -> &'a UWord {
    // Trivial reads
    /*if operand.time.value() <= 0 {*/
    if dbg!(universe.t + &operand.time - 1) < dbg!(universe.states.len()) {
        operand_to_ref_inner(universe.t_offset(&operand.time), &operand.op)
    }
    // Reads into the future
    else {
        // Are we already in resolution?
        let target_to_match = &universe.target;
        match target_to_match {
            None => {
                let ti = universe.t - 1;
                let tf = universe.t + operand.time.value() as usize;
                // @André: Aqui tens universe.states[universe.t - 1], mas universe.now() é [universe.t].
                //         Qual a convenção? universe.t antes de actuar?
                let guess = operand_to_ref_inner(&universe.states[ti], &operand.op);
                universe.target = Some((ti, tf, operand.op.clone(), guess.clone()));
                universe.guess = guess.clone();
                guess
            }
            Some((_ti, _tf, _op, _guess)) => {
                /*debug_assert!();*/
                //&state.target.unwrap().3 //&guess
                &universe.guess
            }
        }
    }
}

fn operand_to_mut_ref<'a>(universe: &'a mut Universe, operand: &'a Operand) -> &'a mut UWord {
    if operand.time.value() == 0 {
        /*operand_to_mut_ref_inner(state.t_offset(operand.time), &operand.op)*/
        operand_to_mut_ref_inner(universe.now_mut(), &operand.op)
    } else {
        panic!()
    }
}

//

fn set_flag_z(state: &mut Machine, value: &UWord) {
    state
        .cpu
        .flags
        .write(Flag::Z, /*dbg!*/ (value.value()) == 0) // TODO: Ele aqui diz que value é 0 mas a flag não está a ficar set...
}

fn set_flag_n(state: &mut Machine, value: &UWord) {
    /*state.cpu.flags.write(Flag::N, value.value() < 0)*/
    state
        .cpu
        .flags
        .write(Flag::N, value.value() & 0b100000 != 0)
}

fn set_flag_v(state: &mut Machine, op1: u8, op2: u8, result: u8) {
    // Signed overflow of operands have same sign and that sign is different from value
    state
        .cpu
        .flags
        .write(Flag::V, (op1 ^ result) & (op2 ^ result) & 0b100000 != 0)
}

fn set_flag_v_dummy(state: &mut Machine, value: &UWord) {
    state
        .cpu
        .flags
        .write(Flag::V, value.value() & 0b010000 != 0)
}

fn set_flag_nvz(state: &mut Machine, value: &UWord) {
    set_flag_n(state, value);
    set_flag_v_dummy(state, value);
    set_flag_z(state, value);
}

fn set_flag_c(state: &mut Machine, carry: bool) {
    state.cpu.flags.write(Flag::C, carry)
}

fn get_flag_c(state: &Machine) -> bool {
    state.cpu.flags.read(Flag::C)
}

//

fn execute(state: &mut Universe, instruction: &Instruction) {
    let mk_ref = operand_to_ref;
    let mk_mref = operand_to_mut_ref;

    match instruction {
        // Memory
        Instruction::Mov(Operands { src, dst }) => {
            let word = *mk_ref(state, &src);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Psh(x) => {
            let word = *mk_ref(state, &x);
            state.now_mut().write_sp(word);
        }
        Instruction::Pop(x) => {
            let word = state.now_mut().read_sp();
            *mk_mref(state, &x) = word;
            set_flag_nvz(state.now_mut(), &word);
        }

        // Arithmetic
        Instruction::Add(Operands { src, dst }) => {
            let src_orig = mk_ref(state, &src).value();
            let dst_orig = mk_ref(state, &dst).value();
            let result = src_orig + dst_orig + u8::from(get_flag_c(state.now()));
            let (div, rem) = div_rem(result, MAX_UNSIGNED_VALUE + 1);
            let carry = div > 0;
            let word = UWord::from(rem);
            *mk_mref(state, &dst) = word;
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &word);
            set_flag_v(state.now_mut(), src_orig, dst_orig, rem);
        }
        Instruction::Sub(Operands { src, dst }) => {
            let src_orig = mk_ref(state, &src).value();
            let dst_orig = mk_ref(state, &dst).value();
            let result =
                MAX_UNSIGNED_VALUE + 1 + dst_orig - src_orig - u8::from(!get_flag_c(state.now()));
            let (div, rem) = div_rem(result, MAX_UNSIGNED_VALUE + 1);
            let carry = div > 0;
            let word = UWord::from(rem);
            *mk_mref(state, &dst) = word;
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &word);
            set_flag_v(state.now_mut(), !src_orig, dst_orig, rem);
        }

        Instruction::Mul(Operands { src, dst }) => {
            let result = mk_ref(state, &dst).value() * mk_ref(state, &src).value();
            let word = UWord::from(result % (MAX_UNSIGNED_VALUE + 1));
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Muh(Operands { src, dst }) => {
            // Multiply normally
            let result: u16 =
                mk_ref(state, &dst).value() as u16 * mk_ref(state, &src).value() as u16;
            let word = UWord::from((result >> WORD_SIZE) as u8);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Mus(Operands { src, dst }) => {
            // First sign-extend to 12-bit two's complement integer
            fn sign_extend(x: u8) -> u16 {
                if x & 0b100_000 == 0 {
                    x as u16
                } else {
                    0b111_111_000_000 | (x as u16)
                }
            }
            let result =
                sign_extend(mk_ref(state, &dst).value()) * sign_extend(mk_ref(state, &src).value());
            let word = UWord::from((result >> WORD_SIZE) as u8);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Div(Operands { src, dst }) => {
            let result = mk_ref(state, &dst).value() / mk_ref(state, &src).value();
            let word = UWord::from(result);
            *mk_mref(state, &dst) = word;
            set_flag_z(state.now_mut(), &word);
        }
        Instruction::Mod(Operands { src, dst }) => {
            let result = mk_ref(state, &dst).value() / mk_ref(state, &src).value();
            let word = UWord::from(result);
            *mk_mref(state, &dst) = word;
            set_flag_z(state.now_mut(), &word);
        }

        // Logic
        Instruction::And(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() & mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Or(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() | mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Xor(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() ^ mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Not(x) => {
            let result = !mk_ref(state, &x).value();
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Lsl(x) => {
            let result = mk_ref(state, &x).value() << 1;
            let carry = (0b01000000 & result) != 0;
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state.now_mut(), &word);
            set_flag_c(state.now_mut(), carry);
        }
        Instruction::Lsr(x) => {
            let result = mk_ref(state, &x).value() >> 1;
            let word = UWord::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state.now_mut(), &word);
        }

        // Comparisons, CMP = SUB, BIT = AND, mas deitar fora os argumentos
        Instruction::Cmp(Operands { src, dst }) => {
            let src_orig = mk_ref(state, &src).value();
            let dst_orig = mk_ref(state, &dst).value();
            let result =
                MAX_UNSIGNED_VALUE + 1 + dst_orig - src_orig - u8::from(!get_flag_c(state.now()));
            let (div, rem) = div_rem(result, MAX_UNSIGNED_VALUE + 1);
            let carry = div > 0;
            let word = UWord::from(rem);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &word);
            set_flag_v(state.now_mut(), !src_orig, dst_orig, rem);
        }

        Instruction::Bit(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() & mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            set_flag_nvz(state.now_mut(), &word);
        }

        // Jumping
        Instruction::Jmp(addr) => state.now_mut().cpu.pc = *addr,
        Instruction::Cal(addr) => {
            let current_addr = state.now().cpu.pc;
            state.now_mut().write_sp(current_addr.low);
            state.now_mut().write_sp(current_addr.high);
            state.now_mut().cpu.pc = *addr
        }
        Instruction::Ret => {
            let high = state.now_mut().read_sp();
            let low = state.now_mut().read_sp();
            state.now_mut().cpu.pc = Address { high, low }
        }

        // Branching
        Instruction::Bcc(x) => {
            if state.now().cpu.flags.read(Flag::C) == false {
                /*state.cpu.pc += x*/
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }
        Instruction::Bcs(x) => {
            if state.now().cpu.flags.read(Flag::C) == true {
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }
        Instruction::Bne(x) => {
            if state.now().cpu.flags.read(Flag::Z) == false {
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }
        Instruction::Beq(x) => {
            if state.now().cpu.flags.read(Flag::Z) == true {
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }
        Instruction::Bpl(x) => {
            if state.now().cpu.flags.read(Flag::N) == false {
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }
        Instruction::Bmi(x) => {
            if state.now().cpu.flags.read(Flag::N) == true {
                state.now_mut().cpu.pc = state.now().cpu.pc + *x
            }
        }

        // Misc
        Instruction::Clc => state.now_mut().cpu.flags.write(Flag::C, false),
        Instruction::Sec => state.now_mut().cpu.flags.write(Flag::C, true),
        Instruction::Nop => (),
    }
}

pub fn step(universe: &mut Universe) {
    eprintln!("Step: t={} target={:?}", universe.t, universe.target);

    let target_to_match = universe.target.clone();
    match target_to_match {
        // Normal execution
        None => (),
        // Time resolution
        Some((ti, tf, op, guess)) => {
            // Target time
            if tf == universe.t {
                let value = operand_to_ref_inner(universe.now(), &op).clone();
                // Fixed point: we're done, go back to ti with the correct result
                if dbg!(value) == dbg!(guess) {
                    universe.rewind_keep(ti);
                    universe.target = None;
                }
                // No fixed point: go back to ti, destroying this timeline, try again with guess=value
                else {
                    universe.rewind_destroy(ti);
                    universe.target = Some((ti, tf, op.clone(), value));
                    universe.guess = value;
                }
            }
            // Running the resolution
            else {
                ()
            }
        }
    }

    eprintln!("=>    t={} target={:?}", universe.t, universe.target);

    universe.push_new_state();
    let instruction = Instruction::decode(universe.now_mut());
    execute(universe, &instruction);

    eprintln!("=>    t={} target={:?}", universe.t, universe.target);
}
