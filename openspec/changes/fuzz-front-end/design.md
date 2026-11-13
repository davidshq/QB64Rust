# Design

## Context

See `proposal.md` for the reason. What exists and shapes the design:

- `crates\driver\tests\it\mutate.rs` has a function `check(bytes, emitted)`. It calls
  `qb64rust_driver::frontend("mutant.bas", bytes)`, compares `print(&fe.parsed.main().green, bytes)` with the
  bytes, and for a program without errors of at most 32 KB calls `lower` and `emit_ir_debug` with coverage and
  tracing on. It wraps all of that in `catch_unwind`.
- `frontend` uses `NoLoader`, so `$INCLUDE` reads nothing from the disk.
- The compiler runs on a thread with `STACK_SIZE` of stack (`with_stack`). The mutation test starts its
  threads with the same size.
- The workspace forbids `unsafe` code (`[workspace.lints.rust]`) and pins Rust 1.99.1.
- The release profile keeps overflow checks on, so an overflow in the compiler's arithmetic panics.
- The Linux test steps run in WSL (the distribution `Ubuntu`). The host is Windows with the MSVC toolchain.
- Every GitHub workflow is started by hand (`workflow_dispatch`, OpenSpec change `testing-reset`).

Not yet measured: whether `cargo-fuzz` builds this workspace in WSL, and how many inputs per second each
target reaches. Task group 1 measures both before anything else is built.

## Goals / Non-Goals

**Goals:**

- Find panics, broken round trips and very slow inputs in the lexer, the parser, the checker, lowering, the
  emitter and the server's three calls.
- Make every finding a test that `cargo test` runs on Windows and Linux, with no fuzzer installed.
- Keep the workspace, the Windows build and `just quick` unchanged in what they need and how long they take.

**Non-Goals:**

- Formatting twice gives the same text. A different property; a `SOMEDAY.md` row if wanted.
- Building or running the C++ that fuzzed programs emit.
- Programs generated from a grammar.
- Fuzzing on Windows.

## Decisions

### D1. `cargo-fuzz` with libFuzzer, in WSL

The fuzzer is libFuzzer through `cargo-fuzz` and the crate `libfuzzer-sys`. It runs on Linux only: in WSL on
the user's machine and on GitHub's `ubuntu-latest`.

Why: it is the usual tool for Rust, it needs no code in the compiler, it minimizes a failing input
(`cargo fuzz tmin`), and it takes a dictionary. On Linux it is the best supported. On Windows with MSVC its
support is partial, and the front end has no code that differs by host: `frontend` already reads every
program as compiled on Windows.

It needs a nightly Rust. The nightly is installed in WSL and on the runner only, and its date is pinned in
`fuzz\rust-toolchain.toml` so that a run is repeatable.

Alternatives:

- *Only raise `QB64RUST_MUTATE_COUNT`.* No new tool, but no coverage feedback. A thousand times more random
  edits still rarely get past the first syntax error. It stays available and costs nothing.
- *AFL++ through `afl.rs`.* Works on stable Rust, and its `fuzz!` macro runs in persistent mode. Its comparison
  tracing (CmpLog) helps it find the keywords and constants a parser compares against. It would need a second
  set of targets and a second build in `tools\fuzz\fuzz.py`, and has no `cargo fuzz tmin`. It is the first thing
  to try if coverage stops growing (`SOMEDAY.md`).
- *A property-testing crate (`proptest`, `bolero`).* `bolero` can drive libFuzzer and also run in
  `cargo test`. It would add a dependency to the workspace for what the regression test (D6) already gives.

If task 1.1 finds that `cargo-fuzz` cannot build the workspace in WSL, stop and put the alternatives to the
user. Nothing else in the design depends on libFuzzer except D5's flags.

Measured (task 1.1, 2026-11-12): `cargo-fuzz` 0.13.2 with `nightly-2026-11-13` (rustc 1.102.0-nightly
`8eb778d04`) builds the fuzz crate and the workspace's crates in WSL. A build from nothing took 1 min 54 s
(about 10 min of CPU on 16 cores).

### D2. The fuzz crate is outside the workspace

`fuzz\Cargo.toml` has its own empty `[workspace]` table and its own `Cargo.lock`. It depends on
`qb64rust-driver` and `qb64rust-syntax` by path and on `libfuzzer-sys`.

Why: `cargo fuzz init` makes it this way. The workspace's `cargo build`, `cargo test`, `clippy` and `fmt` do
not see it, so they need no nightly and no Linux. The `fuzz_target!` macro expands to an exported C function,
which the workspace's `unsafe_code = "forbid"` would refuse.

`cargo deny` runs on the workspace's lock file. Task 2.4 adds a second run on `fuzz\Cargo.lock` to the
existing job, so the license and source policy covers `libfuzzer-sys` and what it pulls in.

### D3. One function holds the properties

A new public function in `crates\driver\src\lib.rs`:

```rust
pub enum Probe { Parse, Compile, Server }
pub fn probe(kind: Probe, bytes: &[u8]) -> Result<Reached, &'static str>
```

- `Probe::Parse` lexes and parses, and returns `Err("round trip failed")` when the tree does not print back
  the bytes.
- `Probe::Compile` is today's body of `check` in `mutate.rs`: `frontend`, the round trip, and for an accepted
  program of at most 32 KB `lower` and `emit_ir_debug` with coverage and tracing. `Reached` says whether C++
  generation was reached, which the mutation test counts.
- `Probe::Server` calls `check_for_server`, `format_for_server` and `index_for_server` as `server_hooks`
  wires them, with no path on disk.

`probe` does not catch panics. The mutation test and the regression test wrap it in `catch_unwind`, as
`mutate.rs` does now. The fuzz targets do not: libFuzzer reports the panic itself.

Why in `driver` and not in the fuzz crate: the regression test must run on Windows without the fuzz crate,
and two copies of the properties would drift. Alternative: a test-support crate. One function of about 40
lines does not need a crate.

Each fuzz target is one file of a few lines, `fuzz\fuzz_targets\parse.rs`, `compile.rs`, `server.rs`:
`fuzz_target!(|data: &[u8]| { probe(Probe::Parse, data).unwrap(); })`.

### D4. The stack

libFuzzer calls the target on the process's main thread. The run script sets `ulimit -s` to `STACK_SIZE` (in
KB) before it starts the fuzzer, so that the target has the stack the compiler gives itself. A stack overflow
within that size is then a real finding: the compiler would overflow on the same program.

Alternative: start a thread with `STACK_SIZE` for each input, as `with_stack` does. It costs a thread per
input, tens of microseconds, on a target that should run thousands of inputs per second. Task 1.2 measures
both if `ulimit` does not reach the fuzzer's main thread.

Measured (task 1.2, 2026-11-12, a target that calls `frontend` only, `ulimit -s 65536`, one job):

- 5,000 nested parentheses (`x = ((…1…))`, 10,006 bytes) pass in 20 ms. `qb64rust -z` on the same file
  reports `not supported yet: an expression nested more than 1000 levels deep` at column 1005 and exits 1. The
  parser's limit stops both before the stack is deep, so no thread per input is needed.
- With AddressSanitizer (`cargo fuzz`'s default) a 2-minute run from the 514 corpus programs of at most 16 KB
  ended by itself with no finding: 7,234 inputs in 121 s, about 60 a second.
- Without a sanitizer (`-s none`) a 60-second run from the same seeds: 19,576 inputs, about 320 a second, five
  times as many. Tiny inputs (at most 64 bytes, no seeds) ran at about 1,800 a second, so the time goes into
  the size of the corpus programs, not a cost per call.

So `fuzz.py` builds and runs with `-s none` (D5). AddressSanitizer finds memory errors of `unsafe` code, and
the workspace forbids `unsafe`, and the code the targets run is almost all the workspace's own. A panic, an overflow check, a timeout and the memory limit work the same
without it. `fuzz.py --sanitizer address` keeps the other build for a run that wants it.

### D5. One script starts a run

`tools\fuzz\fuzz.py <target> <minutes> [--jobs N]` is the only entry point. `just fuzz` calls it, and so does
the workflow. On Windows it starts itself again in WSL through Git's bash, the way `tools\check\steps.py`
runs the Linux steps. On Linux it:

1. writes the dictionary to `fuzz\target\keywords.dict` from the compiler's keyword and built-in tables (a
   small `--dump-keywords` path of a test binary, or `qb64rust`'s own table printed by a hidden test: task
   3.2 picks the one that needs no new command-line option);
2. copies the corpus programs of at most 16 KB and the regression inputs into the target's seed folder;
3. runs `cargo fuzz run -s none <target> -- -max_total_time=<s> -max_len=16384 -timeout=10 -rss_limit_mb=2048
   -dict=… -jobs=N -workers=N`, with the seed folder and the kept corpus `fuzz\corpus\<target>\`;
4. on a failure runs `cargo fuzz tmin` on the artifact, for five minutes at most, and prints the path of the
   smallest input;
5. prints the summary line the spec asks for, read from libFuzzer's last `stat` lines.

`--jobs` defaults to 2. The user's machine is in use while a run goes, and two cores for some minutes is what
the Linux steps of `just full` already take.

Why 16 KB: libFuzzer mutates small inputs faster and finds the same parser states. Only programs of that size
or less are seeds; the corpus has enough of them. Counted (task 3.1, 2026-11-12): 514 of the corpus's 516
programs are 16 KB or less.

Changed in building (2026-11-12):

- More than one job runs as `-fork=N`, not `-jobs=N -workers=N`. Measured on a target made to panic: `-jobs`
  starts another job after a failure and goes on, while fork mode stops at the first one, as the spec asks.
  Fork mode prints no final stats, so the summary reads the inputs from its last `#N: cov: …` line.
- The fuzz targets are built before the clock starts, so the minutes given are all fuzzing.
- The dictionary comes from an ignored test of the driver's test binary, `write_fuzz_dictionary`. It writes
  the reserved words, the built-ins as written (`LEFT$`) and the words of QB64pe's layout, which hold every
  metacommand: 547 words. `qb64rust_format::words` became public for it.
- A failing input is copied back from the Linux copy to the same path under the Windows tree, so the user
  finds it where the summary line says.
- Fork mode is given `-ignore_crashes=0 -ignore_timeouts=0 -ignore_ooms=0`. Without them it goes on after a
  timeout or a memory failure: the first campaign's `compile` ran its 30 minutes and wrote three timeouts.
- `tmin` is stopped after 5 minutes by `fuzz.py`, with every process it started. Its own `-max_total_time`
  bounds each libFuzzer run it starts, not the whole minimization: one timeout input was still being
  minimized after 15 minutes.

Why not a step of `check`: a step of `check` passes or fails for a tree and is cached by its hash. A fuzz run
is different each time and never "stands". It would also add a nightly toolchain to what `just full` needs.

### D6. A finding becomes a file and a test

A failing input is copied unchanged to `tests\fuzz\regressions\<target>-<short name>.bas`. The short name
says what it broke, for example `parse-unclosed-paren-in-data.bas`. A second file is never needed: the test
runs every file through all three probes.

`crates\driver\tests\it\fuzz_regressions.rs` reads the folder, runs each file on a thread with `STACK_SIZE`
through `probe` for `Parse`, `Compile` and `Server` inside `catch_unwind`, and fails with the file's name and
the problem. An empty folder passes. The test is part of `cargo test`, so of `just quick`, on both hosts.

The files are raw bytes and may hold control characters. `.gitattributes` marks the folder `-text` so that
git changes no line endings. `check_repo.py` already scans every tracked file for private text.

Alternative: put findings in `tests\frontend\`. Those files need a mode line and are text, and a finding is
often neither.

The timeout findings (10 seconds) cannot be asserted by a test without making it flaky under load. For such a
file the regression test only requires that the probe returns. Its fix is checked by running the fuzz target
on the file once (`cargo fuzz run <target> <file>`), and the commit message says so.

### D7. The workflow

`.github\workflows\fuzz.yml`, `workflow_dispatch` with inputs `target` (one of the three, or `all`) and
`minutes` (default 20). It installs the pinned nightly and `cargo-fuzz`, restores `fuzz\corpus` from the
Actions cache, runs `tools\fuzz\fuzz.py`, saves the cache, and uploads `fuzz\artifacts` when the run failed.
It uses GitHub's machine, not the user's, so long runs can go there.

No schedule: the project's workflows are all started by hand. A weekly schedule is one line to add later if
the user wants it.

### D8. The first campaign and its findings

After the targets work, each runs for 30 minutes. Every finding is handled before the change closes:

- a panic or a broken round trip is fixed, with its regression file, in the crate where it is;
- a stack overflow on deep nesting is a design question (a nesting limit with a diagnostic, as QB64pe has
  limits of its own). If one is found, the task stops and puts it to the user with the panel's view;
- a slow input is fixed if the cause is plainly quadratic code, otherwise it gets a `SOMEDAY.md` row and the
  file is kept out of the regressions folder until it is fixed.

The number of findings is not known in advance. If the first hour gives more than about 20 distinct ones,
the campaign stops there, the list goes to the user, and the rest becomes a second change.

The first campaign (task 4.1, 2026-11-12, `--jobs 2`, each target 30 minutes):

- `parse`: 413,790 inputs, no failure.
- `compile`: 305,465 inputs, 3 timeouts, all one bug. `CONST p = 4 ^ 0.5` never finished compiling: the
  constant evaluator's `pow2` (`crates\sema\src\certify.rs`) took its reference from `BigFloat::pow`, which
  does not return for some exact powers (`4 ^ 0.5`, `16 ^ 0.25`, `2.25 ^ 0.5`; `9 ^ 0.5` and `4 ^ 0.25` do
  return). The language server hung on the same line. The fuzzer made it from a corpus line by changing one
  digit. Fixed: the reference is `exp(y × ln x)` with 64 bits more inside (`big_pow`), which the mpmath
  values, the recorded values and the calibration of `powl`'s bound (6.56 ulp) still hold. Its input is
  `tests\fuzz\regressions\compile-exact-fractional-power.bas`, and the unit test
  `exact_fractional_powers_return` holds the five powers above.
- `server`: not run in the first campaign (the session that ran it ended). Run on the fixed tree: 169,774
  inputs, no failure. (Fork mode's `#N` count is not monotonic: it showed 186,977 at 1,626 s and 169,774 at
  the end; the summary takes the last line.)
- The regression input run alone through each target on the fixed tree: 0 ms, 2 ms and 4 ms, exit 0.

Coverage of `compile`'s kept corpus after it (7,482 inputs; `cargo fuzz coverage`, merged and reported with
the nightly's `llvm-profdata` and `llvm-cov`). These are lines and functions of the code linked into the
target:

| Crate | Lines covered | Functions covered |
|---|---|---|
| `syntax` | 4,709 of 5,729 (82.2%) | 693 of 841 |
| `sema` | 9,992 of 13,480 (74.1%) | 1,110 of 1,381 |
| `codegen-cpp` | 3,768 of 6,128 (61.5%) | 393 of 572 |
| `ir` | 1,104 of 2,465 (44.8%) | 115 of 202 |

So the fuzzer does reach the checker and the code generator; the risk below did not come true, and no
grammar-aware mutator is needed yet. `rustup component add llvm-tools-preview` failed in WSL three times
(the download's TLS stream breaks there); the same archive, fetched on Windows and checked against its
published SHA-256, was unpacked in `/tmp` for this, not installed.

## Risks / Trade-offs

- [`cargo-fuzz` or the pinned nightly does not build the workspace] → task 1.1 is first and decides; D1 names
  what to ask the user.
- [The fuzzer spends its time on lexer noise and never reaches the checker] → the corpus seeds and the keyword
  dictionary; task 4.1 reads libFuzzer's coverage of `sema` and `codegen-cpp` after the first run and records
  it in this file. If the checker's coverage is low, the next step is a grammar-aware mutator, as a new change.
- [A run loads the user's machine] → 2 jobs by default, a fixed end time, started by hand. Long runs go to
  the workflow.
- [Findings are many and derail the change] → the limit of D8.
- [A nightly in WSL is one more thing to keep working] → pinned by date in `fuzz\rust-toolchain.toml`; it is
  touched only when a fuzz run is wanted, and nothing else depends on it.
- [The kept corpus grows without limit] → it is git-ignored and under `fuzz\`; `fuzz.py --clean <target>`
  runs `cargo fuzz cmin` to shrink it.

## Open Questions

- Should the `Fuzz` workflow run on a schedule later? It does not change what is built now.
