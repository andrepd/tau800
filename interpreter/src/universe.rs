use crate::machine::Machine;
use crate::prelude::*;
use std::collections::BTreeMap;

struct Universe {
    timeline: [Machine; 1 << (2 * WORD_SIZE)],
}
