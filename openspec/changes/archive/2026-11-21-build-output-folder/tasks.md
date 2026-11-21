# Tasks

Standing rules: CLAUDE.md. What this change does differently: its last group is done in Civil War Strategy's
repo, and its records go there (CLAUDE.md working rule 2).

## 1. Reading `[build]`

- [x] 1.1 `Build` and `CopyEntry` in `crates/driver/src/settings.rs`, read by `parse_all` and returned by `load`
  beside `Project` (design D1, D2). `output` defaults to `build`; paths are joined to the settings file's
  folder, or to the source's folder when there is no file. Update the module's first comment, which lists the
  tables read. Verify: unit tests in `settings.rs` for no settings file, no table, `output` alone, `output = "."`,
  `copy`, a settings file one folder up, and a dotted key (`build.output = "b"`).
- [x] 1.2 The checks of spec `compiler/cli`, "A bad build table": an unknown key, `output` not a string or
  empty, `copy` not a list of strings, a `\` in a path, a `copy` entry that is or holds the output folder, and
  `copy` with `output = "."`. Each error has its line. Verify: a unit test for each, with the message and line.
  Added while building it: a `copy` entry outside the settings file's folder (absolute, or above it with `..`)
  is an error too, since its copy would land outside the output folder; the spec delta says so.
- [x] 1.3 Add the row to `docs/DECISIONS.md`: on 2026-11-21 the user chose `build` as the default output
  folder, the program run in the source's folder, and in the output folder when the project lists `copy`
  (after the comparison with Visual Studio C++, Cargo and .NET in proposal.md). Name design D3 and D5 as Claude's
  choices for the user to see. If `docs/DIVERGENCES-QB64PE.md` has rows for the command line, add one for the
  output folder (QB64pe: its own folder, `qb64pe.bas` line 1130); otherwise say in the row that QB64pe's default
  differed before. Verify: the rows name the change and the date.

## 2. The compiler

- [x] 2.1 `default_paths` in `crates/driver/src/main.rs` (program and run folder, design D2 and D4), and `run`
  reordered so that the settings are read before the path is chosen. A bad settings file removes nothing.
  Verify: `cli.rs` tests for the scenarios of "The output folder" (the wasm one skipped when the wasm toolchain
  is missing, as `wasm_target.rs` does) and for `--no-settings` building into `build` beside the source.
  Done: `the_output_folder` sees the paths through `-z`, which writes its C++ beside where the program would
  go, so it needs no toolchain; the wasm scenario is in `wasm_builds_and_runs`. A bad settings file still
  removes a program named by `-o`, whose path is known (`a_missing_copy_entry`). The merged spec
  `compiler/wasm-target` said `hello.js` is beside the source; a delta for it was added.
- [x] 2.2 `crates/driver/src/output.rs`: the copy of design D3, called by `run` before `compile_exe` when there is
  no `-o`. First check in the Rust documentation and in `library/std/src/sys` what `std::fs::copy` does with the
  modification time on Windows and Linux, and write the source in the module's comment (CLAUDE.md working rule
  17). Verify: unit tests in `output.rs` with times set by `File::set_modified`: a missing file is copied, a
  newer source goes over the copy, a newer copy is kept, equal times copy nothing, a folder is copied file by
  file into its own folders, and a file deleted from the source stays. A `cli.rs` test for "A missing entry":
  status 1, the message has the line, and nothing is built. `cargo test -p qb64rust-driver`.
  Found: on Linux `std::fs::copy` copies the contents only, so the copy gets the time of the copy; on Windows it
  is `CopyFileExW`. So `output.rs` sets the copy's time to the source's itself, and writes the bytes rather
  than calling `std::fs::copy`, which would also copy a read-only attribute.
- [x] 2.3 `--print-paths` (spec `compiler/cli`, "The paths of a program"), with `--target` and `--no-settings`,
  and its line in the help text. Verify: `cli.rs` tests for both scenarios, for "The folder a program runs in",
  and for a bad settings file (status 1, only the error printed).
- [x] 2.4 The sweep: every caller of `qb64rust -x` or `-c` without `-o` that looks for the program beside the
  source. Search `crates/driver/tests`, `tools`, `justfile`, `.github`, `vscode` and the game repos
  (`..\world-war-2`, `..\civil-war-battleset`, `..\civil-war-strategy`, `..\wars-of-napoleon`, and their CI
  files). List what was found in this task, and fix each: in this repo here; in a game repo as a change there,
  named in this task. Verify: `cargo test -p qb64rust-driver` passes, including the tests that check a stale
  program is removed (`cli.rs` around lines 671 and 684, which now look in `build`), and the list is here.
  Found (2026-11-21):
  - `crates/driver/tests/it/cli.rs`: `default_exe` now names `build/<stem>`, and a helper `write_stale`
    writes an older program there. Fixed with it: `generate_cpp_only`, `debug_info_dump`, `fixes_errors`,
    `a_named_folder_that_is_not_qb64pe`, `no_toolchain_found`, `linux_no_toolchain_found` (`build/p`),
    `linux_default_name`, `linux_windows_only_metacommands`, `linux_shared_object_errors_at_the_start`,
    `embedded_file_is_missing_or_a_folder`, `exe_icon_file_is_missing`, `internal_compiler_error`, and the
    wasm tests (`build/p.js`; `wasm_builds_and_runs` now gets `data.txt` into `build` through `copy`). The
    tests at lines 671 and 684 pass `-o` and did not change.
  - `tools` (`run_legacy_tests.py`, `sandbox/run.py`, `tools/screen`), `justfile`, `.github`: every compile
    line passes `-o`. Nothing to change.
  - `vscode`: `buildArgs` and the QB64pe build pass `-o executableFor(file)`; changed in group 3.
  - The game repos: the CI files (`.github/workflows/build.yml`), READMEs, `docs/devnotes.md` and
    `tests/play.py` of all four pass `-o`; so does Civil War Strategy's `tools/playtest.py`, which 4.1
    removes. Nothing to change there for this task.
- [x] 2.5 `docs/MANUAL.md`: "Compiling and running a program" (line 52) says the program goes into `build`; a
  `[build]` entry in "The settings file" with `output`, `copy`, `output = "."`, where the program runs and why,
  the time rule, that a deleted file stays, a double click on the program, and the first version with
  `[build]`; `--print-paths` in the table of options. `docs/QUICKSTART.md` lines 74 and 154. Verify: build the
  manual's example with `qb64rust -x` in a scratch folder, and the files are where the manual says.
- [x] 2.6 No page of `docs/reference` describes building (checked: `coming-from-qb64pe.md`, and the `fixes`
  and `lints` chapters name `qb64rust.toml` only for their own tables). Verify:
  `python tools/reference/reference.py --check` passes.
  Wrong, found by the review (finding 4): the chapter `wasm-target.md`, "Building and running", said `game.js`
  is written beside the source. It now names `build` and `[build]` `copy`; `render` and `--check` pass.

## 3. The extension

- [x] 3.1 `programPaths(file)` in `vscode/src/compiler/qb64rust.ts`: `qb64rust --print-paths`, falling back to
  `executableFor` and the source's folder when it fails (design D5). `buildArgs` passes no `-o` when the paths
  came from the compiler. Verify: unit tests in `test/unit/qb64rust.test.ts` for the arguments and the paths,
  with and without the fallback; `npm run test:unit`.
  Done: `programPaths(binary, file)` with the pure parts `printPathsArgs`, `parsePrintPaths` and
  `fallbackPaths`, so the unit tests need no compiler. `planQb64rust` asks once and the plan carries the paths.
  `npm run test:unit`: 155 passing.
- [x] 3.2 Build, the Run command and Build and Run (`build.ts`), and the run task (`tasks.ts`), use
  `programPaths`: the program's path and `run_in` as the working folder (design D6). Verify: integration tests
  that run a program writing a file by its name alone: with no settings file, the program is in `build` and the
  file is beside the source; with `copy = ["x.txt"]`, the file is in `build`.
  Done: Run and the run task ask through `runPaths` in `buildPlan.ts` (QB64pe chosen or no binary: next to the
  source). In `buildQb64rust.test.ts` the first test now checks `build` and the file beside the source, and a new
  test "a copy list" checks `build/x.txt`, `build/w.txt` and the run task's folder; `debug.test.ts` and
  `firstRun.test.ts` look for the program in `build`. The tests compile and lint clean, but the integration suite
  has not run yet: it runs in CI's extension job (`vscode-extension.yml`), not on the host.
- [x] 3.3 The debugger: `resolveLaunch` leaves `cwd` unset for a `.bas` program without one, and `debug.ts` sets
  it to `run_in` after the build (design D6). Verify: `test/unit/debugConfig.test.ts` cases for a `.bas` and an
  `.exe` program without `cwd`, and for a `cwd` that is given.
  Done: `Builder.debugBuild` returns the program with its folder (`Built`).
- [x] 3.4 Measure the time of `qb64rust --print-paths` on Civil War Strategy's `cws.bas` (10 runs) and write it in
  this task (design, "Risks"). Verify: the number is here.
  Measured 2026-11-21 on the host, the `dev` build (optimized): 14.6 ms on average, 12.4 ms at least, 25.3 ms at
  most (the first of the ten). Run asks once before it starts the program.
- [x] 3.5 `vscode/README.md` lines 34 and 148: where Build writes the program, and the run and debug working
  folders. Verify: the README matches the `editor/build-run` and `editor/debug-launch` deltas.

## 4. Civil War Strategy uses it (in its own repo)

- [x] 4.1 In `..\civil-war-strategy`: `qb64rust.toml` gets `[build]` with the `copy` list of design D7,
  `.gitignore` gets `build/`, and `tools/playtest.py`, `tests/test_playtest.py` and the README paragraph on them
  are removed. The README's build lines lose `-o cws.exe`, and its "Run the program from this folder" paragraph
  says the program is built into `build` and runs there. Verify: `qb64rust -x cws.bas` writes `build/cws.exe`
  and copies the four entries; playing a turn and saving changes `build/CWS.CFG` and writes `build/cws1.sav`,
  and `git status` in the game's repo shows no change to `CWS.CFG`, `HISCORE.CWS` or a `.sav`.
  `python tests/play.py check two-player` still passes, since it builds with `-o`.
  Done 2026-11-21 on the game's branch `qbjs-web`, on top of the user's uncommitted work there (user's choice).
  `tools/playtest.py` and `tests/test_playtest.py` were never committed; they were deleted. `.gitignore`'s
  `playtest/` line became `build/`, so the old `playtest\` folder (earlier playtests' settings and saves) now
  shows as untracked; it was left for the user. `qb64rust -x cws.bas` wrote `build/cws.exe` and copied the four
  entries (`graphics` 10 of 10 files, `scenarios` 9 of 9); `--print-paths` gives `build` as the run folder;
  `play.py check two-player` passed (58 checkpoints); `git status` shows no `build/`. Not checked: playing a
  turn by hand and saving. `CWS.CFG` and `HISCORE.CWS` were already modified there before this task.

## 5. Closing

- [x] 5.1 `just quick` in the background; say what ran and what stood. Update `STATUS.md`'s "Next".
  2026-11-21: 9 passed (fmt, build, clippy, clippy-linux, nextest, lists, doc, reference, slice), 4 stood
  (tools, tools-check, libqb-copy, v37-libs), upstream and verification left for `check full`. The first run
  found a nested `if` for clippy and the new code unformatted; both fixed before this run.

## 6. Fixes from the review (2026-11-21)

- [x] 6.1 Finding 2 and 5: the check that no `copy` entry is or holds the output folder compared the paths as
  written, so `output = "../proj/data/out"` or an absolute one got past it, and the copy then went into itself
  without end. It now compares the paths joined to the settings file's folder and made plain (`resolve_build`
  in `settings.rs`, called by `load`), which also stops an absolute `output` from matching an entry by name.
  Unit test `copy_entries_never_hold_the_output_folder`; a scenario in the `compiler/cli` delta.
- [x] 6.2 Finding 1: after "Build with QB64pe", Run and the run task looked only in `build`. `runPaths` now takes
  the newer of the compiler's program and the one next to the source (`newerProgram`). Unit tests, a case in the
  integration test of the QB64pe fallback, and a scenario in the `editor/build-run` delta.
- [x] 6.3 Finding 3: a failed `--print-paths` other than an older compiler's unknown option is said in the
  "QB64" output (`printPathsProblem`, `ProgramPaths.problem`); the build still goes on next to the source.
- [x] 6.4 Finding 4: the reference chapter `wasm-target.md` (task 2.6).
- [x] 6.5 Finding 6: `--print-paths` refuses `-z`, `-p`, `--dump`, `--debug`, `--coverage` and
  `--trace-values`, which it would have ignored. CLI test `print_paths`. Found by a second review: with `-w` the
  note "settings from <file>" came before the JSON; it is left out under `--print-paths` now, and the test calls
  it with `-w`.
- [x] 6.6 Finding 7: the paths are asked in `planBuild`, so a Test Explorer run starts no `--print-paths`.

## Workflow follow-up

- Archive the change after the user has reviewed it, merging the four deltas into `openspec/specs/`. Then name
  spec `compiler/cli` "The output folder" in the `src:` comment of `docs/reference/chapters/wasm-target.md`,
  "Building and running", where a second comment asks for it, and run `reference.py render` and `--check`.
