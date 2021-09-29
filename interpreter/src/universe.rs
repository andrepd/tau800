use std::collections::VecDeque;

use crate::{
    assembler,
    instruction::{self, Instruction},
    interpreter,
    prelude::Machine,
};

const MAX_ITERATIONS_BEFORE_INCONSISTENCY: usize = 100;

#[derive(Debug)]
pub enum ConsistencyError {
    DidNotConverge,
}

#[derive(Clone)]
pub struct Universe {
    timeline: VecDeque<Machine>,
}

impl PartialEq for Universe {
    fn eq(&self, other: &Self) -> bool {
        self.timeline
            .iter()
            .zip(other.timeline.iter())
            .all(|(a, b)| a == b)
    }
}

impl Universe {
    pub fn bootstrap(source_code: &str) -> Self {
        // First, assemble the entry point (the machine at the present when the
        // program is started, before any code is ran).
        let entry_point = {
            let instruction_iterator = assembler::assemble(source_code);
            let mut entry_point = Machine::new();
            for instruction in instruction_iterator {
                Instruction::encode(&mut entry_point, &instruction);
            }
            entry_point.reset_cpu(); // CPU is modified when writing RAM
            entry_point
        };

        // Next, create an inconsistent timeline, to be resolved.
        let timeline = {
            // Because we admit up to 200 cycles of offset to the future or past,
            // and *instructions are transitives between states*, we need to keep
            // (state after each past command) + (state after present command) +
            //      (state after each future command) + (state before and after first and last command, respectively) =
            // = 200+1+200+2 = 403 snapshots (index 200 is before the current command)
            //
            // In order to bootstrap the timeline, the machines must be able to
            // read from the timeline in the first place. The rules for this bootstrapping
            // are:
            //  - Future reads (i.e. reads from states not yet observed) = 0
            //  - Past reads - as are, despite consistency
            //  - Future writes - overridden
            //  - Past writes - performed without "re-consistency"
            let mut timeline = VecDeque::from(vec![Machine::new(); 403]);

            timeline[200] = entry_point.clone();

            let mut current_state = entry_point;
            for delta_t in 0..200 {
                let bootstrap_universe = Universe {
                    timeline: timeline.clone(),
                };
                interpreter::step(&mut current_state, &bootstrap_universe);
                timeline[201 + delta_t] = current_state.clone();
            }

            timeline
        };

        let universe = Universe { timeline };
        universe
            .consist()
            .expect("Could not bootstrap consistency.")
    }

    pub fn step(mut self) -> Result<Self, ConsistencyError> {
        self.timeline.pop_front();
        let mut next_state = self.timeline.front().unwrap().clone();
        interpreter::step(&mut next_state, &self);
        self.timeline.push_back(next_state);
        Ok(self.consist().expect("Could not self-consist."))
    }

    pub fn now(&self) -> &Machine {
        &self.timeline[200]
    }

    fn consist(self) -> Result<Self, ConsistencyError> {
        let mut last_universe = self.clone();
        for _iteration in 0..MAX_ITERATIONS_BEFORE_INCONSISTENCY {
            let mut new_universe = last_universe.clone();
            for delta_t in 0..200 {
                let mut next_state = new_universe.timeline[200 + delta_t].clone();
                interpreter::step(&mut next_state, &new_universe);
                new_universe.timeline[201 + delta_t + 1] = next_state;
            }

            if new_universe == last_universe {
                return Ok(new_universe);
            } else {
                last_universe = new_universe;
                continue;
            }
        }

        Err(ConsistencyError::DidNotConverge)
    }
}
