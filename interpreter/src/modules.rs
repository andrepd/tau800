use std::{
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

use chrono::{Local, Timelike};

use super::{universe::Universe, word::UWord};

pub trait Module: Debug {
    /// Number of words this module writes to memory.
    fn size(&self) -> usize;

    /// Update the mem-mapped memory.
    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>>;
}

pub struct ModuleCollection<'a>(Vec<Box<&'a mut dyn Module>>);

impl<'a> ModuleCollection<'a> {
    pub fn new(modules: Vec<Box<&'a mut dyn Module>>) -> Self {
        Self(modules)
    }

    pub fn run(&mut self, universe: &mut Universe) {
        let mut slice_start = 0x10;
        for module in self.0.iter_mut() {
            let slice_length = module.size();
            let memmap = &mut universe.now_mut().ram.0[slice_start..(slice_start + slice_length)];
            println!("{:?}", slice_start..(slice_start + slice_length));
            module.run(memmap).expect("Failed to run IO module.");
            slice_start += slice_length;
        }
    }
}

/// A module that maps the current time as four words (digits; h h m m) into
/// memory.
#[derive(Debug)]
pub struct ClockModule;

impl Module for ClockModule {
    fn size(&self) -> usize {
        4
    }

    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>> {
        let now = Local::now();
        let hour = format!("{:0>2}", now.hour());
        let minute = format!("{:0>2}", now.minute());
        let digits = hour.chars().chain(minute.chars());
        memory
            .iter_mut()
            .zip(digits)
            .for_each(|(p, s)| *p = UWord::from(s.to_digit(10).unwrap() as u8));
        Ok(())
    }
}

#[derive(Debug)]
pub struct DisplayModule {
    pub hours: String,
    pub minutes: String,
}
impl DisplayModule {
    pub fn new() -> Self {
        DisplayModule {
            hours: "00".to_string(),
            minutes: "00".to_string(),
        }
    }

    fn read_seven_segment(bits: &[bool]) -> char {
        let number = match &bits[..] {
            [true, true, true, true, true, true, false] => '0',
            [false, false, true, false, false, true, false] => '1',
            [false, true, true, true, true, false, true] => '2',
            [false, true, true, false, true, true, true] => '3',
            [true, false, true, false, false, true, true] => '4',
            [true, true, false, false, true, true, true] => '5',
            [true, true, false, true, true, true, true] => '6',
            [false, true, true, false, false, true, false] => '7',
            [true, true, true, true, true, true, true] => '8',
            [true, true, true, false, true, true, true] => '9',
            bits => {
                // Return garbage for visual effect;
                // we convert the boolean values to an equivalent binary number,
                // and index into some garbage &'static str.
                let index = bits.iter().enumerate().fold(0, |acc, (i, &value)| {
                    if value {
                        acc + 2_usize.pow(i as u32)
                    } else {
                        acc
                    }
                });
                const ALPHABET: &'static str = "abcd";
                ALPHABET.chars().nth(index % ALPHABET.len()).unwrap()
            }
        };

        number
    }
}

impl Module for DisplayModule {
    fn size(&self) -> usize {
        7
    }

    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>> {
        println!("{:?}", memory);

        let a = memory[0..7]
            .iter()
            .map(|v| (0b000001 & v.value()) != 0)
            .collect::<Vec<bool>>();
        let b = memory[0..7]
            .iter()
            .map(|v| (0b000010 & v.value()) != 0)
            .collect::<Vec<bool>>();
        let c = memory[0..7]
            .iter()
            .map(|v| (0b000100 & v.value()) != 0)
            .collect::<Vec<bool>>();
        let d = memory[0..7]
            .iter()
            .map(|v| (0b001000 & v.value()) != 0)
            .collect::<Vec<bool>>();

        let a = DisplayModule::read_seven_segment(&a[..]);
        let b = DisplayModule::read_seven_segment(&b[..]);
        let c = DisplayModule::read_seven_segment(&c[..]);
        let d = DisplayModule::read_seven_segment(&d[..]);

        {
            let bytes = unsafe { self.hours.as_bytes_mut() };
            a.encode_utf8(&mut bytes[0..1]);
            b.encode_utf8(&mut bytes[1..2]);
        }

        {
            let bytes = unsafe { self.minutes.as_bytes_mut() };
            c.encode_utf8(&mut bytes[0..1]);
            d.encode_utf8(&mut bytes[1..2]);
        }

        Ok(())
    }
}
