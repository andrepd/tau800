use chrono::{Timelike, Utc};
use neon::prelude::*;
use rand::distributions::Bernoulli;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::{mpsc, Arc, Once};
use std::thread;

use crate::emu::assembler;
use crate::emu::modules::{ClockModule, DisplayModule, ModuleCollection};
use crate::emu::universe::Universe;

mod emu;

macro_rules! let_move {
    ($x:ident) => {
        let $x = $x;
    };
    (mut $x:ident) => {
        let mut $x = $x;
    };
}
struct Report {
    numbers: [String; 2],
    registers: [[bool; 6]; 9],
    stack: u32,
    had_time_jump: bool,
    history: Vec<String>,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            numbers: Default::default(),
            registers: Default::default(),
            stack: Default::default(),
            had_time_jump: Default::default(),
            history: Default::default(),
        }
    }
}

impl Report {
    fn into_jsobject<'r, 'h: 'r>(
        &self,
        cx: &'r mut CallContext<'h, neon::prelude::JsObject>,
    ) -> NeonResult<Handle<'h, JsObject>> {
        let numbers = {
            let array = JsArray::new(cx, 2);
            let number = cx.string(self.numbers[0].clone());
            array.set(cx, 0, number)?;
            let number = cx.string(self.numbers[1].clone());
            array.set(cx, 1, number)?;
            array
        };
        let registers = {
            let array = JsArray::new(cx, 9);
            for i in 0..9 {
                let subarray = JsArray::new(cx, 6);
                for j in 0..6 {
                    let boolean = cx.boolean(self.registers[i][j]);
                    subarray.set(cx, j as u32, boolean)?;
                }
                array.set(cx, i as u32, subarray)?;
            }
            array
        };
        let stack = cx.number(self.stack);
        let history = {
            let array = JsArray::new(cx, 6);
            for i in 0..6 {
                let command = cx.string(self.history[i].clone());
                array.set(cx, i as u32, command)?;
            }
            array
        };

        let object = JsObject::new(cx);

        object.set(cx, "numbers", numbers)?;
        object.set(cx, "registers", registers)?;
        object.set(cx, "stack", stack)?;
        object.set(cx, "had_time_jump", had_time_jump)?;
        object.set(cx, "history", history)?;

        Ok(object)
    }
}

static mut POLL_CHANNEL: Option<mpsc::SyncSender<oneshot::Sender<Report>>> = None;

fn poll(mut cx: FunctionContext) -> JsResult<JsObject> {
    let sender = unsafe { &POLL_CHANNEL };
    if sender.is_none() {
        // If the sender has not yet been initialized by the emulation thread,
        // return an empty object.
        // This is done instead of returning, e.g., an undefined because it's
        // unlikely that this situation will occur for long (if at all), and allows
        // the front-end not to have logic to filter the response.
        return Report::default().into_jsobject(&mut cx);
    }
    let sender = sender.as_ref().unwrap();

    // Poll the emulation thread for the current state
    // This is done in a semi-async way, where we send a one shot channel back to
    // this thread, and await a response on that channel.
    // We always expect the emulation thread to be listening to the channel; it
    // is this thread that closes the channel on exit.
    let (report_tx, report_rx) = oneshot::channel::<Report>();
    sender.send(report_tx).unwrap();
    let report = report_rx.recv().unwrap();
    report.into_jsobject(&mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let (send, receive) = mpsc::sync_channel::<oneshot::Sender<Report>>(0);
    unsafe {
        POLL_CHANNEL = Some(send);
    }

    let _emulation_thread = thread::spawn(move || {
        let_move!(receive);
        'all: loop {
            // IO
            let mut clock_module = ClockModule;
            let mut display_module = DisplayModule::new();

            // Emulation

            let mut universe = Universe::new();
            assembler::assemble_into(universe.now_mut(), include_str!("program.asm"));

            let mut cmd_history = VecDeque::new();
            cmd_history.resize_with(6, || "nop".to_string());

            'emu: while let Ok(response_channel) = receive.recv() {
                // Step the machine
                let mut time_jump_iterations = 0;
                {
                    let mut io_modules = ModuleCollection::new(vec![
                        Box::new(&mut clock_module),
                        Box::new(&mut display_module),
                    ]);
                    io_modules.run(&mut universe);

                    let mut last_command = emu::interpreter::step(&mut universe);
                    while !universe.is_normal() {
                        // In resolution
                        last_command = emu::interpreter::step(&mut universe);

                        time_jump_iterations += 1;
                        if time_jump_iterations > 100 {
                            eprintln!("Consistency failure. Resetting machine.");
                            //panic!("Consistency failure.");
                            continue 'emu; // Reset the machine on panic
                        }
                    }

                    cmd_history.pop_back();
                    cmd_history.push_front(emu::assembler::mnemonic(last_command));
                }

                // Read the information

                let hours = (&display_module.hours).clone();
                let minutes = (&display_module.minutes).clone();

                let stack = 0x7f - universe.now().cpu.sp.value() as u32;

                let registers = {
                    let mut registers = [[false; 6]; 9];
                    let now = universe.now();

                    let value_as_bits = |value: u16, slice: &mut [bool]| {
                        for i in 0..6 {
                            slice[i] = value & (1 << i) != 0
                        }
                    };

                    value_as_bits(now.cpu.flags.word.value() as u16, &mut registers[0]);
                    value_as_bits(now.cpu.a.value() as u16, &mut registers[1]);
                    value_as_bits(now.cpu.bh.value() as u16, &mut registers[2]);
                    value_as_bits(now.cpu.bl.value() as u16, &mut registers[3]);
                    value_as_bits(now.cpu.ch.value() as u16, &mut registers[4]);
                    value_as_bits(now.cpu.cl.value() as u16, &mut registers[5]);
                    value_as_bits(now.cpu.x.value() as u16, &mut registers[6]);
                    value_as_bits(now.cpu.sp.value(), &mut registers[7]);
                    value_as_bits(now.cpu.pc.value(), &mut registers[8]);

                    registers
                };

                let had_time_jump = time_jump_iterations > 0;

                let dummy_report = Report {
                    numbers: [hours, minutes],
                    registers,
                    stack,
                    had_time_jump,
                    history: cmd_history.iter().cloned().collect(),
                };

                if response_channel.send(dummy_report).is_err() {
                    // Channel is closed
                    break 'all;
                }
            }
        }
    });
    cx.export_function("poll", poll)?;
    Ok(())
}
