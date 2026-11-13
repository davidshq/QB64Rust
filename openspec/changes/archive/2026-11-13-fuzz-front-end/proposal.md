# Proposal

## Why

The language server runs the lexer, the parser and the checker on half-typed code all day, and a panic there
ends the server. Today only two things test them on input nobody wrote on purpose:

- `inputs.rs` runs every program of the test sets through the front end. All of them are valid or nearly valid
  programs.
- The seeded mutation test (`crates\driver\tests\it\mutate.rs`) makes 20 mutants of each corpus program, about
  5,600 in all, from one fixed seed. It finds a panic only if one of those 5,600 inputs reaches it.

A coverage-guided fuzzer keeps the inputs that reach new code and mutates those again. In minutes it reaches
branches that a fixed set of random edits does not. The front end has an ideal property for it: no panic, and
the tree prints back the exact bytes it was parsed from. `SOMEDAY.md` has held this as a P4 row since
2026-10-29, and `study\47` section 2 names it as the fourth of the next wins.

## What Changes

- A fuzz crate, `fuzz\`, outside the Cargo workspace, with three targets:
  - `parse`: the lexer and the parser. No panic, and an exact round trip.
  - `compile`: the lexer, the parser and the checker. A program the checker accepts also goes through lowering
    and C++ generation. No panic at any stage.
  - `server`: the three calls the language server makes on a document (check, format, index). No panic.
- One function in the `driver` crate holds the properties, so that the fuzz targets, the mutation test and
  the regression test check the same thing.
- A command, `just fuzz <target> <minutes>`, runs a target in WSL for a set time. It is started by hand. It is
  not a step of `just quick`, `just full` or `just merge`.
- A workflow, `Fuzz`, runs the same on GitHub's Linux runner. It is started by hand, like the other workflows.
- A folder, `tests\fuzz\regressions\`, holds each input that failed, made as small as the fuzzer can make it.
  A new test in `cargo test` runs every file in it. A finding is fixed in the same commit as its file.
- Limits the fuzzer enforces besides "no panic": one input may take 10 seconds and 2 GB at most. An input that
  goes past either is a finding.
- The first campaign: each target for 30 minutes. Every finding is fixed or, if it is a design question,
  written to `SOMEDAY.md`.
- The `SOMEDAY.md` row "Fuzz the lexer and parser" is removed.

Not in this change:

- Checking that formatted output is stable when formatted again. It is a different property and may fail on
  many inputs today.
- Fuzzing the generated C++ by compiling and running it.
- Generating programs from the grammar. The corpus and a keyword dictionary are the seeds.
- A scheduled run. Every workflow here is started by hand (OpenSpec change `testing-reset`).

## Capabilities

### New Capabilities

- `testing/fuzzing`: the fuzz targets and their properties, how a run is started and bounded, and what becomes
  of a finding.

### Modified Capabilities

None. The seeded mutation test of `testing/compiler-tests` keeps its requirement. It only calls the shared
function instead of its own copy.

## Impact

- **New:** `fuzz\` (a crate with its own `Cargo.toml` and `Cargo.lock`, depending on `libfuzzer-sys`),
  `tests\fuzz\regressions\`, `tools\fuzz\fuzz.py`, `.github\workflows\fuzz.yml`, a `fuzz` recipe in `justfile`.
- **Changed:** `crates\driver\src\lib.rs` (one new public function), `crates\driver\tests\it\mutate.rs` (calls
  it), `crates\driver\tests\it\` (one new test file), `.gitignore`, `deny.toml` or its workflow if the fuzz
  crate's dependencies need covering, `docs\LAYOUT.md`, `crates\README.md` (the test commands), `SOMEDAY.md`.
- **Toolchain:** a nightly Rust and `cargo-fuzz` in WSL and on the GitHub runner only. The workspace stays on
  1.99.1, and nothing on Windows changes.
- **Machine time:** a run uses the cores it is given for the minutes it is given. The default is 2 jobs.
- **Disk:** the fuzzer's working corpus and build stay under `fuzz\`, git-ignored.
