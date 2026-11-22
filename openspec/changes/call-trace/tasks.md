# Tasks

Standing rules: CLAUDE.md. The decision to build this is in `DECISIONS.md` (2026-11-21).

## 1. The options

- [x] 1.1 Add `--call-trace` and `--no-call-trace` to the compile line and to `test` in
  `crates\driver\src\main.rs`, with the help text, and the field `call_trace` in `compile.rs` (design D7).
  Verify: `main.rs` has no unit tests, so the CLI tests of `crates\driver\tests\it\call_trace.rs` check it:
  `when_a_build_has_the_call_trace` (both options, with `--debug`), `the_records_of_the_other_builds` (with
  `--coverage --trace-values`), `print_paths_takes_no_call_trace_option`; `--target wasm` in task 4.1.
- [x] 1.2 Add `call_trace` to the `[build]` table in `crates\driver\src\settings.rs`, and the one function
  that decides the value: command line, else settings file, else `--debug`. Verify: unit tests for each of the
  five combinations of the spec's scenarios, and for `call_trace = "yes"` and the unknown-key message naming
  three keys.
- [x] 1.3 Check how the build cache keys a program. If it keys on the options, add the call trace to the key.
  Found: the key holds the text of every fragment (`build::program_inputs_key`), so nothing was added. Verify:
  the CLI test `a_key_of_its_own_in_the_build_cache` prints the key with and without `--call-trace`, and they
  differ.
- [x] 1.4 Document the options and the key in `docs\MANUAL.md`, beside `--trace-values`: the options' table,
  `[build]`, and a section "The call stack of a runtime error". Verify: the manual's example command runs as
  written (`qb64rust <file> --call-trace`, built and run on `nested.bas` in a scratch folder).

## 2. The hook in libqb

- [x] 2.1 Write the edits `tools\libqb\edits\call-trace-a-error-hook-header.edit`,
  `call-trace-b-critical-message.edit` and `call-trace-c-fix-error.edit` (one old and new pair per file): the
  pointer `libqb_error_call_stack`, its declaration in `error_handle.h`, and the calls in `fix_error` and
  `critical_error_message` (design D6), listed in `libqb\SOURCE.md`. No `UPSTREAM.md` entry: that file lists
  QB64pe's bugs to offer, and this is a feature of ours. Verify: `copy_libqb.py --check`
  passes (70 edits, 0 problems), and a program built without the call trace prints the same error message as
  before: `no_call_stack_without_the_option` and the CLI tests `an_untrapped_error_exits_with_1`,
  `going_on_after_an_error_exits_with_0`, `an_error_that_ends_the_program_traces_only_d012` and
  `trace_record_at_a_runtime_error` pass.

## 3. The emitter and the run-time part

- [x] 3.1 Add `crates\codegen-cpp\src\call_trace.rs` and `call_trace_rt.h`: the tables of procedure names,
  sites and `GOSUB` sites, the `thread_local` state, push, pop, and the hook's writer with its 24-row limit
  (design D1, D3, D4, D6). `trace.rs` has no walk of a statement's calls to share (design, "Found while
  building"): the emitter notes a call where it writes one. Verify: unit tests of the site numbering, the
  tables and the declared names; `cargo test -p qb64rust-codegen-cpp` (29 pass).
- [x] 3.2 Emit the push and pop in `Emitter::procedure` (`procs.rs`), the site store in `Emitter::body`, the
  event cases in `events.rs`, the handler's frame in `mainerr.txt`, `RESUME`, and the frames dropped by `RUN`
  (design D1, D2, D5). Pass the value down from `lib.rs`. Verify: the snapshot
  `tests\frontend\call_trace.bas` (mode `call-trace`), and the existing snapshots unchanged without it.
- [x] 3.3 CLI tests in `crates\driver\tests\it\call_trace.rs`, one or more per scenario of spec
  `compiler/call-trace`, compared with the whole block; several errors share a build under
  `QB64PE_NOPROMPT=continue`: a SUB, the main module, a FUNCTION inside an expression, an included file,
  recursion, the three `GOSUB` cases, a SUB and a label run by `ON TIMER`, an error in an error handler, 1,000
  calls deep, error 256, after a handled error, after `RUN`, after `RESUME` to a label, and `EXIT SUB`. The
  dialog is not tested: its text is the same string. Verify: the 11 tests that build pass on Windows
  (`after_resume_to_a_label` added 2026-11-22).
- [ ] 3.4 Measure the cost: a recursive loop of 10 million calls, and `g12_game`'s nine scripts, with and
  without the option, on Windows and in the suites VM's Linux. Write the numbers into `docs\MANUAL.md`. If a
  call costs more than about 10 ns, stop and put the design to the user (design, "Risks"). Verify: the
  manual's table holds the measured numbers and their date.

## 4. Other builds and the recordings

- [x] 4.1 A wasm build: a CLI test builds one `$CONSOLE:ONLY` program with `--target wasm --call-trace`, runs
  it under Node and compares the block with the desktop build's. Verify: the test passes
  (`the_same_call_stack_on_the_wasm_target`, on the host with `..\emsdk`).
- [x] 4.2 A program with a `SUB _GL` that calls procedures, and an error in the main program: a `screen\` or
  CLI test checks that the block lists the main program's frames only. Verify: the test passes 20 times in a
  row (`sub_gl_beside_the_program`, 20 runs in one test, in the suites VM's `cli` step, 2026-11-22). Its program
  has `$CONSOLE`, since a window program without one wrote no stderr there. The same run: 149 of 150 CLI tests
  pass; `unit_tests::a_timeout` timed out under load and passed alone, as `STATUS.md` notes it has before. Its
  first run failed four debugger tests, fixed in the macros (design, "Found while building").
- [x] 4.3 Find the tests that build a recorded program with `--debug` and compare its stderr (the debugger's
  CLI tests, `check`'s debug steps). Give them `--no-call-trace` or cut the block, as design D8 says. Found: none
  compares a debug build's error text (`debug_build_same_output` runs a program with no error; the debugger's
  tests read stops and exceptions), and no tool builds with `--debug`, so nothing changed. Verify: `just quick`
  passes (10 steps, 3 standing, 2026-11-22); the debugger's CLI tests run in the VM with 4.2.
- [x] 4.4 Run the programs under `tests\` whose recorded output holds `Runtime error: Line` (45 files, 38
  programs) with `--call-trace`, and compare their output with the block cut out. `check` cannot pass the
  option, so the runner got `--call-trace` (`tools\legacy_tests\run_legacy_tests.py`, `without_call_stacks`, with
  unit tests), and the run went on the host by the user's leave (2026-11-22). Verify: 25 corpus, 11 verification
  and 2 compile programs pass. The first run failed `slice\s19_header_errors`: the cutter took the program's own
  `  then` for a row; a row is now matched by its form.
- [x] 4.5 The same on Linux: the CLI tests of 3.3. The Linux steps run in WSL on the host (`tools\linux\run.sh`),
  not in the suites VM, and `linux-tier1` skips ignored tests, so they ran there by name. Verify: 11 pass in
  this tree's Linux copy (`cargo test -p qb64rust-driver --test it call_trace:: -- --ignored`, without the window
  test of 4.2; the wasm test says it has no SDK there and passes).

## 5. Records

- [x] 5.1 Add the `DIVERGENCES-QB64PE.md` row: a call-trace build's runtime error message holds a
  `Call stack:` block that QB64pe's does not. Verify: the row (D-076) names spec `compiler/call-trace` and a test.
- [x] 5.2 Edit `docs\reference\chapters\error-handling.md`: the message of an untrapped error, with the block
  and how to turn it on. Run `reference.py render` and `reference.py --check`. Verify: `--check` passes (400
  pages, 0 problems). Its source is D-076 until the spec is synced.
- [x] 5.3 `docs\LAYOUT.md` has no rows for single source files, so it gets the mode `call-trace` of
  `tests\frontend\` and the edits `call-trace-*` in the row of `tools\libqb\`. Add a `SOMEDAY.md` row for a
  function that returns the call stack to the program, and one for the call trace as the default of every
  build. Verify: `tools\repo_check\check_repo.py` passes (0 findings).
- [x] 5.4 Run `just quick` in the background, read its log, and update `STATUS.md`'s "Next". Verify: the log
  shows every step passing or says which stood and why (2026-11-22: 6 passed, among them `slice` and `reference`;
  7 stood, `lists`, `nextest`, `tools`, `tools-check` and `build` from the run before on the same inputs).

## Workflow follow-up

- Sync the delta specs into `openspec\specs\` and archive the change when the user asks.
- The game ports turn it on in their own `qb64rust.toml` (`call_trace = true`), in their own repos.
