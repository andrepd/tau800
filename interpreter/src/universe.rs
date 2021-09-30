use crate::prelude::*;

pub type Timeline = Vec<Machine>;

pub struct Universe {
    pub states: Timeline,
    pub t: usize,
}

impl std::ops::Add<&IWord> for usize {
    type Output = Self;

    fn add(self, other: &IWord) -> Self {
        (self as isize + other.value() as isize) as usize
    }
}

impl Universe {
    pub fn new() -> Self {
        Universe { states: vec![Machine::new()], t: 0 }
    }

    pub fn push_state(&mut self, x: Machine) {
        self.states.push(x);
        self.t += 1;
    }

    pub fn now(&self) -> &Machine {
        &self.states[self.t]
    }

    pub fn now_mut(&mut self) -> &mut Machine {
        &mut self.states[self.t]
    }

    pub fn t_offset(&self, x: &IWord) -> &Machine {
        &self.states[self.t + x]
    }

    pub fn t_offset_mut(&mut self, x: &IWord) -> &mut Machine {
        &mut self.states[self.t + x]
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


