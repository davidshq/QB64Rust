# Design

## Context

See proposal.md, "Why". The state of the code on 2026-11-27, branch `after-pr-29`:

- **The output folder.** `Build::new` in `crates/driver/src/settings.rs` joins `[build]` `output` to the settings
  file's folder, and takes `build` when the table does not set it. `default_paths` in `crates/driver/src/main.rs`
  gives the program's path and `run_in`: the source's folder without a `copy` list, the output folder with one.
- **The run folder.** libqb changes to the program's own folder before the first statement (`libqb.cpp`, "switch to
  directory of this EXE file"). So `run_in` is not where a program in `build` runs. An environment variable,
  `QB64RUST_RUN_IN`, was built on 2026-11-27 to send the program back, and taken out of the working files the same
  day, before any commit. The extension's integration test "built by qb64rust" fails until this change is built.
- **Subcommands.** `main` in `main.rs` takes `lsp`, `dap`, `toolchain`, `convert`, `serve` and `test` as the first
  argument, each with a usage text of its own.
- **New Project.** `vscode/src/newProject.ts` writes `main.bas`, a `qb64rust.toml` with `all = true` under `[fixes]`
  and `encoding = "utf-8"` under `[project]`, and `.vscode/settings.json`. It runs no compiler.
- **The settings file's keys** are matched by name in `settings.rs`. An unknown key in `[build]` or `[format]` is an
  error whose message lists the keys.

The user's decisions that bind this design: the default is beside the source, `init` has both front ends and a way
without questions (2026-11-27); a new project has the fixes on and an existing program has them off (2026-10-26);
a new project is UTF-8 (2026-11-24).

## Goals / Non-Goals

**Goals:**

- A person with one `.bas` file and no settings file never meets a build folder, and the program behaves the same
  however it is started.
- One rule for where a program runs: its own folder.
- One writer of a settings file for a program that exists: the compiler.
- A settings file that shows what else can be set.

**Non-Goals:**

- Changing "QB64: New Project". It stays as it is (D7).
- An offer in VS Code, made without being asked, to set up a folder that has no settings file. See "Open
  Questions".
- `qb64rust new <name>`. `qb64rust init <folder>` creates a missing folder, which covers it.
- Changing a settings file that exists. `init` refuses; editing stays by hand.
- Patterns in `copy`, and removing files from the output folder, as before.

## Decisions

### D1. The output folder is chosen per source and target

`Build` in `settings.rs` keeps `output` as written: `Option<PathBuf>`, joined to the settings file's folder when
set. A new method `Build::output_for(source_folder, platform)` gives the folder a build uses:

1. `output` when set;
2. else `build` in the settings file's folder (the source's when there is no file), when `copy` has an entry or
   the platform is wasm;
3. else the source's folder.

`default_paths` in `main.rs` calls it, and so do `output::prepare` and `output::copied_files` in
`crates/driver/src/output.rs`. `resolve_build`'s checks (no `copy` entry is or holds the output folder; no `copy`
with `output = "."`) run only with a `copy` list, where the folder is case 1 or 2, so they do not change.

*Why `copy` alone still means `build`:* a `copy` list into the source's own folder has no meaning, and Civil War
Strategy's settings file has `copy` and no `output`. Making that an error would break it for no gain.

*Why wasm keeps `build`:* a wasm build writes `p.js`, `p.wasm` and for a window program a `web` folder. A page reads
the files packed into it, not the source's folder, so the reason for building beside the source does not hold.
Spec `compiler/wasm-target` stays as it is. This is Claude's choice.

*Alternatives:* `[build]` present means `build` (a table with only `call_trace` would move the program); beside the
source for wasm too (three or more files and a folder land in the source folder).

### D2. A program runs in its own folder

`default_paths` returns the program's folder as `run_in`. The key stays in `--print-paths`, so the extension's
`programPaths` in `vscode/src/compiler/qb64rust.ts` does not change. A new test in
`crates/driver/tests/it/cli.rs`, `a_program_runs_in_its_own_folder`, builds a program without a settings file,
starts it from another folder, and finds the file it wrote beside its source.

*Alternative:* an environment variable that sends a program in an output folder back to its source's folder, for a
project with `output` and no `copy` list. It was built as `QB64RUST_RUN_IN` and taken out (see "Context"). Such a
project named its output folder on purpose, and `init` asks for its data files in the same step (D5). The variable
would keep two rules.

### D3. `init` is a module of the driver with the questions behind an interface

A new module `crates/driver/src/init.rs`, called from `main` in `main.rs` beside `serve`, with `INIT_USAGE`. Its
parts:

- `Plan::read(folder, output)`: what is in the folder (the `.bas` files, the data files offered, a settings file
  here or above, a `.git` entry here or above) and the default of each answer. `--plan` prints it with
  `serde_json`, which the driver already uses.
- `Answers`: one field per answer, filled from the options, then from the questions or the defaults.
- `ask(plan, answers, input, output)`: the questions, written to any `Write` and read from any `BufRead`. The
  tests drive it with strings. `main` calls it only when `std::io::IsTerminal` says that both the standard input
  and the standard output are a terminal, and `--yes` is not given.
- `render(plan, answers) -> String`: the text of the file (D4).
- `write(plan, answers)`: creates each file with `create_new`, so that a file made meanwhile is never overwritten,
  and removes what it wrote when a later file fails, as `writeProject` in `newProject.ts` does.

Each question is a numbered list with the default marked, answered by a number or by Enter. It needs no new crate.
That `IsTerminal` is in the standard library of the toolchain this repo pins is checked in the task that uses it
(CLAUDE.md working rule 17).

At most three questions are asked: the main file, where the program goes, and the data files. The keyword case and
the fixes are options and commented lines only (user, 2026-11-27: "do all three", on the pragmatic engineer's
review, which proposed this cut). The fixes' default already follows the user's ruling of 2026-10-26, and a beginner
cannot answer either question on the first day.

*Alternative:* a prompt crate with arrow keys. It is a dependency for five questions, and its terminal handling
could not be driven by the tests.

### D4. The file is a template, and tests hold it to the reader

`render` writes the tables in the order `[project]`, `[build]`, `[format]`, `[lints]`, `[fixes]`. A setting with an
answer other than its default is a plain line. Every other setting of `[project]`, `[build]` and `[format]` is a
line that starts with `# `, holds the default, and ends with a few words. `[lints]` and `[fixes]` get their `all`
line and a line that names `qb64rust --list-fixes` or the manual's list of warnings. A table with no plain line has
its header commented too, so that the file of every default sets nothing.

Three tests in `init.rs` keep the template true:

- the file of each combination of answers is read by `settings::parse_all` without an error;
- for each commented setting, the file with that line and its table header uncommented is read without an error;
- each key that `settings.rs` reads for `[project]`, `[build]` and `[format]` is in the template. For this,
  `settings.rs` gets one list of keys per table, which its "unknown setting" messages are built from too. Today
  those messages hold the names as text.

*Alternative:* write the file with a TOML library. It would lose the comments, which are the point.

### D5. The data files offered, and all of them by default

`Plan::read` lists the folder's own entries and leaves out what the spec names (requirement "The data files
offered"). The program a `.bas` file builds is `<name>.exe` on Windows and `<name>` elsewhere.

When the output folder is not the source's and no option says otherwise, every entry offered is the default. In a
terminal the question lists them and Enter takes all; the user may type the numbers to leave out.

*Why all:* a data file left out gives "File not found" when the program runs in `build`, which is the failure this
change is about. A file copied without need costs disk only.

*Alternative:* none by default. Then `--yes --output build` writes a project that cannot find its data.

### D6. The ignore line is printed, not written

`Plan::read` walks up from the folder for an entry named `.git` (a folder, or the file of a worktree). In a work
tree `init` prints the line to add: `/<output>/` for an output folder, or `/<program>` for each program built
beside its source. It changes no `.gitignore` and runs no `git` command.

*Why printed:* editing a file the person owns needs rules for the lines already there and for where the file is,
and a wrong line is worse than none. The line printed is exact, so it is never wrong (the pragmatic engineer's
review, approved by the user on 2026-11-27).

### D7. New Project stays; a test holds the two equal

`init` in a folder with no `.bas` file writes the three files that `newProject.ts` writes. The texts of `main.bas`
and `.vscode/settings.json` are the same bytes. The settings file has the same two settings, with the commented
lines of D4 added.

`.vscode/settings.json` is written by the command line too. Without it, VS Code would save the project's files in
code page 437 while the settings file says UTF-8. The pragmatic engineer's review asked whether the command line
should leave it out, so that a terminal user gets no `.vscode` folder. That needs the user to accept the code-page
risk, and the user has not answered, so it is written.

New Project does not call `init`, because it would then need a compiler and a trusted workspace (spec
`editor/build-run`, "Workspace trust": the compiler's path comes from workspace settings). It is the first command
a beginner runs. An integration test in `vscode/test/integration/suite` runs `qb64rust init --yes` in an empty
folder and compares: `main.bas` and `.vscode/settings.json` byte for byte with `MAIN_BAS` and `WORKSPACE_SETTINGS`,
and the settings file's plain lines with those of `SETTINGS_TOML`.

*Alternative:* New Project calls `init` and falls back to its own text without a compiler. That is still two
writers, and the fallback would be the less tested one.

### D8. The extension asks from the plan

- `vscode/src/compiler/qb64rust.ts` gets `initPlan(binary, folder)`, which runs `qb64rust init <folder> --plan`
  and reads the line, and `initWrite(binary, folder, answers)`, which runs `qb64rust init <folder> --yes` with one
  option per answer. An older `qb64rust` reads `init` as a file to build and prints no JSON; `initPlan` gives
  undefined, and the command says a newer `qb64rust` is needed.
- `vscode/src/setUpPlan.ts` is pure: from a plan, what to do first (open the settings file, run New Project, or the
  questions), and the data files question once the output folder is chosen. Unit tests cover it.
- `vscode/src/setUpProject.ts` registers `qb64rust.setUpProject` and shows each question with `showQuickPick`.
  The command offers only the source's folder and `build`. A plan without `--output` leaves `build` out of the data
  files, so one call of `initPlan` serves both answers. `package.json` adds the command, titled "QB64: Set Up
  Project".

## Risks / Trade-offs

- [A person who built from `main` since 2026-11-21 has a stale `build\game.exe`] → Nothing reads it: `exe` of
  `--print-paths` is now beside the source. The changelog says the folder can be deleted.
- [`game.exe` beside `game.bas` is built by either compiler, so Run cannot tell which built it] → As before
  2026-11-21. The output channel names the compiler of the last build.
- [On Linux the program of `maps.bas` is `maps`, which may be a data folder's name] → As before 2026-11-21 and as
  QB64pe. The build stops before the compile with a message that names the folder and says to use `-o` or
  `output`. The check is made on every system, so that it does not depend on the linker's message, and its test
  runs on Windows with a folder `maps.exe`.
- [Programs land in a git repository's source folder] → The manual and `init`'s closing line say which line to add
  to `.gitignore`.
- [A project with `output` and no `copy` list runs without its data] → `init` asks for the data files when it sets
  `output`. For a file written by hand, the manual's paragraph on `output` says so.
- [The template falls behind a new setting] → The third test of D4 fails until the setting is in the template.
- [Tests, tools and documents here assume `build\`] → A task searches this repo for them, as task 2.4 of
  `build-output-folder` did the other way.

## Migration Plan

A project with no settings file, or with one that has neither `output` nor `copy`, builds beside its source from
the first build. A project that wants `build` back sets `output = "build"`, by hand or with `qb64rust init` in a
folder that has no settings file yet. Civil War Strategy and every project with a `copy` list are unchanged. No
release holds the old default, so no released version changes behavior.

## Open Questions

- **An offer to set up a folder.** VS Code could say once, when a folder with `.bas` files has no settings file,
  that "QB64: Set Up Project" exists. The panel's teacher suggested it on 2026-11-27. The user has not asked for
  it, and it can be added later without changing this design.
- **A `--target wasm` answer in `init`.** A project meant for the browser may want questions of its own (the page's
  files). Left until the wasm target is no longer experimental.
