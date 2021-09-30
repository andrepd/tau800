use std::{
    error::Error,
    fmt::{Debug, Display},
};

use chrono::{Local, Timelike};

use crate::{universe::Universe, word::UWord};

pub trait Module: Debug {
    /// Number of words this module writes to memory.
    fn size(&self) -> usize;

    /// Update the mem-mapped memory.
    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>>;
}

pub struct ModuleCollection(Vec<Box<dyn Module>>);

impl ModuleCollection {
    pub fn new(modules: Vec<Box<dyn Module>>) -> Self {
        Self(modules)
    }

    pub fn run(&mut self, universe: &mut Universe) {
        let slice_start = 0x10 << 6;
        for module in self.0.iter_mut() {
            let slice_length = module.size();
            let memmap = &mut universe.now_mut().ram.0[slice_start..(slice_start + slice_length)];
            module.run(memmap).expect("Failed to run IO module.");
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
    hours: String,
    minutes: String,
}

#[derive(Debug)]
pub enum DisplayModuleError {
    BadSevenSegment([bool; 7]),
}

impl std::fmt::Display for DisplayModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayModuleError::BadSevenSegment(bits) => {
                write!(f, "Bad 7-segment display bits: {:?}", bits)
            }
        }
    }
}

impl Error for DisplayModuleError {}

impl DisplayModule {
    pub fn new() -> Self {
        DisplayModule {
            hours: "00".to_string(),
            minutes: "00".to_string(),
        }
    }

    fn read_seven_segment(memory: u8) -> Result<char, DisplayModuleError> {
        let bits = (0..8)
            .map(|i| {
                if memory & (1 << (7 - i)) != 0 {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<bool>>();

        let number = match &bits[..] {
            [true, false, false, true, false, false, false] => '1',
            [false, true, true, true, true, false, true] => '2',
            [false, true, true, false, true, true, true] => '3',
            [true, false, true, false, false, true, true] => '4',
            [true, true, false, false, true, true, true] => '5',
            [true, true, false, true, true, true, true] => '6',
            [false, true, true, false, false, true, false] => '7',
            [true, true, true, true, true, true, true] => '8',
            [true, true, true, false, false, true, true] => '9',
            _ => {
                let mut copy = [false; 7];
                copy.clone_from_slice(&bits[..]);
                return Err(DisplayModuleError::BadSevenSegment(copy));
            }
        };

        Ok(number)
    }
}

impl Module for DisplayModule {
    fn size(&self) -> usize {
        // 0bAAAAAA 0bABBBBB 0bBBCCCC 0bCCCDDD 0bDDDDxx
        5
    }

    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>> {
        // 0bAAAAAA 0bABBBBB 0bBBCCCC 0bCCCDDD 0bDDDDxx
        let a = (memory[0].value() << 1) | (memory[1].value() >> 5);
        let b = ((memory[1].value() & 0b011111) << 2) | (memory[2].value() >> 4);
        let c = ((memory[2].value() & 0b001111) << 3) | (memory[3].value() >> 3);
        let d = ((memory[3].value() & 0b000111) << 4) | (memory[4].value() >> 2);

        let a = DisplayModule::read_seven_segment(a)?;
        let b = DisplayModule::read_seven_segment(b)?;
        let c = DisplayModule::read_seven_segment(c)?;
        let d = DisplayModule::read_seven_segment(d)?;

        self.hours.clear();
        self.hours.push(a);
        self.hours.push(b);

        self.minutes.clear();
        self.minutes.push(c);
        self.minutes.push(d);

        Ok(())
    }
}
