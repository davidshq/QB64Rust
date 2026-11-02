# Spec Delta

## ADDED Requirements

### Requirement: One test binary per crate
Each crate's integration tests SHALL be linked into one test binary (`tests/it/main.rs` with one module per
former file), so that a change to a crate links one binary and not one per test file. No test SHALL change its
name or what it checks by the move.

#### Scenario: The driver's tests
- **WHEN** the driver crate's tests build
- **THEN** one integration-test binary is linked, and every test of the former twelve files runs in it

### Requirement: The Rust tests run under nextest
The workspace's tests SHALL run under `cargo nextest` with the repository's configuration: each test in its own
process, a slow test ended after its timeout, a test that passes only on a retry reported as flaky and counted as
a failure, and a JUnit report written. The doc tests SHALL run under `cargo test --doc` beside it.

#### Scenario: A hanging test
- **WHEN** a test does not end within the configured timeout
- **THEN** it is ended, reported as timed out, and the other tests go on

#### Scenario: A flaky test
- **WHEN** a test fails once and passes on its retry
- **THEN** the run reports it as flaky and fails

## MODIFIED Requirements

### Requirement: Tier 2 slice run
The corpus runner SHALL accept `--list <file>` to run only the programs named in it (one `<group>/<name>` per
line, `#` comments allowed). Running it with `--qb64 <qb64rust.exe> --list tests/corpus/slice.list` SHALL pass
every listed program. The slice list is the program run of `check quick` (spec `testing/check-workflow`).

#### Scenario: Slice list run
- **WHEN** `run_legacy_tests.py --suite corpus --qb64 target/release/qb64rust.exe --list tests/corpus/slice.list`
  is run
- **THEN** every listed program passes

#### Scenario: Run by the quick layer
- **WHEN** `check quick` runs after a change under `crates/`
- **THEN** the slice list runs with the pass cache and its result is one step of the quick layer's report
