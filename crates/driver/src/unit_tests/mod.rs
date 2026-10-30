//! `qb64rust test`: unit tests of BASIC code (spec `compiler/unit-tests`; OpenSpec change `basic-unit-tests`).
//!
//! A test file (`*_test.bas`) holds test SUBs (`discover.rs`). Each file is built once, through the compile path of
//! `-x` (`crate::compile`), with a generated entry point as its main file (`entry.rs`) and the assertion library
//! served from memory (`loader.rs`); each test then runs in a process of its own (`run.rs`) and is judged from the
//! result file the program writes, its stderr and its exit code (`judge.rs`). `report.rs` prints the results as
//! text or JSON lines.

pub mod discover;
pub mod entry;
pub mod judge;
pub mod loader;
pub mod report;
pub mod run;
