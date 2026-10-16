# Proposal

## Why

The user asked for it on 2026-10-16 (`DECISIONS.md`, item (5)): "a run-time trace of divergences: an environment
variable makes the runtime print one line when a D-row's path is taken, and the test runner sets it and fails on
a line it does not expect, for every D-row with a run-time path". Until now a program could take the path of a
row of `DIVERGENCES.md` and still pass, when nothing it prints shows the difference: the register rows were
pinned by CLI tests one by one, and nothing told where else a program took them.

## What Changes

- **The generated program traces each D-row path.** `qbr_diverged(n)` stands where the program takes the path of
  row D-n; when `QB64RUST_DIVERGENCES` names a file, the first time of each row in a run appends `D-0nn` to it.
  Without the variable it does nothing. Rows with a run-time path: D-003, D-004, D-006, D-016, D-018, D-019,
  D-020, D-021, D-022. The others have none: D-001, D-002 (the default build of the old compiler wraps too),
  D-005, D-009, D-014 (the C++ text differs, and no branch is taken at run time), D-007, D-008, D-015, D-017
  (compile time), D-010–D-013 (not built).
- **The runner sets the variable for every run** of the corpus and compile suites and fails a program whose trace
  names a row that its `<name>.divergences` sidecar does not list (stage "divergences").
- **The CLI tests of the D-rows check the trace**: each names the rows its program takes; every other CLI test
  that builds and runs a program takes none. D-016 gets its first test.
- **Five slice programs take a row on purpose** and get a sidecar: `s22_store_rules`, `s61_member_arrays`,
  `s114_dyn_members`, `s117_dyn_member_kinds` (D-004), `s118_dyn_member_files` (D-019).

Out of scope: tracing at compile time; the verification suite, which records the old compiler only.

## Capabilities

### Modified Capabilities

- `testing/golden-corpus`: a requirement "The run-time trace of divergences".

## Impact

- `crates\codegen-cpp` (`decl.rs` the helper, `stmt.rs`, `arrays.rs`, `owner.rs`, `mem.rs`, `value.rs`),
  `crates\sema` (`ExprKind::MemOf` gets `lock_slot0`, set in `check\mem.rs`).
- `tools\legacy_tests\run_legacy_tests.py` and its tests; `crates\driver\tests\cli.rs`.
- Every `cpp` snapshot (the helper in `global.txt`).
- Documents: `DIVERGENCES.md` (how a row is traced), `tests\corpus\README.md`, `tools\legacy_tests\README.md`,
  `DECISIONS.md`, `STATUS.md`.
