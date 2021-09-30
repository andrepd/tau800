use std::{error::Error, fmt::Debug};

use chrono::{Timelike, Local};

use crate::{universe::Universe, word::UWord};

pub trait Module: Debug {
    /// Number of words this module writes to memory.
    fn size(&self) -> usize;

    /// Update the mem-mapped memory.
    fn run(&mut self, memory: &mut [UWord]) -> Result<(), Box<dyn Error>>;
}

/// A module that maps the current time as four words (digits; h h m m) into
/// memory.
#[derive(Debug, Clone)]
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
