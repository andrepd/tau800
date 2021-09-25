use crate::prelude::*;
use crate::instruction::{Instruction, Offset, Operand, Operands, Register, Timed};
// use crate::prelude::sig::Unsigned;
use crate::state::Flag;



// De momento tá a ignorar o time, depois temos de fazer um struct estado total que tem 
// os estados em todos os momentos.
// fn operand_to_ref<'a, Signedness: sig::Signature>(state: &'a Machine, operand: &Operand) -> &'a Word<Signedness> {
fn operand_to_ref<'a>(state: &'a Machine, operand: &'a Operand) -> &'a UWord {
    use Register as R;
    type TR = Timed<Register>;
    type TA = Timed<Address>;
    use Operand::*;

    match operand {
        Reg(TR{op: R::A,  time}) => &state.cpu.a,
        /*Reg({op=F; time}) => &state.cpu.flags,*/
        Reg(TR{op: R::BH, time}) => &state.cpu.bh,
        Reg(TR{op: R::BL, time}) => &state.cpu.bl,
        Reg(TR{op: R::CH, time}) => &state.cpu.ch,
        Reg(TR{op: R::CL, time}) => &state.cpu.cl,
        Reg(TR{op: R::X,  time}) => &state.cpu.x,
        Reg(TR{op: R::SP, time}) => &state.cpu.sp,

        Imm(word)  => &word,
        /*Iml(dword) => &dword,*/

        Abs(TA{op, time}) => &state.ram[*op], 
        Abx(TA{op, time}) => &state.ram[*op + state.cpu.x], 

        Ind(TA{op, time}) => {
            let low  = state.ram[*op];
            let high = state.ram[u16::from(*op) as usize + 1];
            let address = LongWord::<sig::Unsigned> { low, high };
            &state.ram[u16::from(address) as usize]
        }

        /*Inr(A{op, time}) => &state.ram[state.ram[]], */
    }
}

fn operand_to_mut_ref<'a>(state: &'a mut Machine, operand: &'a Operand) -> &'a mut UWord {
    use Register as R;
    type TR = Timed<Register>;
    type TA = Timed<Address>;
    use Operand::*;

    match operand {
        Reg(TR{op: R::A,  time}) => &mut state.cpu.a,
        /*Reg({op=F; time}) => &mut state.cpu.flags,*/
        Reg(TR{op: R::BH, time}) => &mut state.cpu.bh,
        Reg(TR{op: R::BL, time}) => &mut state.cpu.bl,
        Reg(TR{op: R::CH, time}) => &mut state.cpu.ch,
        Reg(TR{op: R::CL, time}) => &mut state.cpu.cl,
        Reg(TR{op: R::X,  time}) => &mut state.cpu.x,
        Reg(TR{op: R::SP, time}) => &mut state.cpu.sp,

        Imm(word)  => unreachable!(),
        /*Iml(dword) => unreachable!(),*/

        Abs(TA{op, time}) => &mut state.ram[*op], 
        Abx(TA{op, time}) => &mut state.ram[*op + state.cpu.x], 

        Ind(TA{op, time}) => {
            let low  = state.ram[*op];
            let high = state.ram[u16::from(*op) as usize + 1];
            let address = LongWord::<sig::Unsigned> { low, high };
            /*&mut state.ram[u16::from(address) as usize]*/
            unimplemented!()  // doesnt work lol
        }

        /*Inr(A{op, time}) => &state.ram[state.ram[]], */
    }
}



fn set_flag_z(state: &mut Machine, value: &UWord) {
    state.cpu.flags.write(Flag::Z, value.value() == 0)
}

fn set_flag_n(state: &mut Machine, value: &UWord) {
    state.cpu.flags.write(Flag::Z, value.value() < 0)
}

fn set_flag_v(state: &mut Machine, value: &UWord) {
    unimplemented!();  // Pá ainda não tenho bem a certeza como se implementa isto
}

fn set_flag_nvz(state: &mut Machine, value: &UWord) {
    set_flag_n(state, value);
    set_flag_v(state, value);
    set_flag_z(state, value);
}



fn execute(state: &mut Machine, instruction: &Instruction) {
    // Epá não há maneira mais fácil de fazer isto? nome_curto = nome_gigante?
    fn mk_ref<'a>(state: &'a Machine, operand: &'a Operand) -> &'a UWord {
        operand_to_ref(state, operand)
    }
    fn mk_mref<'a>(state: &'a mut Machine, operand: &'a Operand) -> &'a mut UWord {
        operand_to_mut_ref(state, operand)
    }

    match instruction {
        // Memory
        Instruction::Mov(Operands{src, dst}) => {
            *mk_mref(state, &dst) = *mk_ref(state, &src)
        },
        // Onde começa o stack? Para onde cresce? É preciso ver
        Instruction::Psh(x) => {
            unimplemented!()
        },
        Instruction::Pop(x) => {
            unimplemented!()
        },

        // Arithmetic
        Instruction::Add(Operands{src, dst}) => {
            let result = mk_ref(state, &src).value() + mk_ref(state, &dst).value() + u8::from(state.cpu.flags.read(Flag::C));
            let (div, rem) = div_rem(result, MAX_UNSIGNED_VALUE);
            let carry = div > 0;
            let word = Word::<sig::Unsigned>::from(rem);
            *mk_mref(state, &dst) = word;
            state.cpu.flags.write(Flag::C, carry);
            set_flag_nvz(state, &word);
        },
        Instruction::Sub(Operands{src, dst}) => {
            unimplemented!()
        },

        Instruction::Mul(_) | 
        Instruction::Mus(_) | 
        Instruction::Div(_) | 
        Instruction::Dis(_) | 
        Instruction::Mod(_) | 
        Instruction::Mos(_) => unimplemented!(), 

        // Logic
        Instruction::And(Operands{src, dst}) => {
            let result = mk_ref(state, &src).value() & mk_ref(state, &dst).value();
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state, &word);
        },
        Instruction::Or(Operands{src, dst}) => {
            let result = mk_ref(state, &src).value() | mk_ref(state, &dst).value();
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state, &word);
        },
        Instruction::Xor(Operands{src, dst}) => {
            let result = mk_ref(state, &src).value() ^ mk_ref(state, &dst).value();
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &dst) = word;
            set_flag_nvz(state, &word);
        },
        Instruction::Not(x) => {
            let result = !mk_ref(state, &x).value();
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state, &word);
        },
        Instruction::Lsl(x) => {
            let result = mk_ref(state, &x).value() << 1;
            let carry = (0b01000000 & result) != 0;
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state, &word);
            state.cpu.flags.write(Flag::C, carry);
        },
        Instruction::Lsr(x) => {
            let result = mk_ref(state, &x).value() >> 1;
            let word = Word::<sig::Unsigned>::from(result & 0b00111111);
            *mk_mref(state, &x) = word;
            set_flag_nvz(state, &word);
        },

        // Comparisons, CMP = SUB, BIT = AND, mas deitar fora os argumentos
        Instruction::Cmp(_) | Instruction::Bit(_) => unimplemented!(),

        // Jumping
        Instruction::Jmp(addr) => {
            unimplemented!()
        },
        Instruction::Cal(addr) => {
            unimplemented!()
        },
        Instruction::Ret => {
            unimplemented!()
        },

        // Branching
        Instruction::Bcc(x) => {
            if state.cpu.flags.read(Flag::C) == false {
                /*state.cpu.pc += x*/
                state.cpu.pc = state.cpu.pc + *x
            }
        },
        Instruction::Bcs(x) => {
            if state.cpu.flags.read(Flag::C) == true {
                state.cpu.pc = state.cpu.pc + *x
            }
        },
        Instruction::Bne(x) => {
            if state.cpu.flags.read(Flag::Z) == false {
                state.cpu.pc = state.cpu.pc + *x
            }
        },
        Instruction::Beq(x) => {
            if state.cpu.flags.read(Flag::Z) == true {
                state.cpu.pc = state.cpu.pc + *x
            }
        },
        Instruction::Bpl(x) => {
            if state.cpu.flags.read(Flag::N) == false {
                state.cpu.pc = state.cpu.pc + *x
            }
        },
        Instruction::Bmi(x) => {
            if state.cpu.flags.read(Flag::N) == true {
                state.cpu.pc = state.cpu.pc + *x
            }
        },

        // Misc
        Instruction::Clc => {
            state.cpu.flags.write(Flag::C, false)
        },
        Instruction::Sec => {
            state.cpu.flags.write(Flag::C, true)
        },
        Instruction::Nop => {
            ()
        },
    }
}

fn step(state: &mut Machine) {
    let instruction = Instruction::decode(/*&mut*/ state);
    execute(state, &instruction)
}
