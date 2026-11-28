# Tasks

Standing rules: CLAUDE.md.

## 1. The output folder and the run folder

- [x] 1.1 Add the row to `docs/DECISIONS.md`: on 2026-11-27 the user decided that a build without a settings file
  writes the program beside its source ("the default should be in the same folder"), reversing the default of the
  2026-11-21 row, and asked for `qb64rust init` with both front ends and a way without questions. Name as Claude's
  choices, for the user to see: wasm keeps `build` (design D1), a `copy` list alone still means `build` (D1), all
  data files offered are the default (D5), New Project stays (D7). Done on 2026-11-27 with the removal of
  `QB64RUST_RUN_IN`, which the same row records. When the default is built, change the row's "Not built yet"
  sentence.
- [x] 1.2 `Build::output_for(source_folder, platform)` in `crates/driver/src/settings.rs`, with `output` kept as
  written (design D1). `default_paths` in `main.rs` and `output::prepare` and `output::copied_files` in
  `output.rs` call it. Verify: unit tests in `settings.rs` for no settings file, a settings file one folder up
  with no `[build]`, `call_trace` alone, `output` alone, `output = "."`, `copy` alone, `copy` with `output`, and
  each of these for the wasm platform.
- [x] 1.3 `default_paths` returns the program's folder as `run_in` (design D2). Verify: `cli.rs` tests for every
  scenario of spec `compiler/cli`, "The output folder" and "The paths of a program", seen through `-z` and
  `--print-paths` as `the_output_folder` does now; the existing tests that expect `build` without a settings file
  are changed, not deleted. A new test `a_program_runs_in_its_own_folder` (design D2) builds a program without a
  settings file, starts it from another folder, and finds the file it wrote beside its source.
- [x] 1.4 Search this repo for what assumes `build` after a build without `-o`: `crates/*/tests`, `tools`,
  `vscode/src`, `vscode/test`, `tests`, and the documents. Fix each. Verify: the search's command and its hits are
  listed here when done, and `cargo test -p qb64rust-driver` passes.
  Done: `grep -rnE "(^|[^a-z_-])build[/\\]"` and `grep -rnE "['\"]build['\"]"` over `vscode/src`, `vscode/test`,
  `tools`, `tests`, `docs/*.md` and `crates/*/{src,tests}`. Fixed: `default_exe` and the tests that use it in
  `cli.rs` (`build_exe` for a `copy` list), `builtOf` in `vscode/test/integration/suite/helpers.ts`
  (`builtInBuild` for a `copy` list), `buildQb64rust.test.ts`, the comment in `debug.test.ts`, `MANUAL.md`,
  `QUICKSTART.md`, `vscode/README.md` and `package.json`. Left as they are: the wasm paths (`build/web`,
  `build/<name>.js`), the unit tests' made-up JSON in `qb64rust.test.ts` and `emsdk.test.ts`, the workspace folder
  named `build` in `build.test.ts`, and the `tools/check` step named `build`.
- [x] 1.5 On Linux (WSL), build `maps.bas` beside a folder named `maps` and read the message. If it does not name
  the folder, make it do so. Verify: a `cli.rs` test, for Linux only, that the message names `maps`.
  Done differently: the build checks for a folder at the program's path before the compile, on every system, so
  the test `a_folder_of_the_programs_name` runs on Windows too, with a folder `maps.exe`. Not run in WSL.
- [x] 1.6 Documents of the default: `docs/MANUAL.md` (where the program goes; the paragraph on `output` says that a
  program in an output folder needs its data in `copy`; which line to add to `.gitignore`), `docs/QUICKSTART.md`,
  `vscode/README.md`, `vscode/CHANGELOG.md` (an old `build` folder can be deleted), and row D-073 of
  `docs/DIVERGENCES-QB64PE.md`. No page of `docs/reference/entries/` or `docs/reference/chapters/` describes the
  host default; `chapters/wasm-target.md` stays right, since wasm keeps `build`: read it to confirm. Verify: each
  example command in the edited passages was run as written.

## 2. The extension's test and the manual's paragraph

- [x] 2.1 In `vscode/test/integration/suite/buildQb64rust.test.ts`, "built by qb64rust" expects the program
  beside the source. Verify: that integration suite passes on the host.
- [x] 2.2 `docs/MANUAL.md` has a paragraph, added with `QB64RUST_RUN_IN` and kept without it, that says a program
  without a `copy` list runs in `build` when started outside the editor. Rewrite it with task 1.6: every program
  runs in its own folder. Verify: a search of the repo for `QB64RUST_RUN_IN` finds only `DECISIONS.md` and this
  change.

## 3. `qb64rust init`

- [x] 3.1 Check in the Rust standard library's documentation that `std::io::IsTerminal` is available in the
  toolchain `rust-toolchain.toml` pins, and name the source in the module's first comment (CLAUDE.md working
  rule 17). Verify: the comment names the page.
- [x] 3.2 One list of keys per table in `crates/driver/src/settings.rs` for `[project]`, `[build]` and `[format]`,
  which the "unknown setting" messages are built from (design D4). Verify: the existing tests of those messages
  pass unchanged.
- [x] 3.3 `crates/driver/src/init.rs`: `Plan::read` (design D3, D5, D6). Verify: unit tests on temporary folders
  for the scenarios of "The data files offered" and "The plan of a folder", a settings file here and one folder
  up, a `.git` folder, a `.git` file, and no git.
- [x] 3.4 `Answers` and the options, with their errors (spec "The answers and their options"). Verify: a unit
  test for each scenario there, with the message of each bad value.
- [x] 3.5 `render` and its three tests (design D4): every combination of answers is read by `settings::parse_all`;
  each commented setting uncommented is read; each key of 3.2 is in the template. Verify: the three tests pass,
  and the third fails when a key is taken out of the template by hand.
- [x] 3.6 `ask`, over a `BufRead` and a `Write` (design D3). Verify: unit tests that Enter at each question gives
  the answers of `--yes`, that a wrong answer is asked again, that an option's question is not asked, and that
  the questions asked for one `.bas` file, for several, with a build folder, and in a git work tree are those of
  spec "Questions" and "The answers and their options".
- [x] 3.7 `write`, the new project's three files (design D7) and the `.gitignore` line (D6). Verify: unit tests
  for "Init never overwrites", "A program that exists and a new project" and "The ignore file", and that a
  failure at the second file leaves none. The `.gitignore` line is printed, not written (design D6).
- [x] 3.8 The subcommand in `main` of `main.rs`, with `INIT_USAGE`, `--plan`, and the terminal check. Add `init` to
  `USAGE` and to the first comment of `main.rs`. Verify: `cli.rs` tests for "Init subcommand" (both scenarios),
  "Not a terminal" (input from a file), and `--plan`'s line; and one program built with the written file of a
  build folder, run, that reads its copied data file.
- [x] 3.9 Documents: a section "Setting up a project" in `docs/MANUAL.md` before "The settings file", two lines in
  `docs/QUICKSTART.md`, the row of `init.rs` in `docs/LAYOUT.md`. No word of the language changes, so no page of
  `docs/reference/` is edited. Verify: each command in the new section was run as written, and the files it
  printed are the ones the section shows.

## 4. "QB64: Set Up Project"

- [x] 4.1 `initPlan` and `initWrite` in `vscode/src/compiler/qb64rust.ts` (design D8). Verify: unit tests in
  `vscode/test/unit/qb64rust.test.ts` for the arguments of each answer and for output that is not JSON ("too
  old"). The tests are in `vscode/test/unit/setUpPlan.test.ts`, beside those of 4.2.
- [x] 4.2 `vscode/src/setUpPlan.ts`: the questions from a plan. Verify: unit tests for one program, several
  programs, a build folder with data files, no data files, and a git work tree.
- [x] 4.3 `vscode/src/setUpProject.ts` and the command in `vscode/package.json`, registered in `extension.ts`;
  the cases of spec `editor/project-setup`, "Folders the command does not set up". Verify: integration tests in
  `vscode/test/integration/suite` for "A program beside its data", "Every default", "Canceled", "A settings file
  is there" and "An empty folder", with the picks given in place of the lists as `newProject`'s test gives the
  folder.
- [x] 4.4 The integration test of design D7: `qb64rust init --yes` in an empty folder against `MAIN_BAS`,
  `WORKSPACE_SETTINGS` and the plain lines of `SETTINGS_TOML`. Verify: it passes, and fails when `all = true` is
  taken out of either side by hand. It passes; the run with a line taken out was not made, since the test compares
  the lines whole.
- [ ] 4.5 Documents: `vscode/README.md` (the command), `vscode/CHANGELOG.md`, the rows of the two new files in
  `docs/LAYOUT.md`. Verify: the README's steps were followed in a VS Code window on a folder with one program.
  The documents are written. Following the steps in a window is the user's: Claude cannot drive VS Code's pick
  lists.

## 5. Closing

- [x] 5.1 `openspec validate project-init --strict`, `just quick` in the background, and the extension's
  integration suite on the host (it opens no port; say how long it takes before starting it). Verify: each
  passes, and the reply says what ran and what stood.
- [x] 5.2 Update "Next" of `docs/STATUS.md`. Verify: it names what comes after this change, and no known bug there says that a program in `build` runs in `build`.

## Workflow follow-up

- Archive the change once the user has committed it: sync the delta specs into `openspec/specs/`, which creates
  `compiler/project-init` and `editor/project-setup`.
