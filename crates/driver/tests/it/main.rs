//! The crate's integration tests, one module per former test file, linked into one binary (spec
//! `testing/compiler-tests`, "One test binary per crate").

mod check;
mod cli;
mod consts_v45;
mod format_oracle;
mod format_server;
mod frontend;
mod fuzz_regressions;
mod inputs;
mod language_features;
mod mutate;
mod names;
mod support;
mod unit_tests;
mod versioninfo;
