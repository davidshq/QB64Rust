# Design

## Context

See proposal.md, "Why". Today the path of the program is decided in three places, and all three put it next to the
source:

- **The compiler.** `run` in `crates/driver/src/main.rs` (about line 951) takes `-o`, else
  `input.with_extension(platform.exe_extension())`. It works out this path before it reads the settings file:
  `program_settings` runs later, and on a bad settings file `compile::remove_old` removes the old program at that
  path.
- **The extension.** `executableFor` in `vscode/src/compiler/qb64pe.ts` gives `<dir>/<name>.exe`. The qb64rust build
  (`buildArgs` in `vscode/src/compiler/qb64rust.ts`) passes it as `-o`, and so do the QB64pe build, the Run
  command (`runCommand` in `vscode/src/build.ts`) and the run task (`makeTask` in `vscode/src/tasks.ts`). Run starts
  the program in the program's folder (`startInTerminal`); the run task in the source's folder. Both are the
  same folder today.
- **The debugger's default `cwd`.** `resolveLaunch` in `vscode/src/debugConfig.ts` sets it to the folder of
  `program` before the build, and `program` is still the `.bas` file then.

QB64pe's own command line does not put the program beside the source either. Unless its setting
`SaveExeWithSource` is on, it writes into `DefaultExeSaveFolder$`, which is QB64pe's own folder by default
(`qb64pe.bas` line 1130, `ide/config/cfg_methods.bas` line 138). So no default here matches QB64pe's, and this
change does not move away from it.

The settings file is read by `crates/driver/src/settings.rs`. `Project` (fixes, format, lints) is `Copy` and is
used in many places. `[project]` `exclude` is read beside it by `parse_all`, which returns the patterns separately.

## Goals / Non-Goals

**Goals:**

- No build writes into the source folder unless asked to (`-o`, or `output = "."`).
- A program that reads data files beside its source still finds them, with no setting.
- A program that writes files beside itself can be kept from writing into the source folder by one list.
- One place decides where the program goes and where it runs: the compiler. The extension asks it.

**Non-Goals:**

- Mirroring: removing from the output folder what was removed from the source folder.
- Patterns (`*`, `**`) in `copy`. Each entry is a file or a folder by name. Patterns can come later without
  changing the meaning of a plain name.
- The output folder for QB64pe builds in the extension. QB64pe does not read `qb64rust.toml`.
- Making the files of `copy` visible to a wasm program. They are copied beside the `.js` and `.wasm` as for any
  target. A wasm program reads files only through what the wasm target loads for it (spec `compiler/wasm-target`).
- A program started outside the extension, such as by a double click on `build\game.exe`. It starts in `build`,
  where its data files are only if they are in `copy`. Visual Studio's C++ projects have the same limit.
- `qb64rust test`. Its builds go to folders of its own, as now.

## Decisions

### D1. `[build]` is read beside `Project`, not into it

`settings.rs` gets a struct `Build { output: PathBuf, copy: Vec<CopyEntry>, root: PathBuf }`. `root` is the
settings file's folder, or the source's folder when there is no file. `output` is joined to `root`, `build` when
the table does not set it. A `CopyEntry` holds the joined path and its line in the file, so that a missing entry is
reported at its line. `parse_all` reads the table; `load` returns `Build` beside `Project`, so `Project` stays
`Copy`.

The checks of spec `compiler/cli`, "A bad build table", are made in `parse_all`, with the other tables' checks.
They are checks of the file alone, so `-y`, `qb64rust check` and the language server report them as they report
a bad `[format]`.

*Alternative:* add the fields to `Project`. That drops `Copy`, and every caller of `Project` would clone it for
fields that only `-x` and `--print-paths` use.

### D2. The default is `build`, in the settings file's folder or the source's

The folder is relative to the settings file's folder because that is the project's folder. A program whose main
file is in `src` gets one `build` at the project's top, as Cargo's `target` is at the workspace's top. With no
settings file the source's folder is the project's.

`output = "."` builds into the settings file's folder: next to the source when the file is beside it, which is the
old behavior. It is the way back for a project that wants it. An empty `output` is an error, not a second spelling
of `"."`.

In `run`, the path is chosen after the settings are loaded: `-o`, else `Build::output` joined with the default
name. A function `default_paths(input, platform, build)` in `main.rs` gives both the program's path and the
folder it runs in (D4); `--print-paths` and `-x` use the same function, so they cannot disagree.

On a bad settings file the build stops, as now. `remove_old` then removes nothing: the output folder is unknown
when the file cannot be read, and a program next to the source is no longer this compiler's.

*Alternatives:* `build` always in the source's folder (a project with sources in several folders would get a
`build` in each main file's folder); `target` or `bin` as the name (`build` is the name the user asked for,
2026-11-21).

### D3. Files are copied before the compile, newest wins

A new module `crates/driver/src/output.rs` copies the `copy` entries into the output folder. `run` calls it after
the settings are read and before `compile::compile_exe`, only when there is no `-o`. A file is copied when the
output folder has none of that name, or when the source's modification time is later than the copy's. A folder is
walked, and each file in it is treated the same way. Folders are created as needed. Nothing is deleted. A failure
(a missing entry, a file that cannot be read or written) stops the command with status 1 before anything is
compiled. The message names the settings file and the entry's line for a missing entry, and the file for an I/O
error.

Copying first means a program that fails to compile still gets its data copied. That costs nothing, and it keeps
"no program is left at the output path on failure" (requirement "qb64pe-compatible compile") simple: nothing is
written after the program.

*Why the time rule:* it is .NET's "Copy if newer" (`PreserveNewest`), and it gives both things wanted here. A file
the program rewrote is newer than the source's, so it is kept. A data file edited in the source folder is newer
than the copy, so it goes over it. Whether `std::fs::copy` keeps the source's modification time on Windows and on
Linux is checked in task 2.2 (CLAUDE.md working rule 17). The rule works either way, since a copy is never older
than its source; the tests set the times themselves.

*Alternatives:* copy every time (.NET's `Always`; it loses what the program wrote, which is the bug this change
fixes); copy only when missing (an edited data file would never reach the build); two lists, one to refresh and
one to seed once (more to explain for the same result in the cases above).

### D4. The program runs in the source's folder, or in the output folder with a `copy` list

With no `copy` entry the program runs in its source's folder, as Visual Studio's C++ debugger and `cargo run` do.
Every program that reads files beside its source works as today, and nothing it writes changes. With a `copy`
entry it runs in the output folder, as a .NET program runs in `bin`. The project then says which files the program
needs, and what the program writes stays in `build`.

The rule is the presence of `copy`, not a third setting. A project that lists its data has said that the program
belongs in the output folder; a project that has not would break there.

*Alternatives:* always the output folder (every program that reads a data file breaks until it gets a `copy`
list); always the source folder (Civil War Strategy still writes into its repo, which is what this change is
for); a setting `run-in = "source" | "output"` (a third key whose only useful values follow from `copy`).

### D5. `--print-paths` is the extension's way to the paths

`qb64rust --print-paths <file.bas>` prints `{"exe": …, "run_in": …}` from `default_paths`, as one line of JSON, and
exits. The extension calls it in a new function `programPaths(file)` in `vscode/src/compiler/qb64rust.ts`. It is
used by Build, Run, Build and Run, the run task and the debugger whenever qb64rust builds. When QB64pe builds, or
when the call fails (an older `qb64rust` named by `qb64rust.path`, which has no `--print-paths`), the paths are
`executableFor(file)` and the source's folder, and the qb64rust build passes that path as `-o`, as now. So an
older compiler builds and runs as before.

With a compiler that has `--print-paths`, the qb64rust build passes no `-o`, so the compiler puts the program in
its default place and copies the data.

*Alternatives:* the extension reads `qb64rust.toml` itself (a second reader of the file, which could disagree
with the compiler's); two options, one per path (two processes for each Run).

### D6. The extension starts the program in `run_in`

`startInTerminal` in `build.ts` and the run task in `tasks.ts` take the folder from `programPaths`. In
`debugConfig.ts`, `resolveLaunch` leaves `cwd` unset when the configuration has none and `program` is a `.bas`
file; the build step in `debug.ts` then sets it to `run_in`. For an executable as `program`, the default stays its
folder.

### D7. What changes in Civil War Strategy

In the game's repo, not this one: `qb64rust.toml` gets `[build]` and
`copy = ["CWS.CFG", "HISCORE.CWS", "graphics", "scenarios"]`; `.gitignore` gets `build/`; `tools/playtest.py`,
`tests/test_playtest.py` and their README paragraph are removed. The game's `tests/play.py` builds with `-o`, so
its builds and recordings do not change.

## Risks / Trade-offs

- [Scripts and documents that build without `-o` and then look beside the source break] → Task 2.4 finds them in
  this repo and the game repos by search and fixes each. The manual's and QUICKSTART's examples change with the
  code. The first release with this change says it in its notes.
- [Users who start the program outside VS Code, or by a double click, start it in `build` without its data] →
  The manual says where a program runs and how `copy` or `output = "."` changes it.
- [A file deleted from the source folder stays in the output folder, so the program can read a stale file] →
  Written in the manual. Deleting the output folder gives a clean build.
- [`--print-paths` runs the compiler once more for each Run] → It reads one small file and prints a line, with no
  build. It is measured in task 3.4, and the result is written there.
- [Times on a FAT disk are kept to 2 seconds] → An edit within 2 seconds of the copy could be missed. Builds are
  not that fast after an edit; the manual says it.

## Migration Plan

Existing projects get `build` on their next build, and run as before (in the source folder) until they add a
`copy` list. A project that wants the old place sets `output = "."`. Today's compiler ignores a table it does not
know (`settings.rs`, "Any other table is ignored"), so a project with `[build]` built by an older compiler builds
next to the source, without a message. The manual says which version first reads `[build]`.
