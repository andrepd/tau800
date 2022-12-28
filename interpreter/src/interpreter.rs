use super::prelude::*;

//

fn operand_to_ref_inner<'a>(state: &'a Machine, op: &'a Op) -> &'a uWord {
    use Op::*;
    use Register::*;

    match op {
        Reg(A)  => &state.cpu.a,
        Reg(BH) => &state.cpu.bh,
        Reg(BL) => &state.cpu.bl,
        Reg(CH) => &state.cpu.ch,
        Reg(CL) => &state.cpu.cl,
        Reg(X)  => &state.cpu.x,
        Imm(op) => &op,
        Abs(op) => &state.ram[*op],
        Abx(op) => {
            let address = uLong::from(*op + state.cpu.x.value());
            &state.ram[address]
        }
        Ind(op) => {
            let lo = state.ram[*op];
            let hi = state.ram[usize::from(*op) + 1];
            let address = uLong::from_hi_lo(hi, lo);
            &state.ram[address]
        }
    }
}

fn operand_to_mut_ref_inner<'a>(state: &'a mut Machine, op: &'a Op) -> &'a mut uWord {
    use Op::*;
    use Register::*;

    match op {
        Reg(A)  => &mut state.cpu.a,
        Reg(BH) => &mut state.cpu.bh,
        Reg(BL) => &mut state.cpu.bl,
        Reg(CH) => &mut state.cpu.ch,
        Reg(CL) => &mut state.cpu.cl,
        Reg(X)  => &mut state.cpu.x,
        Imm(_)  => panic!("Illegal: attempted to write to immediate."),
        Abs(op) => &mut state.ram[*op],
        Abx(op) => {
            let address = uLong::from(*op + state.cpu.x.value());
            &mut state.ram[address]
        }
        Ind(op) => {
            let lo = state.ram[*op];
            let hi = state.ram[usize::from(*op) + 1];
            let address = uLong::from_hi_lo(hi, lo);
            &mut state.ram[address]
        }
    }
}

//

fn operand_get<'a>(universe: &'a mut Universe, operand: &'a Operand) -> &'a uWord {
    let t1 = universe.t - 1;  // -1: because we read from the state before execution and write to state after execution
    let t2 = t1 + operand.time;
    // Trivial reads (present or past)
    if operand.time.value() <= 0 {
        operand_to_ref_inner(&universe[t2], &operand.op)  //offbyone
    }
    // Reads from the future
    else {
        // Does that moment in the future not even exist? Then we need to run until it does and then check consistency
        if (t2) >= (universe.timeline.tf()) || universe.timeline.tf() == universe.t + 1 {
            universe.pending_reads.push((t2, t1, operand.op.clone(), uWord::lit(0)));  // Bootstrap with 0
            &uWord::ZERO
        }
        // If it already does
        else {
            let value = operand_to_ref_inner(&universe.timeline[t2], &operand.op);
            universe.pending_reads.push((t2, t1, operand.op.clone(), *value));
            value
        }
    }
}

fn operand_set<'a>(universe: &'a mut Universe, operand: &'a Operand, value: uWord) {
    let t1 = universe.t;
    let t2 = t1 + operand.time;
    // Trivial write (present)
    if operand.time.value() == 0 {
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
            /*universe.pending_writes.push((t2, operand.op.clone(), value));*/  // Put in pending writes anyway, in case we need to rewind further back
            ()  // ok
        } else {
            dprintln!("Inconsistent! Writing value {} to where was {}", value.value(), operand_to_ref_inner(&universe[t2], &operand.op).value());
            *operand_to_mut_ref_inner(&mut universe[t2], &operand.op) = value;
            universe.mode.add_inconsistent(t2 /*- 1*/, t1+4/*+1*/);
        }
    }
}

//

fn set_flag_z(state: &mut Machine, value: &uWord) {
    state.cpu.flags
        .write(Flag::Z, value.value() == 0)
}

fn set_flag_n(state: &mut Machine, value: &uWord) {
    state.cpu.flags
        .write(Flag::N, value.sign_bit())
}

fn set_flag_v(state: &mut Machine, value: &uWord) {
    // Hacker's delight: signed overflow occurs if carry = sign bit
    let carry = get_flag_c(state);
    state.cpu.flags
        .write(Flag::V, carry == value.sign_bit())
}

fn set_flag_nvz(state: &mut Machine, value: &uWord) {
    set_flag_n(state, value);
    set_flag_v(state, value);
    set_flag_z(state, value);
}

fn set_flag_c(state: &mut Machine, carry: bool) {
    state.cpu.flags.write(Flag::C, carry)
}

fn get_flag_c(state: &Machine) -> bool {
    state.cpu.flags.read(Flag::C)
}

//

// Aux function: from x return the lowest 6 bits as a word, and whether 
// any further bits are set
fn normalise(x: u8) -> (uWord, bool) {
    let result: u8 = x & uWord::MAX.value();//.into()
    let carry = x >> WORD_SIZE != 0;
    (uWord::lit(result), carry)
}

fn execute(state: &mut Universe, instruction: &Instruction) {
    let get = operand_get;
    let set = operand_set;

    match instruction {
        // Memory
        Instruction::Mov(Operands{ src, dst }) => {
            let word = *get(state, &src);
            set(state, &dst, word);
            set_flag_nvz(state.now_mut(), &word);
        }
        Instruction::Psh(x) => {
            let word = *get(state, &x);
            state.now_mut().write_sp(word);
        }
        Instruction::Pop(x) => {
            let word = state.now_mut().read_sp();
            set(state, &x, word);
            set_flag_nvz(state.now_mut(), &word);
        }

        // Arithmetic
        Instruction::Add(Operands { src, dst }) => {
            let a = get(state, &src).value();
            let b = get(state, &dst).value();
            let carry = u8::from(get_flag_c(state.now()));
            let (result, carry) = normalise(a + b + carry);
            set(state, &dst, result);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Sub(Operands { src, dst }) => {
            let a = get(state, &src).value();
            let b = get(state, &dst).value();
            let borrow = u8::from(!get_flag_c(state.now()));
            let (result, carry) = normalise((1<<WORD_SIZE) - a + b - borrow);
            set(state, &dst, result);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &result);
        }

        Instruction::Mul(Operands { src, dst }) => {
            let a = get(state, &src).value() as u16;
            let b = get(state, &dst).value() as u16;
            let result = uLong::try_from(a * b).unwrap().lo();
            set(state, &dst, result);
            /*set_flag_c(state.now_mut(), carry);*/
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Muh(Operands { src, dst }) => {
            let a = get(state, &src).value() as u16;
            let b = get(state, &dst).value() as u16;
            let raw = (a * b) >> WORD_SIZE;
            let (result, carry) = normalise(raw as u8);
            set(state, &dst, result);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Mus(Operands { src, dst }) => {
            let a = get(state, &src).as_iword().value() as i16;
            let b = get(state, &dst).as_iword().value() as i16;
            let raw = (a * b) >> WORD_SIZE;
            let (result, carry) = normalise(raw as u8);
            set(state, &dst, result);
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Div(Operands { src, dst }) => {
            if get(state, &src).value() != 0 {
                let a = get(state, &src).value();
                let b = get(state, &dst).value();
                let (result, _) = normalise(a / b);
                set(state, &dst, result);
                set_flag_z(state.now_mut(), &result);
            }
        }
        Instruction::Mod(Operands { src, dst }) => {
            if get(state, &src).value() != 0 {
                let a = get(state, &src).value();
                let b = get(state, &dst).value();
                let (result, _) = normalise(a % b);
                set(state, &dst, result);
                set_flag_z(state.now_mut(), &result);
            }
        }

        // Logic
        Instruction::And(Operands { src, dst }) => {
            let raw = get(state, &src).value() & get(state, &dst).value();
            let (result, _) = normalise(raw);
            set(state, &dst, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Or(Operands { src, dst }) => {
            let raw = get(state, &src).value() | get(state, &dst).value();
            let (result, _) = normalise(raw);
            set(state, &dst, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Xor(Operands { src, dst }) => {
            let raw = get(state, &src).value() ^ get(state, &dst).value();
            let (result, _) = normalise(raw);
            set(state, &dst, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Not(x) => {
            let raw = !get(state, &x).value();
            let (result, _) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Lsl(x) => {
            let raw = get(state, &x).value() << 1;
            let (result, carry) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
            set_flag_c(state.now_mut(), carry);
        }
        Instruction::Lsr(x) => {
            let a: u8 = get(state, &x).value();
            let carry = u8::from(get_flag_c(state.now())) << WORD_SIZE;
            let raw = (a >> 1) + carry;
            let (result, _) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Asr(x) => {
            let a: i8 = get(state, &x).as_iword().value();
            let carry = u8::from(get_flag_c(state.now())) << WORD_SIZE;
            let raw = (a >> 1) as u8 + carry;
            let (result, _) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Inc(x) => {
            let raw: u8 = get(state, &x).value() + 1;
            let (result, _) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Dec(x) => {
            let raw: u8 = get(state, &x).value() - 1;
            let (result, _) = normalise(raw);
            set(state, &x, result);
            set_flag_nvz(state.now_mut(), &result);
        }

        // Comparisons, CMP = SUB, BIT = AND, mas deitar fora os argumentos
        Instruction::Cmp(Operands { src, dst }) => {
            let a = get(state, &src).value();
            let b = get(state, &dst).value();
            let borrow = u8::from(!get_flag_c(state.now()));
            let (result, carry) = normalise(a + b - borrow);
            /*set(state, &dst, result);*/
            set_flag_c(state.now_mut(), carry);
            set_flag_nvz(state.now_mut(), &result);
        }
        Instruction::Bit(Operands { src, dst }) => {
            let raw = get(state, &src).value() & get(state, &dst).value();
            let (result, _) = normalise(raw);
            /*set(state, &dst, result);*/
            set_flag_nvz(state.now_mut(), &result);
        }

        // Jumping
        Instruction::Jmp(addr) => state.now_mut().cpu.pc = *addr,
        Instruction::Cal(addr) => {
            let current_addr = state.now().cpu.pc;
            state.now_mut().write_sp(current_addr.lo());
            state.now_mut().write_sp(current_addr.hi());
            state.now_mut().cpu.pc = *addr
        }
        Instruction::Ret => {
            let hi = state.now_mut().read_sp();
            let lo = state.now_mut().read_sp();
            state.now_mut().cpu.pc = Address::from_hi_lo(hi, lo)
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
        Instruction::Hcf => state.now_mut().cpu.pc = state.now().cpu.pc + (-1),  // Ugly hack
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
    //TODO mike pls fix
    let asdf = 
        universe.pending_writes.clone().into_iter().filter(|(t, op, value)| {
            if *t == universe.t {
                let state = universe.now_mut();
                *operand_to_mut_ref_inner(state, &op) = *value;
                false
            } else {
                true
            }
        });
    universe.pending_writes = asdf.collect::<Vec<_>>();
    /*universe.pending_writes.retain(|(t, op, value)| {
        if *t == universe.t {
            let state = universe.now_mut();
            *operand_to_mut_ref_inner(state, &op) = *value;
            false
        } else {
            true
        }
    });*/

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

    println!(">finish");
    println!("Instruction: {:?}", instruction);
    println!("{}", universe.now());

    ()
}

/*pub enum Result {
    /// Running, executed Instruction and updated to Machine
    Running (Machine, Instruction), 
    /// Cpu idle
    Halted (Machine), 
    /// Time inconsistency
    Inconsistent, 
}*/

/// Performs one full step on universe: micro steps until a fixed state can be yielded and pushed onto universe. 
/// Returns `false` iff inconsistency is reached
pub fn step_one(universe: &mut Universe, modules: &mut ModuleCollection) -> bool {
    /*dprintln!("step_one");*/
    // Fix memory leak: every 2^12 iterations clean unreachable in pending_{reads,writes}
    if universe.t % (1<<12) == 0 {
        let ti = universe.timeline.ti();
        universe.pending_reads.retain(|x| x.0 >= ti);  
        universe.pending_writes.retain(|x| x.0 >= ti);  
    }

    // Do step_micro until we hit inconsistency
    step_micro(universe, modules);
    const INCONSISTENT_ITERATIONS_LIMIT: usize = 1000*10;
    let mut inconsistent_iterations: usize = 0;
    while !universe.is_consistent() {
        if dbg!(inconsistent_iterations) == INCONSISTENT_ITERATIONS_LIMIT { return false }
        step_micro(universe, modules);
        inconsistent_iterations += 1
    };

    true
}

/// Returns Some(Machine, Instruction) or None if time inconsistency was reached
pub fn step(universe: &mut Universe, modules: &mut ModuleCollection) -> Option<(Machine, Instruction)> {
    /*dprintln!("step {} {}", universe.timeline.tf() - universe.timeline.ti(), universe.timeline.is_full());*/
    // Universe not full: continue filling
    while !universe.timeline.is_full() {
        /*dprintln!("foo {} {}", universe.timeline.ti(), universe.timeline.tf());*/
        if !step_one(universe, modules) { return None }
    } 
    // Universe full (and consistent): this means the state we pop from front is stable
    let machine = universe.pop_state();
    let instruction = Instruction::decode(&mut machine.clone());  // TODO overkill mas acho que não é bottleneck
    Some((machine, instruction))
}