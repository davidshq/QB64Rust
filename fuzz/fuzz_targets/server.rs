//! The fuzz target `server` (spec `testing/fuzzing`): the language server's check, format and index calls on a
//! document that is not a file. No panic.
#![no_main]

use libfuzzer_sys::fuzz_target;
use qb64rust_driver::{Probe, probe};

fuzz_target!(|data: &[u8]| {
    probe(Probe::Server, data).unwrap();
});
