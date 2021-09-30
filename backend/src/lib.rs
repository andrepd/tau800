use chrono::{Timelike, Utc};
use neon::prelude::*;
use rand::distributions::Bernoulli;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::sync::{mpsc, Arc, Once};
use std::thread;

macro_rules! let_move {
    ($x:ident) => {
        let $x = $x;
    };
    (mut $x:ident) => {
        let mut $x = $x;
    };
}

const REGISTER_A: usize = 0;
const REGISTER_F: usize = 1;
const REGISTER_BH: usize = 2;
const REGISTER_BL: usize = 3;
const REGISTER_CH: usize = 4;
const REGISTER_CL: usize = 5;
const REGISTER_X: usize = 6;
const REGISTER_SP: usize = 7;
const REGISTER_PC: usize = 8;
const REGISTERS: [usize; 9] = [
    REGISTER_A,
    REGISTER_F,
    REGISTER_BH,
    REGISTER_BL,
    REGISTER_CH,
    REGISTER_CL,
    REGISTER_X,
    REGISTER_SP,
    REGISTER_PC,
];

struct Report {
    numbers: [String; 2],
    registers: [[bool; 6]; 9],
    stack: u32,
    history: Vec<String>,
}

impl Default for Report {
    fn default() -> Self {
        Self {
            numbers: Default::default(),
            registers: Default::default(),
            stack: Default::default(),
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

    let emulation_thread = thread::spawn(move || {
        let_move!(receive);

        let mut lines_of_code = include_str!("program.asm")
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .cycle();
        let mut code_history = VecDeque::from_iter(lines_of_code.clone().take(6));
        let mut stack = 0;

        let mut registers = [[false; 6]; 9];

        while let Ok(response_channel) = receive.recv() {
            for _ in 0..thread_rng().gen_range(0..6) {
                lines_of_code.next();
            }
            code_history.push_front(lines_of_code.next().unwrap());
            code_history.pop_back();

            if thread_rng().gen_bool(0.25) {
                match thread_rng().gen_range(0..=2) {
                    0 => {
                        if stack > 0 {
                            stack -= 1
                        }
                    }
                    1 => {
                        if stack < 6 {
                            stack += 1
                        }
                    }
                    2 => {}
                    _ => unreachable!(),
                }
            }

            for register in registers.iter_mut() {
                if thread_rng().gen_bool(0.5) {
                    continue;
                }
                for bit in register.iter_mut() {
                    if thread_rng().gen_bool(0.5) {
                        *bit = !*bit;
                    }
                }
            }

            let now = Utc::now();
            let minute = format!("{:0>2}", now.minute())
                .chars()
                .rev()
                .collect::<String>();
            let hour = format!("{:0>2}", now.hour())
                .chars()
                .rev()
                .collect::<String>();

            let dummy_report = Report {
                numbers: [minute, hour], // Reversed on purpose
                registers,
                stack,
                history: code_history.iter().map(|x| x.to_string()).collect(),
            };

            if response_channel.send(dummy_report).is_err() {
                // Channel is closed
                break;
            }
        }
    });
    cx.export_function("poll", poll)?;
    Ok(())
}
