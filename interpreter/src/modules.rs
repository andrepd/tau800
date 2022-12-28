/// External modules that interact with the machine via mem-mapped IO

use super::prelude::*;

use std::error::Error;

pub trait Module: std::fmt::Debug {
    /// Update the state.
    fn run(&mut self, m: &mut Machine) -> Result<(), Box<dyn Error>>;
}

pub struct ModuleCollection(Vec<Box<dyn Module>>);

impl ModuleCollection {
    pub fn new(modules: Vec<Box<dyn Module>>) -> Self {
        Self(modules)
    }

    pub fn run(&mut self, universe: &mut Universe) {
        for module in self.0.iter_mut() {
            module.run(universe.now_mut()).expect("Failed to run IO module.");
        }
    }
}

/// A module that maps the current time as four words (digits; h h m m) into
/// memory, at locations %1000–%1300.
#[derive(Debug)]
pub struct ClockModule;

impl Module for ClockModule {
    fn run(&mut self, m: &mut Machine) -> Result<(), Box<dyn Error>> {
        use chrono::Timelike;
        let now = chrono::Local::now();
        let hour = format!("{:0>2}", now.hour());
        let minute = format!("{:0>2}", now.minute());
        let digits = hour.chars().chain(minute.chars());
        m.ram[0x10..0x14]
            .iter_mut()
            .zip(digits)
            .for_each(|(p, s)| *p = uWord::try_from(s.to_digit(10).unwrap() as u8).unwrap());
        Ok(())
    }
}

/// A module that reads a 4-digit seven-segment display from memory (see 
/// manual for the format).
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
        match &bits[..] {
            [true,  true,  true,  true,  true,  true,  false] => '0',
            [false, false, true,  false, false, true,  false] => '1',
            [false, true,  true,  true,  true,  false, true ] => '2',
            [false, true,  true,  false, true,  true,  true ] => '3',
            [true,  false, true,  false, false, true,  true ] => '4',
            [true,  true,  false, false, true,  true,  true ] => '5',
            [true,  true,  false, true,  true,  true,  true ] => '6',
            [false, true,  true,  false, false, true,  false] => '7',
            [true,  true,  true,  true,  true,  true,  true ] => '8',
            [true,  true,  true,  false, true,  true,  true ] => '9',
            bits => {
                // Return garbage for visual effect;
                // we convert the boolean values to an equivalent binary number,
                // and index into some garbage &'static str.
                let index = bits.iter().fold(0, |acc, &value| {
                    (acc << 1) + (value as usize)
                });
                const ALPHABET: &'static str = "x%@#";
                ALPHABET.chars().nth(index % ALPHABET.len()).unwrap()
            }
        }
    }
}

impl Module for DisplayModule {
    fn run(&mut self, m: &mut Machine) -> Result<(), Box<dyn Error>> {
        let memory = &m.ram[0x14..0x14+7];

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

/// A module that emulates a "disk drive", i.e. maps an external file. Page `n` 
/// (1k word pages) is requested by writing `n-1` to %3000,%3100 (big endian), 
/// and served on addresses %0030–%3f3f.
#[derive(Debug)]
pub struct DiskModule (Vec<uWord>);

impl DiskModule {
    /// Create a DiskModule mapping file `path`.
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        use std::io::prelude::*;
        let mut data = vec![];
        let f = std::io::BufReader::new(std::fs::File::open(path)?);
        for line in f.lines() {
            for word in line?.split_whitespace() {
                data.push(u8::from_str_radix(word, 16).unwrap().try_into().unwrap())
            }
        };
        Ok(DiskModule(data))
    }
}

impl Module for DiskModule {
    fn run(&mut self, m: &mut Machine) -> Result<(), Box<dyn Error>> {
        let page = uLong::from_hi_lo(m.ram[0x31], m.ram[0x30]);
        if page.value() != 0 {
            let page = (page.value() - 1) as usize;
            let start = page * 1024;
            let end = start + 1024;
            if start >= self.0.len() { 
                ()
            } else if end >= self.0.len() {
                let len = self.0.len() - start;
                let src = &self.0[start..start+len];
                let dst = &mut m.ram[0x30*0x40 .. 0x30*0x40+len];
                dst.clone_from_slice(src)
                /*let dst = &mut m.ram[0x30*0x40+len .. 0x40*0x40];
                dst.fill(uWord::ZERO);*/
            } else {
                let src = &self.0[start..end];
                let dst = &mut m.ram[0x30*0x40 .. 0x40*0x40];
                dst.clone_from_slice(src)
            }
        }
        Ok(())
    }
}
