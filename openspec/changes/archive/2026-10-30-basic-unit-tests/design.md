# Design

## Context

See `proposal.md` (Why). The state this design starts from:

- **libqb ends a program on an error in two places**, both in `libqb\internal\c\libqb\src\error_handle.cpp`.
  `fix_error`, for an error no handler takes, calls `end()` when the answer is "no" (under `QB64PE_NOPROMPT=y`, or
  at the dialog). `end()` leaves through the main loop's `libqb_exit(exit_code)` (`libqb.cpp`), and `exit_code` is
  still 0 there. `exit_after_fatal_error`, for critical errors (300 to 315, out of memory, the fatal division
  entry), calls `libqb_exit(0)`. Measured on 2026-10-29 with `target\release\qb64rust.exe`: `ERROR 5` and
  `a(9) = 1` for `DIM a(3)` both exit 0 under `QB64PE_NOPROMPT=y`. A failed `_ASSERT 1 = 2, "math"` on line 3 of an
  included `a_test.bas` writes `math` to stdout, then `Runtime error: Line: 3 (in a_test.bas)` and `_ASSERT failed
  (check console for description)` to stderr, and exits 0.
- **The run-time trace** is written by the generated program (`qbr_diverged`, `crates\codegen-cpp\src\decl.rs`,
  `global`). That function is static in the program's C++, so libqb cannot call it.
- **The recordings.** 133 programs of `verification\` end in `Runtime error`, every one with `exit=0`, and about 35
  `.output` files of `tests\corpus` and `tests\upstream` print one (among them `noprompt\noprompt-y`).
- **The compile path** is `run()` in `crates\driver\src\main.rs`. The front end takes a main file's name, its bytes
  and a `Loader` (`frontend_at`, `crates\driver\src\lib.rs`). `FileLoader` resolves an `$INCLUDE` in the including
  file's folder, then the compiler root. The fixes come from the `qb64rust.toml` above the main file's path
  (`program_fixes`).
- **Measured in both compilers** (2026-10-29, `qb64rust.exe` and `qb64pe.exe`): an included file may start with
  `OPTION _EXPLICIT` and `DEFSTR A-Z` after the including file's `$CONSOLE:ONLY`, `$ASSERTS` and `DIM SHARED`.
  Module-level code may follow the included file's SUBs. An `INTEGER` variable passed to an `_INTEGER64` parameter
  prints its value. So an entry point that includes the test file, then dispatches, compiles in both.

## Goals / Non-Goals

**Goals:**

- A result that cannot be a false pass: a test passes only on a positive record that its SUB returned.
- One mechanism for every way a test ends: return, failed assertion, `_ASSERT`, runtime error, `END`/`SYSTEM`,
  crash, timeout.
- The entry point and the library are plain BASIC that `qb64pe.exe` also compiles, so the framework can be checked
  against the old compiler.

**Non-Goals:**

- Testing the SUBs of a program without running its main module (`SOMEDAY.md` row, added by this change).
- Graphics tests (`SCREEN`, a window): the entry point is `$CONSOLE:ONLY`, so phase 1 is for **console-only
  code**. Measured in task 4.5 (2026-10-29, `tests\unit_tests\graphics\image_test.bas` and a probe): images work
  under `$CONSOLE:ONLY`: `_NEWIMAGE`, `_DEST` and `_SOURCE` of an image, `PSET` and `_RGB32` into it with `POINT`
  reading it back, `_PUTIMAGE` between images, `PRINT` into an image, `_FREEIMAGE`; `SCREEN 12`, `SCREEN
  _NEWIMAGE(...)` and `_DISPLAY` run without an error. What fails is drawing on the console itself: `PSET` or
  `CIRCLE` with no image as the destination is error 5, "Illegal function call", reported as ERROR, not FAIL (CLI
  test `images_work_and_drawing_on_the_console_is_an_error`). The extension's README says so, so that such an
  error is not taken for a fault of the framework.
- Debugging a test, coverage per test, a pass cache of test executables: rows in `SOMEDAY.md`.
- The line of a failed library assertion. BASIC cannot know its caller's line, so the failure is shown at the
  test SUB. `_ASSERT` gives a line.

## Decisions

### D1. D-012 as a libqb edit, value 1, traced by libqb

`tools\libqb\edits\d012-error-exit-code.edit` changes `error_handle.cpp`:
- `fix_error` sets `exit_code = 1` before `end()` when the answer is "no".
- `exit_after_fatal_error` calls `libqb_exit(1)`.
- Both call a new static `trace_divergence_012()`. It appends `D-012` to the file `QB64RUST_DIVERGENCES` names,
  with the same rule as `qbr_diverged` (nothing without the variable).

The path is shared by Windows and Linux, console and window programs.

- **Value 1**: the common "failed" code. A program's own `SYSTEM 1` gives the same code, which is acceptable: the
  trace tells the runner which it was, and `qb64rust test` reads its result file. Rust's 101 and the error
  number were considered. The error number does not fit in a Linux exit code (errors up to 518) and would mix two
  meanings.
- **Traced by libqb, not by the program**: the end happens inside libqb, which cannot call the program's static
  `qbr_diverged`. Making that function extern for libqb to call couples the libqb copy to every program's C++.
  A dozen lines in the edit cost less.
- **Always on, not a fix**: D-012 was decided on 2026-10-08 as a divergence. The 2026-10-26 rule puts only *kept*
  behaviors behind the switch. A switch that is off by default would leave scripts blind, which is what D-012
  exists to end.
- Crash exit codes (a signal, an access violation) are not changed. `END n` and `SYSTEM n` are unchanged.
- `UPSTREAM.md` gets the edit as an offer, as D-031's was.

### D2. The runner's D-012 rule, without sidecars

In `tools\legacy_tests\run_legacy_tests.py`, at each place a suite checks a run's exit code (corpus `run_case`, the
compile suite, the verification compare):
- When the trace holds `D-012` and the wanted code is 0, want 1. For the verification compare, the recorded
  `exit=0` last line counts as `exit=1`.
- `D-012` is left out of the rows a `.divergences` sidecar must list.

A record run with `qb64pe.exe` sets no trace and is untouched.

- **Alternative: a sidecar per program** (about 170 files saying `D-012`). Rejected: each file would restate one
  rule, and new recordings would need one too. The rule checks both directions exactly (trace ⇒ 1, no trace ⇒ the
  recorded code), so the strictness the sidecars give is kept.
- Tier 1 tests that expect success after an untrapped error (CLI tests such as
  `an_error_that_ends_the_program_traces_nothing`, which builds `RETURN back` and `a(9).m = 5`) are updated to
  expect 1 and `D-012`.

### D3. `qb64rust test` compiles in the process, with a virtual main file beside the test file

The test command builds each test file with the compile path that `-x` uses, factored out of `run()` into
`crates\driver\src\compile.rs` (`compile_exe(main_path, name, bytes, loader, options, out) -> Result<bool, String>`:
`Ok(false)` when the program has errors, `Err` for a failure of the compiler or the build; what the command line
prints goes to `out`). `run()` calls it, and its behavior is unchanged: the existing CLI tests are the check.

- The main file is **virtual**: bytes generated in memory, its path `<test folder>\<name>.bas.test-entry.bas`,
  which never exists on disk.
- Because its folder is the test file's folder, everything the compiler finds from the main file's folder
  resolves as for a build of the test file: `$INCLUDE` of the test file by its bare name, the test file's own
  includes, `qb64rust.toml` (the fixes), `$EXEICON`, `$EMBED`, the QB64pe folder's relative paths.
- **`TestLoader`** (`crates\driver\src\unit_tests\loader.rs`) wraps `FileLoader`. It serves the library's two files
  from memory under a reserved name (`qb64rust-test\qbtest.bi`, `qb64rust-test\qbtest.bm`) and passes every other
  path on. The library is embedded with `include_str!` from `crates\driver\src\unit_tests\qbtest.bi` and `.bm`.
- The executable and its build folder go to `<temp>\qb64rust-test\<fnv1a64 of the test file's canonical
  path>\<process id>\`, so nothing is written beside the test file, two test files never share a folder, and
  neither do two runs of one file at once (found in task 4.5: the CLI tests, like the editor and a terminal,
  build one file with different filters at the same time, and one run then ran the other's executable). The
  folder is removed when all the file's tests passed, and kept otherwise (also after a build error that wrote
  something there, its path printed). The next run of the file removes what earlier runs kept: each run holds a lock, `<process id>.lock`
  beside its folder, taken before the folder is made, and a sibling folder whose lock no run holds is removed
  (found in the review of the staged change: a test rerun while it fails would otherwise leave one executable per
  run in the temp folder). The build cache spares the C++ build of a second run.
- **Alternative: write the entry point to a temp folder and spawn `qb64rust -x` on it.** Rejected: every lookup
  from the main file's folder would then be the temp folder's.
- **Alternative: write the entry point beside the test file.** Rejected by the spec (a source tree left with
  generated files; two runs at once collide).

### D4. The entry point and the protocol

Generated by `crates\driver\src\unit_tests\entry.rs`:

```
$CONSOLE:ONLY
$ASSERTS
'$INCLUDE:'qb64rust-test/qbtest.bi'
'$INCLUDE:'math_test.bas'
qbtest_begin
SELECT CASE UCASE$(ENVIRON$("QB64RUST_TEST_NAME"))
CASE "TESTADD": TestAdd
CASE "TEST_SUB": test_sub
CASE ELSE: qbtest_unknown
END SELECT
qbtest_done
SYSTEM
'$INCLUDE:'qb64rust-test/qbtest.bm'
```

- Everything the library and the entry point use has an explicit type and a `qbtest_` prefix, except the
  documented `Assert*`, `Fail` and `TestTempDir$`. The test file's `DEFxxx`, `OPTION _EXPLICIT` and `OPTION BASE`
  cannot change them.
- The test file's module-level code runs at `'$INCLUDE:'math_test.bas'`, before the dispatch: it is each test's
  setup, in a fresh process.
- **The result file** is named by `QB64RUST_TEST_RESULT`. It is opened for append at each write and closed again,
  so a crash leaves what was written. Lines:
  - `BEGIN`: written by `qbtest_begin`, after the module-level code ran.
  - `FAIL<TAB><message>`: a failed assertion. The library then ends the test with `SYSTEM 2`.
  - `DONE`: written by `qbtest_done` after the SUB returned.
  - `UNKNOWN`: the name matched no SUB.
- `TestTempDir$` returns `QB64RUST_TEST_TEMP` with a trailing separator.
- **Judging** (`crates\driver\src\unit_tests\judge.rs`), in this order:
  - Timed out: TIMEOUT.
  - A `FAIL` line: FAIL with its message.
  - stderr has libqb's `_ASSERT failed` line: FAIL, with the line and file of the `Runtime error: Line: N (in F)`
    line before it and, as the message, the last line the program printed to stdout (where libqb writes the
    message, measured above) when the line is error 315's `_ASSERT failed (check console for description)`;
    `_ASSERT failed` for error 314, an `_ASSERT` with no message, which writes nothing to stdout
    (`sub__assert`, `libqb.cpp`).
  - stderr has another `Runtime error:` line: ERROR with that message.
  - `DONE` and exit code 0: pass.
  - Anything else: ERROR. "ended before its SUB returned" with no `BEGIN` or no `DONE`; "unknown test" with
    `UNKNOWN`; the exit code when it is not 0.

  Printed output never decides a pass.
- **Alternatives considered:**
  - Markers on stdout: rejected, because the code under test prints too.
  - Passing the test name in `COMMAND$`: rejected, because a path or name with blanks would need QB64's quoting
    rules.
  - Exit code only: rejected, because a `SYSTEM 0` in the code under test would pass.

### D5. Discovery from the test file's own parse

`crates\driver\src\unit_tests\discover.rs` parses the test file alone (the front end with `NoLoader`, so its
includes are not read). It walks the tree of that file for SUB headers with no parameter list or an empty one,
whose name (suffix-free, as SUB names are) starts with `test` in any case. It takes each one's name and line.
Comments and strings are tokens of their own, so they cannot match.

A SUB that is a test by name but takes parameters gets a note in `--list`'s text output (`TestWith takes
parameters: not a test`), since that is likely a mistake. No note in JSON.

### D6. Running and output

`crates\driver\src\unit_tests\run.rs`:
- **Builds**: one per test file, in a thread each, up to `--jobs`. One process per test, with a pool of `--jobs`
  threads (default `std::thread::available_parallelism`), with no new dependency.
- **Timeout**: `try_wait` polled every 10 ms, then `kill` of the test's process only. A child it started (a
  `SHELL`) is not killed with it.
  - A Windows job object, or a process group on Linux, would end the children too. No crate wraps the Windows API
    here, so it would be about 40 lines of hand-declared `kernel32` calls in `unsafe`, kept for good, or a new
    dependency, for a rare case: a test that times out with a child still running.
  - Left as a `SOMEDAY.md` row for both systems (pragmatic engineer on the proposal, 2026-10-29).
- Each test gets a fresh `QB64RUST_TEST_TEMP` folder under its build folder, removed after a pass and kept after
  a failure, with its path printed.
- **Text output** is `crates\driver\src\unit_tests\report.rs`.
- **JSON lines**: `{"event":"test","file":…,"name":…,"line":…}` per test found, then
  `{"event":"result","file":…,"name":…,"outcome":"pass|fail|error|timeout|build-error","message":…,`
  `"location":{"file":…,"line":…}|null,"output":…,"ms":…}` per result, then `{"event":"summary",…}`.
  It is written with a small escaping helper. `json_string` of `crates\codegen-cpp\src\debug.rs` moves to
  `crates\base` so both use one. No `serde`.
- **Exit status** 0/1 as the spec says. Usage errors use the CLI's existing message form.

### D7. Test Explorer

`vscode\src\testing.ts`, wired up in `extension.ts`:
- One `TestController`. At activation and on save, create and delete of `**/*_test.bas`, it runs
  `qb64rust test --list --format json <file>` per changed file and keeps a file item with its test items, each
  with a range at its line.
- **Run profile**: one `qb64rust test --format json --filter …` per request, with one filter per chosen item.
  Its JSON lines are read as they come, and each `result` is mapped to `passed`, `failed`, `errored` (for
  `error`, `timeout` and `build-error`) with a `TestMessage` at `location` or at the SUB.
- `output` goes to `run.appendOutput`. Cancelling kills the process (`runQueue.ts` already kills a build).
- The binary and toolchain come from `compiler\qb64rust.ts` and `toolchain.ts` as Build's do. When missing, the
  controller shows no items and the language status's message and action.
- The JSON parsing and filter building are pure functions in `vscode\src\compiler\testOutput.ts`, with unit tests
  in `vscode\test\unit\testOutput.test.ts`. There is one integration test (`vscode\test\integration\suite`) that
  lists and runs a fixture with one passing and one failing test, skipped like the others when
  `QB64RUST_TEST_QB64RUST` is unset.

### D8. Checked against the old compiler

`crates\driver\tests\unit_tests.rs` is a CLI test, `#[ignore]`d like the other clone-dependent ones. It writes the
entry point and library for a fixture to a temp folder, builds it with `qb64pe.exe`, and runs each test.
- The result file's lines must equal those of the `qb64rust` build, for a pass, a library failure and an
  `END`.
- For a runtime error, only the stderr message must match. The exit code is QB64pe's 0, and the missing `DONE`
  still makes it an ERROR, which shows that the protocol does not rest on D-012.

## Risks / Trade-offs

- **[Risk]** Scripts or CI of users that run a `qb64rust` program and treat any non-zero code as fatal now stop
  on a program that ended with an untrapped error. → That is D-012's intent. It goes in the release notes and in
  the D-012 row.
- **[Risk]** The runner's rule could hide a wrong exit code. → The rule only maps 0 to 1 when the trace names
  D-012, and is tested both ways in `test_run_legacy_tests.py` (the spec's scenarios).
- **[Risk]** Killing a timed-out test on Windows leaves a `SHELL`'d child running, which keeps the test's folder in
  use (as `g12_game.files` showed). → Accepted for phase 1 (D6). The test's
  temp folder is kept after a timeout and its path printed, and the row in `SOMEDAY.md` names the fix.
- **[Trade-off]** One C++ build per test file per run (a few seconds each) with no pass cache. → Builds run in
  parallel. A cache keyed on the C++ fragments is a `SOMEDAY.md` row.
- **[Trade-off]** The `_ASSERT` message is taken from the last stdout line. A test that prints from another thread
  could misplace it. → BASIC programs have one thread. The line and file come from stderr and are exact.
- **[Risk]** The refactor of `run()` into `compile.rs` changes the `-x` path. → It moves code without changing
  it. The CLI tests and the closing runs are the check.
- **[Risk]** Library names (`AssertTrue`, `Fail`, `TestTempDir$`) clash with the code under test. → The compiler's
  error for a name defined twice says which. The names are documented. The internal ones carry `qbtest_`.

## Migration Plan

D-012 needs none for this repository: the runner's rule covers the recordings, which are not re-recorded. For
users the change is the exit code above. The test command is new. Rollback is reverting the edit and the rule,
which are independent of the test command, apart from D8's protocol check.

## Open Questions

- The exact wording of the text output's lines (spacing, time format): decided in the task that writes
  `report.rs`, pinned by its CLI test.
- Whether `--jobs` for builds and for test runs should be one number or two: one for now. Split if a slow build
  starves the tests.
