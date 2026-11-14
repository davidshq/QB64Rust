//! The fuzz target `parse` (spec `testing/fuzzing`): the lexer and the parser, no panic and an exact round trip.
#![no_main]

use libfuzzer_sys::fuzz_target;
use qb64rust_driver::{Probe, probe};

fuzz_target!(|data: &[u8]| {
    probe(Probe::Parse, data).unwrap();
});
