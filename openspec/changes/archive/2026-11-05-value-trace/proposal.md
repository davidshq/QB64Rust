# Proposal

## Why

A user wants to know which values each variable of a program takes over a whole run. One use is to find the
range a variable really has. Another is to see that a variable never leaves a few values. The debugger cannot
answer this. It shows a value only while the program is paused, and its data breakpoints are limited to about
four variables by the CPU's watchpoints. The compiler already has a mode that instruments a whole program and
writes a record at exit: the `--coverage` build. A value-trace build can reuse that mechanism.

## What Changes

- A new compile option, `qb64rust --trace-values`. It builds a program that records each value stored into each
  BASIC variable while it runs.
- When the program ends and the environment variable `QB64RUST_TRACE` names a file, it appends one record to
  that file. The record gives, for each variable:
  - how many stores it had;
  - its distinct values, up to a limit, each with how often it was stored and the line that first stored it;
  - for a number, the smallest and largest value; for a string, the shortest and longest length.
- A variable here is a scalar, an array (all its elements together), or a member path of a user type
  (`player.pos.x`, `ships().hp`).
- A traced build evaluates nothing twice and records no store that a pending error skipped. Its output and
  errors are those of the normal build.
- A variable passed by reference is read again after the call, and recorded when its value changed. An array
  passed to a `SUB` gets a link to the parameter whose entry holds the values stored through it.
- Stores that are not recorded by value are counted and listed in the record instead of being missed:
  - bulk stores that write a variable's starting value, with the array's name: `ERASE`, `REDIM`. A bulk
    store that brings new values (`b() = a()`, `_ARRAYCOPY`, `GET #` into a whole array) is recorded element by
    element (user, 2026-11-05);
  - stores at an address: `POKE`, `_MEMPUT`, `_MEMCOPY`, `_MEMFILL`, `BLOAD`, `CALL ABSOLUTE`, `CLEAR`, `RUN`, a
    graphics `GET` into an array, and writes by a `DECLARE LIBRARY` procedure.
- A new tool, `tools\trace\value_trace.py`. It merges the records of several runs of one program and reports by
  variable, as text or JSON. It refuses records whose source no longer matches the files it reads.
- Without the option, the C++ written is byte for byte what it is today. The option combines with `--coverage`
  and `--debug`.

Out of scope:
- Static range analysis, which would give the values a variable *could* hold without running it. That is a
  separate question for a later change.
- A view in the VS Code extension. The record and the tool come first.
- Which element of an array held a value. An array's values are merged over all its elements.

## Capabilities

### New Capabilities
- `compiler/value-trace`: what a value-trace build records, which stores it sees and which it names as
  untraced, the record format, and the tool that merges and reports records.

### Modified Capabilities
- `compiler/cli`: a new requirement, "Value-trace build", for the `--trace-values` option and the
  `QB64RUST_TRACE` variable, in the form of the existing "Coverage build" requirement.

## Impact

- `crates\ir`: a new function `Op::stores`, beside `Op::may_raise`, that says what each operation stores.
- `crates\codegen-cpp`: a new module `trace.rs` beside `coverage.rs`, its run-time block `trace_rt.h`, and the
  record calls in `Emitter::store` (`stmt.rs`) and the other places that store.
- `crates\driver`: the option in `main.rs`, `compile.rs` and `lib.rs`, passed down as `--coverage` is.
- A new folder `tools\trace\` with `value_trace.py`, its tests and a `README.md` that gives the record format.
- Tests: CLI tests in `crates\driver\tests\it\cli.rs`, a `trace` mode for the front-end tests in
  `tests\frontend\`, and the tool's own tests.
- Documents: `MANUAL.md` (a section beside "Measuring which lines run"), `LAYOUT.md` (the new folder), and a
  `DECISIONS.md` row.
- No change to libqb, to the debugger or to the extension. A traced program runs slower and uses more memory than
  its normal build. Only the traced build pays that cost.
