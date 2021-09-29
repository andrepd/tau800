use std::{collections::VecDeque};

use crate::{
    assembler,
    instruction::{self, Instruction},
    interpreter,
    prelude::Machine,
};

#[derive(Debug)]
pub enum ConsistencyError {

}

pub struct UniversePov {}

pub struct Universe {
    timeline: VecDeque<Machine>,
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
            for plus_t in 0..200 {
                let bootstrap_universe = Universe { timeline: timeline.clone() };
                interpreter::step(&mut current_state, &bootstrap_universe);
                timeline[201 + plus_t] = current_state.clone();
            }

            timeline
        };
        
        let mut universe = Universe { timeline };

        universe.self_consist().expect("Could not bootstrap consistency.");

        universe
    }

    fn self_consist(&mut self) -> Result<(), ConsistencyError> {
        todo!()
    }
}
