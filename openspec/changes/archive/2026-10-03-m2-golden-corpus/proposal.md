# Proposal

## Why

M2 starts the Rust compiler. Before any of it exists, we need a fixed record of what the old compiler does, so
that every later layer (lexer, parser, type checker, codegen) can be measured against it from the first day
(`study\00` §10, layer 2 of R11; lesson from QB64Fresh: measure against `qb64pe.exe` from the start). The existing
QB64pe suite checks 331 programs' output, but they are written to test QB64pe internals. QB64Fresh's 261
`runtime_comparison` programs are small, one feature each (string functions, math, control flow, SUB/FUNCTION,
TYPE, file I/O, error handling, PRINT formatting), and all use `$CONSOLE:ONLY`, so they run headless. Recording
their output with the old compiler now, before the vertical slice, costs no design decisions and gives the slice
and the front end their first conformance targets. This is the first of the queued M2 start items in `STATUS.md`.

## What Changes

- The 261 `.bas` programs from QB64Fresh's `tests/runtime_comparison/` (commit `85e6df8`, the user's own MIT code)
  are copied into `tests\corpus\runtime_comparison\` under `CLAUDE.md` rule 5: each is read in full; the QB64Fresh
  runner scripts, `DIFFERENCES.md` and its recorded results are **not** copied (they were measured on Linux against
  QB64Fresh and are not checked against `qb64pe.exe`). A `SOURCE.md` records origin, commit and licence.
- `verification\v11_wrap_o2.bas` (LONG overflow) joins the corpus, as decided in `study\16` §8.
- For each program, the old compiler's result is recorded next to it, in the format of QB64pe's own
  `compile_tests`: `<name>.output` (program output) or `<name>.err` (compiler error), so one runner reads both.
- The Windows runner `tools\legacy_tests\run_legacy_tests.py` gains a suite `corpus` (run and compare) and a
  `--record` option (write the expected files). Programs run with the old compiler's **default** build settings,
  not the `-O2` the legacy suite uses, because the default build is the behaviour we match (`study\16` §8).
- Programs whose output depends on the machine (environment variables, command line, clock) get a sidecar that
  replaces the varying part before comparison; recorded files hold no local paths (`CLAUDE.md` rule 2).
- Each program is also built with `-O2`; programs whose output differs between the two builds are listed in
  `tests\corpus\README.md` as input for the divergence register (this also measures the `_INTEGER64` question from
  `study\16` §8).
- A new baseline file in `baselines\` with the corpus result for the old compiler.

## Capabilities

### New Capabilities

- `testing/golden-corpus`: the corpus layout, how expected results are recorded, and how a compiler is run against
  them.

### Modified Capabilities

None.

## Impact

- New folder `tests\corpus\` (about 520 small files). No build or code changes outside the Python runner.
- Recording needs `..\QB64pe\qb64pe.exe` (pinned at `16f629784e`, QB64pe 4.7.0); recording, checking and the
  `-O2` pass each compile all 262 programs once (recording runs each executable twice).
- Not in this change: the 143 `qbasic_testcases` (mostly graphical or interactive; their run-time behaviour needs
  screen capture and input scripting, a later change), the Rust workspace, the divergence register and the
  numeric-semantics spec (next change, `m2-workspace-and-slice`), CI for the corpus (it needs the compiler build,
  which CI does not have yet).
- `STATUS.md`, `study\00` §10 and `CLAUDE.md` (layout table) updated when done.
