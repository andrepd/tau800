use super::prelude::*;
use std::collections::VecDeque;

const MAX_WINDOW: usize = 200;

/// Represents a timeline slice starting at time t0
#[derive(Debug, Clone)]
pub struct Timeline {
    t0: usize, 
    states: VecDeque<Machine>, 
}

impl std::ops::Add<&IWord> for usize {
    type Output = Self;

    fn add(self, other: &IWord) -> Self {
        (self as isize + other.value() as isize) as usize
    }
}

impl std::ops::Index<usize> for Timeline {
    type Output = Machine;
    fn index(&self, t: usize) -> &Self::Output {
        &self.states[t - self.t0]
    }
}

impl std::ops::IndexMut<usize> for Timeline {
    fn index_mut(&mut self, t: usize) -> &mut Self::Output {
        &mut self.states[t - self.t0]
    }
}

impl Timeline {
    /*pub fn at(&self, t: usize) -> Option<&Machine> {
        if self.in_interval(t) { 
            Some(&self.states[t - self.t0]) 
        } else { 
            None
        }
    }

    pub fn at_mut(&mut self, t: usize) -> Option<&mut Machine> {
        if self.in_interval(t) { 
            Some(&mut self.states[t - self.t0])
        } else {
            None
        }
    }*/

    pub fn ti(&self) -> usize {
        self.t0
    }

    // One-past
    pub fn tf(&self) -> usize {
        self.t0 + self.states.len()
    }

    pub fn in_interval(&self, t: usize) -> bool {
        self.ti() <= t && t < self.tf()
    }

    pub fn in_next_slot(&self, t: usize) -> bool {
        t == self.tf()
    }

    pub fn push_back(&mut self, x: Machine) {
        if self.states.len() == MAX_WINDOW {
            self.states.pop_front();
            self.t0 += 1;            
        };
        self.states.push_back(x);
    }
}

#[derive(Debug, Clone)]
pub enum Mode {
    /// Current timeline is consistent
    Consistent, 
    /// Current timeline, from ti to ti+states.len() might be inconsistent
    Maybe (usize, usize),
    /// Current timeline, from ti to ti+states.len() is definitely inconsistent
    Inconsistent (usize, usize),
}

// Basic state flow on Mode: 
//   Consistent --temporal inconsistency found-> Inconsistent
//   Inconsistent --reached end of interval-> Maybe
//   Maybe --reached end of interval-> Consistent

impl Mode {
    pub fn add_inconsistent(&mut self, ti: usize, tf: usize) {
        use std::cmp::{min,max};
        *self = match self {
            Mode::Consistent              => Mode::Inconsistent (ti, tf),
            Mode::Maybe        (ti_, tf_) => Mode::Inconsistent (min(ti, *ti_), max(tf, *tf_)),
            Mode::Inconsistent (ti_, tf_) => Mode::Inconsistent (min(ti, *ti_), max(tf, *tf_)),
        }
    }
}

pub struct Universe {
    pub timeline: Timeline, 
    pub t: usize,
    pub mode: Mode, 
    pub pending_writes: Vec<(usize, Op, UWord)>,
    pub pending_reads: Vec<(usize, usize, Op, UWord)>,
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            timeline: Timeline {
                states: VecDeque::from(vec![Machine::new()]),
                t0: 0,
            },
            t: 0,
            mode: Mode::Consistent,
            pending_writes: vec![],
            pending_reads: vec![],
        }
    }

    /// Pushes, overwriting existing state if necessary
    pub fn push_state(&mut self, x: Machine) {
        eprint!("push_state t0={:?} t={:?} len={:?} ", self.timeline.t0, self.t, self.timeline.states.len());
        self.t += 1;
        // Insert at immediately next time: ok
        if self.timeline.in_next_slot(self.t) {
            eprintln!("(push)");
            self.timeline.push_back(x);
        } 
        // Insert over existing time: ok
        else if self.timeline.in_interval(self.t) {
            eprintln!("(overwrite)");
            self.timeline[self.t] = x;
        } 
        // Insert anywhere else: fail
        else {
            panic!("Tried to insert state into the future: disconnected timeline. (Or, too far into the past/future)")
        }
    }

    pub fn push_new_state(&mut self) {
        eprintln!("push_new_state now=t={:?}", self.t);
        self.push_state(self.now().clone())
    }

    pub fn now(&self) -> &Machine {
        &self.timeline[self.t]
    }

    pub fn now_mut(&mut self) -> &mut Machine {
        &mut self.timeline[self.t]
    }

    /*pub fn at_offset(&self, delta: &IWord) -> &Machine {
        &self.timeline[self.t + delta/* - 1*/]
    }

    pub fn at_offset_mut(&mut self, delta: &IWord) -> &mut Machine {
        &mut self.timeline[self.t + delta/* - 1*/]
    }*/

    pub fn is_consistent(&self) -> bool {
        match self.mode { Mode::Consistent => true, _ => false }
    }
}

impl std::ops::Index<usize> for Universe {
    type Output = Machine;

    fn index(&self, t: usize) -> &Self::Output {
        &self.timeline[t]
    }
}

impl std::ops::IndexMut<usize> for Universe {
    fn index_mut(&mut self, t: usize) -> &mut Self::Output {
        &mut self.timeline[t]
    }
}
