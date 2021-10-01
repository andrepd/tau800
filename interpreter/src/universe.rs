use std::collections::VecDeque;

use super::prelude::*;

pub type Timeline = VecDeque<Machine>;
const MAX_MEMORY: usize = 50;

#[derive(Debug, Clone)]
pub enum Mode {
    /// Normal execution mode
    Normal,
    /// Currently resolving a forward reference to read from the future
    Fw {
        ti: usize,
        tf: usize,
        op: Op,
        guess: UWord,
    },
    /// Currently resolving a backward reference to write to the past
    Bw {
        ti: usize,
        tf: usize,
        state: Machine,
    },
}

pub struct Universe {
    pub states: Timeline,
    // How many states have been pop_front()ed from the timeline
    pub forgetten: usize,
    pub t: usize,
    pub mode: Mode,
    pub guess: UWord,
    pub pending_writes: Vec<(usize, Op, UWord)>,
}

impl std::ops::Add<&IWord> for usize {
    type Output = Self;

    fn add(self, other: &IWord) -> Self {
        (self as isize + other.value() as isize) as usize
    }
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            states: VecDeque::from(vec![Machine::new()]),
            forgetten: 0,
            t: 0,
            mode: Mode::Normal,
            guess: UWord::from(0),
            pending_writes: vec![],
        }
    }

    pub fn t_as_index(&self) -> usize {
        self.t - self.forgetten
    }

    /// Pushes, overwriting existing state if necessary
    pub fn push_state(&mut self, x: Machine) {
        if self.t_as_index() + 1 < self.states.len() {
            let t = self.t_as_index();
            self.states[t + 1] = x
        } else {
            self.states.push_back(x);
        };
        if self.states.len() > MAX_MEMORY {
            self.states.pop_front();
            self.forgetten += 1;
        }
        self.t += 1;
    }

    pub fn push_new_state(&mut self) {
        self.push_state(self.now().clone())
    }

    pub fn now(&self) -> &Machine {
        &self.states[self.t_as_index()]
    }

    pub fn now_mut(&mut self) -> &mut Machine {
        let t = self.t_as_index();
        &mut self.states[t]
    }

    pub fn t_offset(&self, x: &IWord) -> &Machine {
        &self.states[self.t_as_index() + x - 1]
    }

    pub fn t_offset_mut(&mut self, x: &IWord) -> &mut Machine {
        let t = self.t_as_index();
        &mut self.states[t + x]
    }

    pub fn rewind_keep(&mut self, t: usize) {
        debug_assert!(t < self.t);
        self.t = t;
    }

    pub fn rewind_destroy(&mut self, t: usize) {
        debug_assert!(t < self.t);
        self.states.resize_with(t + 1, || unreachable!());
        self.t = t;
    }

    pub fn is_normal(&self) -> bool {
        match self.mode {
            Mode::Normal => true,
            _ => false,
        }
    }
}

impl std::ops::Index<usize> for Universe {
    type Output = Machine;

    fn index(&self, i: usize) -> &Self::Output {
        &self.states[i]
    }
}

impl std::ops::IndexMut<usize> for Universe {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.states[i]
    }
}
