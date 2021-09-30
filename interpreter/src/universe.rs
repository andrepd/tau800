use crate::prelude::*;

pub type Timeline = Vec<Machine>;

pub struct Universe {
    pub states: Timeline,
    pub t: usize,
    /// If currently trying for consistency, this will contain Some(ti, tf, operand, value). If 
    /// normal operation, this will be None.
    pub target: Option<(usize, usize, Op, UWord)>,
    pub cona: UWord,
}

impl std::ops::Add<&IWord> for usize {
    type Output = Self;

    fn add(self, other: &IWord) -> Self {
        (self as isize + other.value() as isize) as usize
    }
}

impl Universe {
    pub fn new() -> Self {
        Universe { states: vec![Machine::new()], t: 0, target: None, cona: UWord::from(0) }
    }

    /// Pushes, overwriting existing state if necessary
    pub fn push_state(&mut self, x: Machine) {
        if self.t+1 < self.states.len() {
            self.states[self.t+1] = x
        } else {
            self.states.push(x);
        };
        self.t += 1;
    }

    pub fn push_new_state(&mut self) {
        self.push_state(self.now().clone())
    }

    pub fn now(&self) -> &Machine {
        &self.states[self.t]
    }

    pub fn now_mut(&mut self) -> &mut Machine {
        &mut self.states[self.t]
    }

    pub fn t_offset(&self, x: &IWord) -> &Machine {
        &self.states[self.t + x - 1]
    }

    pub fn t_offset_mut(&mut self, x: &IWord) -> &mut Machine {
        &mut self.states[self.t + x]
    }

    pub fn rewind_keep(&mut self, t: usize) {
        debug_assert!(t < self.t);
        self.t = t;
    }

    pub fn rewind_destroy(&mut self, t: usize) {
        debug_assert!(t < self.t);
        self.states.resize_with(t+1, || {unreachable!()});
        self.t = t;
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


