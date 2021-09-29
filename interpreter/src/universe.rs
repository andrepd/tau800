use std::collections::VecDeque;

use crate::{
    assembler,
    instruction::{self, Instruction},
    interpreter,
    prelude::Machine,
};

const MAX_ITERATIONS_BEFORE_INCONSISTENCY: usize = 100;
const CYCLE_RANGE: usize = 200;

#[derive(Debug)]
pub enum ConsistencyError {
    DidNotConverge,
}

#[derive(Clone)]
pub struct Universe {
    timeline: VecDeque<Machine>,
    offset: usize,
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
            // = 200+1+200+2 = 403 snapshots
            //
            // Larger indices = older cycles
            //  (index 201 is before the current command, index 200 is after current command)
            //
            // In order to bootstrap the timeline, the machines must be able to
            // read from the timeline in the first place. The rules for this bootstrapping
            // are:
            //  - Future reads (i.e. reads from states not yet observed) = 0
            //  - Past reads - as are, despite consistency
            //  - Future writes - overridden
            //  - Past writes - performed without "re-consistency"
            let mut timeline = VecDeque::from(vec![Machine::new(); 3 + CYCLE_RANGE * 2]);

            timeline[CYCLE_RANGE + 1] = entry_point.clone();

            let mut current_state = entry_point;
            for t in (0..=(2 * CYCLE_RANGE)).rev() {
                let bootstrap_universe = Universe {
                    timeline: timeline.clone(),
                    offset: 0,
                };
                interpreter::step(&mut current_state, &bootstrap_universe);
                timeline[t] = current_state.clone();
            }

            timeline
        };

        let universe = Universe {
            timeline,
            offset: 0,
        };
        universe
            .consist()
            .expect("Could not bootstrap consistency.")
    }

    pub fn step(mut self) -> Result<Self, ConsistencyError> {
        self.timeline.pop_front();
        let mut next_state = self.timeline.back().unwrap().clone();
        interpreter::step(&mut next_state, &self);
        self.timeline.push_back(next_state);
        if self.offset < 2 * CYCLE_RANGE + 2 {
            self.offset += 1;
        }
        Ok(self.consist().expect("Could not self-consist."))
    }

    pub fn now(&self) -> &Machine {
        &self.timeline[CYCLE_RANGE + 1]
    }

    fn consist(self) -> Result<Self, ConsistencyError> {
        let past_limit = CYCLE_RANGE + 1 + self.offset;
        let mut last_universe = self.clone();
        for _iteration in 0..MAX_ITERATIONS_BEFORE_INCONSISTENCY {
            let mut new_universe = last_universe.clone();
            for instant in (1..=past_limit).rev() {
                let mut next_state = new_universe.timeline[instant].clone();
                interpreter::step(&mut next_state, &new_universe);
                new_universe.timeline[instant - 1] = next_state;
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
