# Tasks

Standing rules: CLAUDE.md. What this change does differently: wasm runs of more than 25 programs go in the
suites VM, and their length is said before they start (design D10); nothing is recorded for the wasm platform
(design D7); a libqb edit is always inside a test for Emscripten (design D4).

## 1. The SDK and what was not measured

- [x] 1.1 Install the Emscripten SDK in `..\emsdk` with `emsdk install` and `emsdk activate` (the user allowed
  the download, `DECISIONS.md` 2026-11-19). Take the newest release `emsdk` offers; if it is not 6.0.12, change
  the number in the delta specs `compiler/toolchain` and in the design. Add `..\emsdk` to `CLAUDE.md`, "Where
  things live", and to `LAYOUT.md`. Verify: `emcc --version` prints the release, and the folder's size is in
  the design's new "Measured" section.
- [x] 1.2 Measure with the SDK alone, in a scratch folder under `target\`: `emcc -dM -E` for `__linux__`,
  `__EMSCRIPTEN__` and `__wasm32__`; the size of `long double` and of a pointer; the time of one `emcc`
  compile of `libqb\internal\c\libqb.cpp` with console-only flags (it may fail to compile; the time to the
  first error still counts). Verify: each result is a line in "Measured".
- [x] 1.3 A C++ program of a few lines that starts two threads from `main`, blocks `main` in a loop as
  `MAIN_LOOP` does, reads a file by a relative path and calls `exit(3)` from a thread. Build it with the link
  flags of design D4 and run it under the SDK's Node from another folder. Verify: it prints from both threads,
  reads the file, and the process ends with status 3; the flags that were needed are in "Measured", and D4 is
  revised if they differ.
- [x] 1.4 Copy the SDK folder to another path and run 1.3's build there with `EM_CONFIG` set. Verify:
  "Measured" says whether a copied SDK works, and design D8 names the way that follows.

## 2. The target as a value

- [x] 2.1 `Platform::Wasm` (design D1): `exe_extension`, `name`, and an arm at every `match` on `Platform` in
  the workspace. Verify: unit tests for the names; `cargo test -p qb64rust-base`; `cargo build --workspace`
  has no non-exhaustive match.
- [x] 2.2 `--target <name>` in `main.rs` (`parse_args`, `USAGE`) and its use in `compile.rs` and `lib.rs`; the
  default output `.js`, `-o` held to `.js`, and the options a wasm build refuses (spec `compiler/cli`, the
  three new requirements). Verify: CLI tests for an unknown name, `--target host`, a wrong ending of `-o`, and
  each refused option; `cargo test -p qb64rust-driver`.
- [x] 2.3 `$IF` flags for the target (design D6) in `syntax\src\pp.rs`. Edit the reference page of `$IF`
  (`docs\reference\entries\`) for `WASM`, then `reference.py render` and `--check`. Verify: unit tests of
  `PpFlags` for all three platforms; the page names `WASM`.
- [x] 2.4 The target's rejections (design D5): `crates\sema\src\target.rs` with `WASM_UNAVAILABLE`, called
  from `check_program`. Verify: a test per scenario of spec `compiler/wasm-target`, "Console programs only",
  "What the target does not have is rejected" and "Pointer-sized types wait for their width", each also with
  the host as the platform, where no error is reported; `cargo test -p qb64rust-sema -p qb64rust-driver`.
- [x] 2.5 `-z --target wasm` writes Linux's fragments (spec `compiler/wasm-target`, "The C++ is the same as
  for Linux"). Verify: a `codegen-cpp` or driver test that compares the fragments of three console programs
  for `Wasm` and `Linux`, byte for byte; the call-site check's Windows fragments of the slice list equal the
  commit before.
- [x] 2.6 A chapter on the wasm target in `docs\reference\chapters\`: how to build and run, console programs
  only, the words it rejects (read from `WASM_UNAVAILABLE`'s list), `$IF WASM`, `_OS$`, and that it is
  experimental. Edit the page of `_OS$`. Then `reference.py render` and `--check`. Verify: `--check` passes,
  and the chapter's word list equals the table's.

## 3. The SDK lookup and libqb for wasm

- [x] 3.1 `toolchain::find_emsdk`, `EMSDK_RELEASE`, the message, and `toolchain path|install --target wasm`
  (design D2; spec `compiler/toolchain`, the three new requirements). Verify: unit tests on fake SDK folders
  for option, variable, none, and another release; CLI tests for the two subcommand forms.
- [x] 3.2 The wasm copy of libqb (design D3): `wasm_libqb`, `sync_wasm_libqb`, its lock and stamp. Verify:
  unit tests on a fake tree: a changed source is copied, a removed one is removed, an object is kept, and a
  second sync copies nothing; `libqb\` holds no new file after a wasm build.
- [x] 3.3 The edits of design D4 and `build_with_make` for `Wasm`, until this program builds and runs:
  `$CONSOLE:ONLY`, prints an integer, a `SINGLE`, a `DOUBLE`, a `_FLOAT` and a string, and ends with
  `SYSTEM 3`. Each edit file's header says why it is needed. Run `copy_libqb.py` and `--check`. Stop and
  report if the edits pass 40 or one is more than a branch or a stub in `qbs`, `evnt` or file I/O (design,
  Risks). Verify: `qb64rust -x --target wasm` builds it, `node` prints the Linux recording's text and exits
  with 3; `libqb\SOURCE.md` lists the edits; the count of edits and the build's time are in "Measured".
- [x] 3.4 `_OS$` under Emscripten (design D4) and a CLI test, ignored unless the SDK is found, that builds and
  runs the programs of spec `compiler/wasm-target`, "A wasm build" and "The target's name in a program".
  Verify: the test passes with `QB64RUST_EMSDK_ROOT` set and is skipped with a reason without it.
- [x] 3.5 The other platforms compile the same code. For each file an edit touches, compare the preprocessed
  text (`c++ -E` with the Windows build's flags) before and after the edits, without line markers. Tell the
  user that the edits make the next Windows `full` run a cold one. Then `just quick`. Verify: the texts are
  equal for every edited file; `just quick` passes; the `libqb-copy` step passes.

## 4. The runner

- [x] 4.1 `--platform wasm` in `run_legacy_tests.py` (design D7): `judged_as`, the compile and run lines, the
  refusal of `--record`. Update `tools\legacy_tests\README.md`. Verify: tests in `test_run_legacy_tests.py`
  for the files chosen, the exit status rule, the run line, and the refusal; one verification program run by
  name on the host passes.
- [x] 4.2 `known.wasm.list`: reading it, the set of reasons, the skipped reasons, the check of the lists
  (`inputs.rs` or the runner's own check, wherever `known.linux.list` is checked now). Verify: tests for a
  skipped `window` line, a reason outside the set, and a listed program that passes.

## 5. The slice list on the host

- [x] 5.1 Choose `tests\wasm\slice.list` (spec `testing/golden-corpus`, "The wasm slice list"): at most 25
  console programs covering the kinds the spec names. Add `tests\wasm\README.md` saying what the list is for
  and how to run it, and a `LAYOUT.md` row. Verify: each kind the spec names has a program, named in the
  README.
- [x] 5.2 Run the list with `--platform wasm --jobs 4` on the host. Fix what is a fault of an edit or a flag;
  a program that differs by design is replaced on the list and noted for 6.3. Verify: every program on the
  list passes; the time per program is in "Measured", with the estimate for the whole family that follows
  from it.

## 6. The whole family

- [x] 6.1 The SDK in the suites VM and the two `check` steps (design D8): `pins.psd1`, the zip, `suites.ps1`,
  `steps.py`, the "not run" rule; `tools\winvm\README.md` and `tools\check\README.md`. Verify: `tools\check`'s
  tests for the plan of each layer and for a missing SDK; `check wasm-slice` passes on the host; `check
  wasm-verification --where vm --plan` prints the VM command.
- [x] 6.2 Write `tests\verification\known.wasm.list`'s skipped lines before the run: `window` for each
  program without `$CONSOLE:ONLY`, and `target` or `offset` for each that `-z --target wasm` rejects (a script
  over the suite on the host, checking only, no builds). Verify: the counts of the three reasons are in
  "Measured", and they sum with the programs left to the suite's size.
- [x] 6.3 Say how long the run will take (from 5.2) and why that size, then run `check wasm-verification
  --where vm` once. Ask first if the estimate is over 3 hours. Verify: the run's log is kept; "Measured" has
  the counts: pass, fail, skipped by reason.
- [x] 6.4 Sort every failure (design D9). Fix the faults of edits and flags, running only the failed programs
  again by list. Add a `DIVERGENCES-QB64PE.md` row for each kind of defined difference, naming the wasm
  target, and put its programs on `known.wasm.list` as `float` or `clib`; others as `node`, `varies` or
  `open` with what was seen. Add the family's line to `verification\README.md` where a difference is a new
  measured fact. Verify: no failure is left without a fix or a line; each row's programs are named in it.
- [x] 6.5 Run the whole family once more in the VM, after saying its length. Verify: no failing program;
  "Measured" has the closing counts and the share that passes of the programs that ran.

## 7. Closing

- [x] 7.1 `openspec validate wasm-console --strict`; read the delta specs against what was built and correct
  either. Verify: validation passes, and each scenario names a test or a recorded program in a table at the
  end of the design.
- [x] 7.2 `just quick` in the background; say what ran, what stood and why. Update `STATUS.md`'s "Next" with
  the result in two lines and the next step (W0.5). Add `SOMEDAY.md` rows for what this change left: a CI job
  for the wasm slice, a wasm build on a Linux host, the SDK download. Verify: `just quick` passes;
  `check_repo.py` finds no local path in the new files.

## Workflow follow-up

- Archive the change after the user's review, and sync its delta specs into `openspec\specs\`. Then the
  reference's `src:` comments that name `openspec\changes\wasm-console\specs\...\spec.md` (the check reads only
  merged specs) name the requirement instead, and `reference.py --check` runs again.
- Before the archive, make design D8 and `tools\winvm\suites.ps1` agree on how the SDK gets into the VM. D8
  says a zip in `%LOCALAPPDATA%` is unpacked at another path, which task 1.4 measured to work. `suites.ps1`
  copies `..\emsdk` with `tar` and refuses unless the SDK is beside the tree, giving task 1.4 as the reason the
  path must be the same. Check which is true, then correct the design's text or the script's comment.
  Dropping the rule that the SDK be beside the tree would change behavior, so it would be a change of its own.
- At the merge into `main`: `ROADMAP.md`'s M9 table marks W0 done.
