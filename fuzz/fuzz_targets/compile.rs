//! The fuzz target `compile` (spec `testing/fuzzing`): the lexer, the parser and the checker, then lowering and
//! C++ generation for an accepted program. No panic, and an exact round trip.
#![no_main]

use libfuzzer_sys::fuzz_target;
use qb64rust_driver::{Probe, probe};

fuzz_target!(|data: &[u8]| {
    probe(Probe::Compile, data).unwrap();
});
