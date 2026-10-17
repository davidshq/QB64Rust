# Tasks

Standing rules: `CLAUDE.md` (no subagents, nothing staged, backslashes through the tools, test cadence rule 9).

## 1. The trace

- [x] 1.1 `qbr_diverged` in `global.txt` (design D1); the rows of design D2; `MemOf::lock_slot0` in `sema`.
- [x] 1.2 Every `cpp` snapshot read and accepted: 47, each the old one plus the trace (checked by a script that
  undoes the trace's additions and compares).

## 2. Tests and the runner

- [x] 2.1 The CLI tests of the D-rows name the rows they take (`build_and_run_tracing`); every other CLI test that
  builds and runs a program takes none; `mem_of_an_array_parameter_with_no_number_yet` for D-016 (new). 32 pass.
- [x] 2.2 The runner sets `QB64RUST_DIVERGENCES` for every corpus and compile-suite run and fails on an unlisted
  row; `<name>.divergences`; its tests (79 pass, 6 new).
- [x] 2.3 The first slice run named five programs that take a row on purpose (D-004: `s22`, `s61`, `s114`,
  `s117`; D-019: `s118`); each got a sidecar saying why.

## 3. Close (shared with `m3-measured-copies`)

- [x] 3.1 Closing runs, once: the slice list 437 of 437 (432, then the five with their sidecars), 437 of 437 with
  `QB64RUST_NO_FOLD=1`; upstream `pass.list` 318 of 318; differential 59 of 59; the full corpus 456 pass, `218_fre`,
  5 known failures; the full upstream suite 318 pass, 85 rejected (72 at the compile, 13 `.err`), 1 known failure;
  the call-site check 1677 of 1677 (the trace is no libqb call); the demand reports and the shrink-only lists
  unchanged. No program of either suite took a row it does not list.
- [x] 3.2 Documents: `DIVERGENCES.md`, `tests\corpus\README.md`, `tools\legacy_tests\README.md`, `STATUS.md`,
  `DECISIONS.md`.
