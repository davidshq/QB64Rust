# Proposal

## Why

Unit tests of BASIC code could not be made to work with QB64pe (`<qb64contain>\QB64pe\docs\unit-testing-plan.md`,
its appendix on the `more-tests` branch). A test that died of a runtime error still exited 0. A console program
with no `SYSTEM` waited at "Press any key". `'$CONSOLE:ONLY` written as a comment, or a missing `DEFLNG A-Z`, changed
what the test compiled to without a word. The runner had no timeout and could not run tests side by side. The
assertions skipped later checks, and the "coverage" matched file names. The new compiler removes the reasons behind
most of these: it owns the build, it has a line-coverage build and a process runner, and the exit code is ours to
fix (`QB64PE-DIVERGENCES.md` D-012, decided 2026-10-08, not yet built). What remains is to build D-012 and a test
command on top of it whose results cannot pass falsely.

## What Changes

- **D-012 built.** A program that ends because of a runtime error no handler takes exits with code 1. That covers
  `QB64PE_NOPROMPT=y`, "Continue? no" in the dialog, a critical error (300 to 315, a failed `_ASSERT`) and the
  other fatal ends of libqb. QB64pe's program exits 0. The program writes `D-012` to the run-time trace. The test
  runner expects exit code 1 in place of a recorded 0 for a program whose trace names D-012, in every suite. A
  program that goes on after the error (`QB64PE_NOPROMPT=continue`, "Continue? yes") is unchanged.
- **`qb64rust test`**, a new subcommand. It finds the test SUBs of `*_test.bas` files from the parse tree: a SUB
  whose name starts with `Test` and that takes no parameters. It writes a small entry point in BASIC that runs the
  test SUB named in the environment, builds each test file once, and runs each test in a process of its own, with
  a timeout and its standard input from the null device. A test is judged by a result file the program writes,
  and by its exit code, never by its printed output alone. A test passes only when the program wrote that the test
  SUB returned. It fails when an assertion failed. It is an error when a runtime error ended it, when it ended
  early (`END`, `SYSTEM`) or when it timed out. A run that finds no test fails. Output: text, or JSON lines for
  tools; `--list`, `--filter`, `--jobs`, `--timeout`, and the build options of a compile.
- **An assertion library** that the entry point includes: `AssertTrue`, `AssertFalse`, `AssertEqualInt`,
  `AssertEqualSingle`, `AssertEqualDouble`, `AssertNear`, `AssertEqualStr`, `Fail`. Every name is typed
  explicitly, so the test file's `DEFxxx` and `OPTION _EXPLICIT` cannot change it. A failed assertion records
  expected and actual and ends the test. `_ASSERT` works in a test too, and its failure is reported with its line.
- **Test Explorer in the extension.** It lists the tests of the workspace's `*_test.bas` files through
  `qb64rust test --list`, runs one test, a file or all of them through `qb64rust test`, and shows each result at
  its SUB.
- Phase 1 covers code that a test file can `$INCLUDE` (libraries split into `.bi`/`.bm` and the like). Testing the
  SUBs of one large program without running its main module is left for a later design (`SOMEDAY.md`), and so are
  graphics tests, debugging a test and coverage per test.

## Capabilities

### New Capabilities

- `compiler/unit-tests`: the `qb64rust test` subcommand. Test files and test SUBs, the entry point it writes, how a
  test is run and judged, the assertion library, its output and exit status.
- `editor/test-explorer`: the extension's tests in VS Code's Test Explorer: discovery, running, results at the SUB.

### Modified Capabilities

- `language/error-handling`: a program ended by a runtime error that no handler takes exits with code 1 (D-012).
- `language/console-and-window`: the untrapped-error scenario of "The end of a program and errors with a hidden
  window" exits with code 1 instead of 0.
- `language/asserts-and-debug`: a failed `_ASSERT` with a message no longer exits as the old compiler's program
  does: its exit code is 1.
- `testing/golden-corpus`: the runner's rule for D-012. A program whose trace names D-012 is expected to exit with
  1 where the recording or `.exitcode` says 0, in every suite, with no `.divergences` sidecar needed.

## Impact

- **libqb copy**: a new edit under `tools\libqb\edits\` (the fatal ends in `error_handle.cpp` and the "Continue?"
  path), offered upstream through `UPSTREAM.md`.
- **`crates\driver`**: the `test` subcommand (discovery through `crates\syntax`, the entry point, building, running,
  judging, output), the library embedded in the binary, CLI tests.
- **`tools\legacy_tests\run_legacy_tests.py`** and its tests: the D-012 rule. The verification recordings (133
  programs ending in "Runtime error", all `exit=0`) and about 35 corpus and upstream files are left as
  `qb64pe.exe` recorded them; the rule covers them.
- **`QB64PE-DIVERGENCES.md`** D-012: the value, the trace, the tests.
- **`vscode\`**: a test controller, its unit tests and an integration test.
- **Behavior visible to users**: scripts and CI that run a program built by `qb64rust` now see a non-zero exit code
  when it ends on an untrapped error. That is D-012's purpose, but it differs from QB64pe.
- **Not affected**: programs built with `qb64pe.exe`; the fixes switch (D-012 is a decided divergence, always on, as
  the 2026-10-26 rule keeps decided divergences).
