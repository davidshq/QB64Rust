//! Tier 1 (spec `testing/differential-tests`, "Generated programs", "Recorded with the old compiler"): the programs
//! in `tests\differential` are exactly what the generator writes, and each has its recording.

use qb64rust_difftest::{stale, unrecorded};
use std::path::Path;

#[test]
fn programs_are_fresh_and_recorded() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/differential");
    let problems = stale(&dir);
    assert!(
        problems.is_empty(),
        "tests/differential is out of date:\n{}\nregenerate with: cargo run -p qb64rust-difftest -- gen, then record \
         the changed programs (tests/differential/README.md)",
        problems.join("\n")
    );
    let missing = unrecorded(&dir);
    assert!(
        missing.is_empty(),
        "no recording (.output or .err) for:\n{}\nrecord them with qb64pe.exe (tests/differential/README.md)",
        missing.join("\n")
    );
}
