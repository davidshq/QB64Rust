# Design

## Context

See `proposal.md` for motivation and `specs/testing/golden-corpus` for requirements. The source programs are in
`<qb64contain>\QB64Fresh\tests\runtime_comparison\` (commit `85e6df8`): 261 `.bas` files numbered 01–264 with
gaps, plus `README.md`, `run_comparison.sh` and `diff_results.sh`.

### What the programs touch (every program read in full, 2026-10-03, task 1.1)

All 261 are plain ASCII with LF line ends, 2,624 lines in all. The folder is unchanged between `85e6df8` and
QB64Fresh's current `HEAD`.

| Feature | Programs | Consequence |
|---|---|---|
| `$CONSOLE:ONLY` | all 261 | Headless; stdout can be redirected. The "screen `PRINT` with a comma hangs" problem (`study\00` §10) needs a screen, so it does not apply. |
| Keyboard `INPUT` / `INKEY$` | none (the 14 `INPUT` hits are `OPEN … FOR INPUT`, `LINE INPUT #`, `INPUT #`); no `SLEEP` or `_KEYHIT` either | stdin can be the null device, except for `END` (next row). |
| `END` | all 261 (`v11_wrap_o2` ends with `SYSTEM`) | On Windows, `END` in a console program prints `Press any key to continue` and waits for a **console key event** (`sub_end`, `libqb.cpp`; `func__getconsoleinput`, `libqb\src\console.cpp`). The null device or a pipe never sends one, so the program hangs (found in task 2.1; QB64Fresh ran the corpus on Linux, where `END` reads one byte from stdin and gets end-of-file). The runner feeds key presses (D2). The trailer (an empty line, then `Press any key to continue` with no line end) is part of every recorded `.output` of a program that reaches `END`. |
| Files and folders created in the working directory | 05, 52–54, 59, 60, 86, 95–97, 174–183, 213, 214, 219 (all relative names; 213 does `CHDIR ".."` back to its own folder; none reaches outside it) | Each program must run in its own empty folder, never inside the repo. |
| **Real side effect** | `LPRINT` (239): the compiler then enables `DEPENDENCY_PRINTER`, and on Windows the runtime sends the page to the **default printer** (`sub__printimage`, `internal\c\libqb.cpp`) | Compile only, never run (`.norun`, D4). |
| Machine-dependent output | `LEN(ENVIRON$("PATH"))` (212), `TIMER` (234), `FRE` (218) | `.normalize` sidecar (D4); 218 decided by the double run. |
| Possibly machine-dependent | `COMMAND$` (217, empty with no arguments, D2), `CSRLIN`/`POS` on redirected output (237), `SHELL "echo after"` (215, runs `cmd`), `CHAIN` to a missing program (238), `WIDTH 10` (189) | Decided by the double run. |
| `RANDOMIZE n` / `RND` | 56, 232, 233 | Seeded; deterministic per runtime. Recorded as is. |
| Probably rejected by QB64pe | `TRIM$` (16, QB64 has `_TRIM$`); `BYREF` (47, 190, 191); a parameter or field named `name` (48, 57, `NAME` is a statement); SUBs before the main code (192, 193, 196); array field in a TYPE (197) | Recorded as `.err` if the compile fails; still valid tests. |
| Derived from QB64pe's own tests | 240 (`compile_tests/console_only/data.bas`), 241, 242, 243, 246, 247; 250 inspired by `qbasic_testcases/misc/rot13.bas` | Noted in `SOURCE.md`; QB64pe is MIT-licensed too. |

The old compiler and its build: `..\QB64pe\qb64pe.exe` at `16f629784e` (the commit behind `baselines\`).

## Goals / Non-Goals

**Goals:**
- One command records the corpus; one command checks a compiler against it. Both work for the old compiler now and
  for the new one later (same command-line contract as the legacy runner: anything that accepts qb64pe's flags).
- Recorded files are byte-exact program output, reproducible on a rerun, free of local paths.

**Non-Goals:**
- Fixing, rewriting or extending the programs. They are copied unchanged; gaps in coverage are noted, not filled.
- Judging whether the old compiler's output is *right*. The corpus records what it does; disagreements with
  QuickBASIC or with intent go to the divergence register (next change).
- Running the corpus in CI (needs a QB64pe build in CI; later).
- macOS/Linux recording.

## Decisions

### D1. Layout follows QB64pe's `compile_tests`
```
tests\corpus\
  README.md                     what the corpus is, how to run and record, -O2 differences (D6)
  runtime_comparison\
    SOURCE.md                   origin (QB64Fresh 85e6df8, MIT, rule 5), what was not copied
    01_string_ops.bas
    01_string_ops.output        expected merged stdout+stderr of the run
    …
    212_environ_dollar.normalize
  verification\
    v11_wrap_o2.bas             copy of verification\v11_wrap_o2.bas
    v11_wrap_o2.output
```
`<name>.output` means "must compile, run, and print this"; `<name>.err` means "must fail to compile with this
compiler output". The same convention as `compile_tests` means the runner's comparison code is reused and the
two suites read the same way. *Alternative:* one JSON file per program with fields. Rejected: plain files diff well
in git and are what the existing runner already handles.

### D2. Runner: a `corpus` suite in `run_legacy_tests.py`
Add `--suite corpus` (included in `all`), with `--corpus-root` defaulting to `tests\corpus` of this repo, and
`--record`. It differs from `compile_test` in four ways, each deliberate:
1. **Build settings:** no `-f:OptimizeCppProgram=true`. Flags are `-q -m -x`, the default build (D6 explains the
   `-O2` pass).
2. **Working directory:** each program is copied into a fresh empty folder under the results folder and compiled
   and run there, so files it creates (`rt_*.txt`, directories) never touch the repo and cannot leak between
   programs. The folder is deleted after a pass and kept after a failure.
3. **Arguments and environment:** the exe gets no arguments (the legacy runner passes the results path, which
   would put a local path into `COMMAND$`). Environment as the legacy runner plus `QB64PE_NOPROMPT=y`. On Linux
   and macOS stdin is the null device. On Windows the program needs a console to get past `END` (table above):
   the runner starts `tools\legacy_tests\press_any_key.py` with a console of its own and no window
   (`CREATE_NO_WINDOW`), stdout and stderr going to the result file; the helper starts the program in that console
   with stdin `CONIN$` and writes a Shift key press into the console input every 50 ms until the program exits.
   `END` empties the input buffer and then waits, so the next press ends it. This is safe only because no corpus
   program reads the keyboard; a later group that does needs another way. *Alternatives rejected:* editing the
   programs (`END` → `SYSTEM`) breaks D1's unchanged copies; killing the program after its output stops loses the
   trailer (written with `std::cout`, flushed only at exit) and waits for a timeout on every program.
4. **Timeout** 60 s per run (all programs are small; a hang is a failure, recorded as such).

The shared helpers (`run_proc`, `kill_tree`, `clear_temp`, `norm_output`, `norm_text`) are reused, not copied.
The script's name stays (it is referenced from `STATUS.md`, `baselines\README.md`, `study\00`); its docstring gains
the corpus suite. *Alternative:* a separate `tools\corpus\run_corpus.py`. Rejected: it would duplicate process
handling that already copes with Windows process trees and timeouts.

### D3. Recording
`--suite corpus --record` compiles every program **once** and runs the executable **twice** with the old compiler
(non-determinism lives in the program's output, not in the compile), and writes `<name>.output` (or `<name>.err`
if the compile fails) only when both runs agree after normalisation. If they
disagree, nothing is written and the program is reported as non-deterministic; the fix is a `.normalize` sidecar
(D4), never editing the program. Writing an expected file whose content differs from the existing one prints a
warning; git shows the diff. Runtime errors are part of the recorded output: with `QB64PE_NOPROMPT=y` the runtime
prints them to stderr, which the runner merges into stdout (the exit code is always 0, `study\00` §10).

### D4. Normalisation sidecar
`<name>.normalize` holds one rule per line: a Python regular expression, a tab, a replacement (`#` lines are
comments). Rules are applied, in order, to each line of the actual output before comparison and before recording.
Example for `TIMER`: `^timer: *[0-9.]+ *$	timer: <TIMER>`. `PRINT` writes a positive number with a leading and a
**trailing** space (`timer: 51234.56 `), so a rule anchored with `$` must allow trailing spaces. Rules must be narrow (anchored, one value each) so that a
normalised program still checks everything else it prints. Programs whose output cannot be made stable this way
(for example if `CHAIN` misbehaves headless) are listed in `known_failures.txt` with the reason, as
`corpus:<group>/<name>`, rather than removed.

A `<name>.norun` sidecar (one line: the reason) makes a program **compile-only**: the runner compiles it, requires
success and an executable, and never starts it; there is no `.output`. Used for programs with side effects outside
their folder, so far only `239_lprint` (it would print a page on the default printer).

### D5. Byte-exact storage
`.gitattributes` gains `tests/corpus/**/*.output -text` and `tests/corpus/**/*.err -text`, so git stores the
recorded bytes unchanged: a Windows program's redirected console output has CRLF line ends, and with
`core.autocrlf=input` (set here) git would rewrite them to LF on commit. (None of the current programs prints bytes
128–255 or a bare `CHR$(13)`; the attribute also covers later groups that may.)
Comparison still uses `norm_output` (CRLF = LF, trailing newlines ignored), so a later checkout with other
line-ending settings does not break it.

### D6. The `-O2` pass
`--suite corpus --cpp-opt` compiles with `-f:OptimizeCppProgram=true` and compares against the same expected
files. Its failures are not test failures of the corpus; they are the list of programs whose behaviour depends on
the C++ optimiser (signed overflow and the like). The list goes into `tests\corpus\README.md` with one line per
program and feeds the divergence register. `v11_wrap_o2` is expected on it. Any `_INTEGER64` program on it answers
the open question in `study\16` §8.

### D7. Privacy check
Before staging, `grep` all recorded files for drive letters followed by `:\` or `:/`, `Users`, the user name and the
host name (`CLAUDE.md` rule 2). Anything that matches is normalised (D4), not committed.

### D8. Baseline
After recording, a check run (`--suite corpus` without `--record`) must pass for every program not listed in
`known_failures.txt`. Its `results.json` is saved as `baselines\qb64pe-16f629784e-win64-corpus.json`, matching
the existing baseline names; `baselines\README.md` gains a line.

## Risks / Trade-offs

- **Programs written against QB64Fresh's view of QB64pe.** Some use features QB64pe may reject (QB64Fresh-only
  names, `_` prefixes). → They are recorded as `.err`, which is still a valid test (the new compiler must reject
  them too); `tests\corpus\README.md` counts them.
- **The old runtime's output is sometimes wrong by QuickBASIC standards.** → Recorded anyway (D-goal); judgement is
  the divergence register's job.
- **Full runs take time:** 262 compiles of a few seconds each, so roughly 15–30 minutes per pass (record, check,
  `-O2`); not yet measured. → `--glob` already narrows a run; recording is rare.
- **Shared `internal\temp`** (`study\05`): compiles must be sequential. → The runner is sequential already.

## Open Questions

- Whether `CHAIN` (238) to a missing program behaves headless. Answered by the first recording run. (`LPRINT`,
  239, is settled: `.norun`.)
- Whether `SHELL "echo after"` (215) interleaves the child's line with the program's own output in a stable
  order. The double run may not catch a rare race; if the order ever flips, the program gets a `known_failures.txt`
  entry rather than a looser comparison.
