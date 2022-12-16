use super::prelude::*;

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
            let address = ULongWord {low, high};
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
            let address = ULongWord {low, high};
            &mut state.ram[u16::from(address) as usize]
        }
    }
}

//

fn operand_get<'a>(universe: &'a mut Universe, operand: &'a Operand) -> &'a UWord {
    let delta_t = operand.time.value();
    let t1 = universe.t - 1;  // -1: because we read from the state before execution and write to state after execution
    let t2 = t1 + &operand.time;
    // Trivial reads (present or past)
    if delta_t <= 0 {
        operand_to_ref_inner(&universe[t2], &operand.op)  //offbyone
    }
    // Reads from the future
    else {
        // Does that moment in the future not even exist? Then we need to run until it does and then check consistency
        if (t2) >= (universe.timeline.tf()) || universe.timeline.tf() == universe.t + 1 {
            universe.pending_reads.push((t2, t1, operand.op.clone(), UZERO));  // Bootstrap with 0
            &UZERO
        }
        // If it already does
        else {
            let value = operand_to_ref_inner(&universe.timeline[t2], &operand.op);
            universe.pending_reads.push((t2, t1, operand.op.clone(), *value));
            value
        }
    }
}

fn operand_set<'a>(universe: &'a mut Universe, operand: &'a Operand, value: UWord) {
    let delta_t = operand.time.value();
    let t1 = universe.t;
    let t2 = t1 + &operand.time;
    // Trivial write (present)
    if delta_t == 0 {
        *operand_to_mut_ref_inner(universe.now_mut(), &operand.op) = value;
    }
    // Trivial write (future, add to pending writes)
    else if operand.time.value() > 0 {
        universe.pending_writes.push((t2, operand.op.clone(), value));
    }
    // Non-trivial write (past)
    else {
        // Is this inconsistent with what was already recorded?
        if *operand_to_ref_inner(&universe[t2], &operand.op) == value {
            universe.pending_writes.push((t2, operand.op.clone(), value));  // Put in pending writes anyway, in case we need to rewind further back
            ()  // ok
        } else {
            *operand_to_mut_ref_inner(&mut universe[t2], &operand.op) = value;
            universe.mode.add_inconsistent(t2 /*- 1*/, t1/*+1*/);
        }
    }
}

//

fn set_flag_z(state: &mut Machine, value: &UWord) {
    state
        .cpu
        .flags
        .write(Flag::Z, value.value() == 0)
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
    let mk_ref = operand_get;
    let mk_mref = operand_set;

    match instruction {
        // Memory
        Instruction::Mov(Operands { src, dst }) => {
            let word = *mk_ref(state, &src);
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Psh(x) => {
            let word = *mk_ref(state, &x);
            state.now_mut().write_sp(word);
        }
        Instruction::Pop(x) => {
            let word = state.now_mut().read_sp();
            mk_mref(state, &x, word);
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
            mk_mref(state, &dst, word);
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
            mk_mref(state, &dst, word);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &word);
            set_flag_v(state.now_mut(), !src_orig, dst_orig, rem);
        }

        Instruction::Mul(Operands { src, dst }) => {
            let result = mk_ref(state, &dst).value() * mk_ref(state, &src).value();
            let word = UWord::from(result % (MAX_UNSIGNED_VALUE + 1));
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Muh(Operands { src, dst }) => {
            // Multiply normally
            let result: u16 =
                mk_ref(state, &dst).value() as u16 * mk_ref(state, &src).value() as u16;
            let word = UWord::from((result >> WORD_SIZE) as u8);
            mk_mref(state, &dst, word);
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
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Div(Operands { src, dst }) => {
            if mk_ref(state, &src).value() != 0 {
                let result = (mk_ref(state, &dst).value() + u8::from(get_flag_c(state.now())) * 0b01000000) / mk_ref(state, &src).value();
                let word = UWord::from(result);
                mk_mref(state, &dst, word);
                set_flag_z(state.now_mut(), &word);
            }
        }
        Instruction::Mod(Operands { src, dst }) => {
            if mk_ref(state, &src).value() != 0 {
                let result = (mk_ref(state, &dst).value() + u8::from(get_flag_c(state.now())) * 0b01000000) % mk_ref(state, &src).value();
                let word = UWord::from(result);
                mk_mref(state, &dst, word);
                set_flag_z(state.now_mut(), &word);
            }
        }

        // Logic
        Instruction::And(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() & mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Or(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() | mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Xor(Operands { src, dst }) => {
            let result = mk_ref(state, &src).value() ^ mk_ref(state, &dst).value();
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Not(x) => {
            let result = !mk_ref(state, &x).value();
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Lsl(x) => {
            let result = mk_ref(state, &x).value() << 1;
            let carry = (0b01000000 & result) != 0;
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
            set_flag_c(state.now_mut(), carry);
        }
        Instruction::Lsr(x) => {
            let carry = 0b00100000 * u8::from(get_flag_c(state.now()));
            let result = (mk_ref(state, &x).value() >> 1) + carry;
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Asr(x) => {
            let carry = 0b00100000 * u8::from(get_flag_c(state.now()));
            let result = (mk_ref(state, &x).cast_to_signed().value() >> 1) as u8 + carry;
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Inc(x) => {
            let result = mk_ref(state, &x).value() + 1;
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Dec(x) => {
            let result = mk_ref(state, &x).value() + MAX_UNSIGNED_VALUE + 1 - 1;
            let word = UWord::from(result & 0b00111111);
            mk_mref(state, &x, word);
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
        Instruction::Hcf => state.now_mut().cpu.pc = state.now().cpu.pc + IWord::from(-1),  // Ugly hack
    }
}

/// Performs one micro step on `universe`
pub fn step_micro(universe: &mut Universe, modules: &mut ModuleCollection) {
    dprintln!("μStep: t={} mode={:?}", universe.t, universe.mode);

    // Pending reads, são aqui que se checam
    /*universe.pending_reads.retain(|&x| {*/
    let pending_reads_ = 
        universe.pending_reads.clone().into_iter().filter(|(t, ti, op, value)| {
            if *t == universe.t {
                let state = universe.now();
                if *operand_to_ref_inner(state, &op) == *value {
                    false//true
                } else {
                    universe.mode.add_inconsistent(*ti, *t);
                    false
                }
            } else {
                true
            }
        });
    universe.pending_reads = pending_reads_.collect::<Vec<_>>();  // ::<_>>()#@__zyx$$&ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn

    dprintln!(">read   t={} mode={:?}", universe.t, universe.mode);

    universe.push_new_state();

    dprintln!(">push  t={} mode={:?}", universe.t, universe.mode);

    modules.run(universe);
    let instruction = Instruction::decode(universe.now_mut());
    execute(universe, &instruction);

    dprintln!(">exec  t={} mode={:?}", universe.t, universe.mode);

    // Pending writes, são aqui que se fazem
    for (t, op, value) in &universe.pending_writes.clone() {
        if *t == universe.t {
            let state = universe.now_mut();
            *operand_to_mut_ref_inner(state, &op) = *value
        }
    }

    match universe.mode {
        // Maybe inconsistent: if we reach the end of the window, it is consistent
        Mode::Maybe (ti, tf) if universe.t == tf => {
            universe.mode = Mode::Consistent;
            dprintln!(">mode  t={} mode={:?}", universe.t, universe.mode);
            return
        }
        // Definitely inconsistent: if we reach the end of the window, rewind to start as "maybe consistent"
        Mode::Inconsistent (ti, tf) if universe.t == tf => {
            universe.mode = Mode::Maybe(ti, tf);
            universe.t = ti;
            dprintln!(">mode  t={} mode={:?}", universe.t, universe.mode);
            return
        }
        // Anything else: continue execution
        _ => ()
    }

    dprintln!(">mode  t={} mode={:?}", universe.t, universe.mode);

    dprintln!(">writ   t={} mode={:?}", universe.t, universe.mode);

    if cfg!(debug_assertions) {
        for i in &universe.pending_writes { dprintln!("pending w: {:?}", i) };
        for i in &universe.pending_reads { dprintln!("pending r: {:?}", i) };
    }

    ()
}

/// Performs one full step on universe: micro steps until a fixed state can be yielded. 
/// Returns Some(Machine, Instruction) or None if time inconsistency was reached
pub fn step(universe: &mut Universe, modules: &mut ModuleCollection) -> Option<(Machine, Instruction)> {
    // Fix memory leak: every 4000 iterations clean unreachable in pending_{reads,writes}
    if universe.t % 4000 == 0 {
        let ti = universe.timeline.ti();
        universe.pending_reads.retain(|x| x.0 >= ti);  
        universe.pending_writes.retain(|x| x.0 >= ti);  
    }

    const INCONSISTENT_ITERATIONS_LIMIT: usize = 1000;
    let mut inconsistent_iterations: usize = 0;
    while !universe.is_consistent() || !universe.timeline.is_full() {
        step_micro(universe, modules);
        if !universe.is_consistent() { inconsistent_iterations += 1 };
        if inconsistent_iterations == INCONSISTENT_ITERATIONS_LIMIT { return None }
    }

    // Now universe is consistent and full. Which means the state we pop from front is stable
    let machine = universe.pop_state();
    let instruction = Instruction::decode(&mut machine.clone());  // TODO overkill mas acho que não é bottleneck

    Some((machine, instruction))
}