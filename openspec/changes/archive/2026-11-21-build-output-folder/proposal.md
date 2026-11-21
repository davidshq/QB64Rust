# Proposal

## Why

`qb64rust -x game.bas` writes `game.exe` beside `game.bas`, and the VS Code extension runs it from there. A program
that writes files beside itself then writes them into the source folder. Civil War Strategy writes its settings
`CWS.CFG`, its Hall of Fame `HISCORE.CWS` and its saved games there. So each playtest changes files that are
tracked in the game's repo. On 2026-11-20 a two-player game rewrote both files.

Other toolchains keep the program out of the source folder. They differ in where they run it:

- Visual Studio's C++ projects build into a folder such as `x64\Debug`, and the debugger's working folder is the
  project's folder by default (Microsoft Learn, "C++ Debugging Property Pages").
- `cargo run` builds into `target\debug` and runs the program in the folder Cargo was started from (The Cargo
  Book, "cargo run").
- .NET projects run the program in `bin\…`. Each data file marked "Copy if newer" (`PreserveNewest`) is copied
  there (Microsoft Learn, "Application Resource, Content, and Data Files").

On 2026-11-21 the user chose this for QB64Rust: build into `build` by default, run in the source folder as
Visual Studio C++ and Cargo do, and run in `build` when the project lists data files to copy, as .NET does. That
last case is what keeps Civil War Strategy's playtests out of its repo.

## What Changes

- **BREAKING: `qb64rust -x game.bas` with no `-o` writes `build/game.exe`**, not `game.exe` beside `game.bas`.
  `build` is in the settings file's folder when the program has one, and in the source's folder otherwise. `-o`
  still names the path. The output of a wasm build moves the same way.
- **A `[build]` table in `qb64rust.toml`.** `output` names another folder than `build`. `output = "."` builds
  into the settings file's folder, which is next to the source for a settings file beside it.
- **`copy` in `[build]`** lists the data files and folders a program needs beside it, such as
  `copy = ["CWS.CFG", "HISCORE.CWS", "graphics", "scenarios"]`. Before each build without `-o`, each is copied
  into the output folder when it is missing there, or when the source's copy is newer. So data edited in the
  source folder reaches the build, and a file the program rewrote in the build folder is kept. Nothing in the
  output folder is deleted.
- **The folder a program runs in.** With no `copy` list it is the source's folder, as today, so a program finds
  its data files where they are. With a `copy` list it is the output folder, so what the program writes stays
  there.
- **`qb64rust --print-paths <file.bas>`** prints, as JSON, the program's path and the folder to run it in, after
  reading the settings file. The extension uses it.
- **The extension** builds without `-o`, and Run, Build and Run, the run task and Debug start the program in
  that folder. A build with QB64pe in the extension does not read the settings file and is unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/cli`: the default output path is in `build`; the `[build]` table with `output` and `copy`; the new
  option `--print-paths`; the default output of a wasm build.
- `editor/build-run`: Build writes where the compiler's default puts the program; Run and the run task start it
  in the folder `--print-paths` names.
- `editor/debug-launch`: a debug build goes where a build goes, and the default `cwd` is the folder
  `--print-paths` names.

## Impact

- `crates/driver`: `settings.rs` reads `[build]`; `main.rs` chooses the default output and takes
  `--print-paths`; a new module copies the files. CLI tests that build without `-o` expect `build`.
- `vscode`: `src/compiler/qb64rust.ts`, `src/build.ts`, `src/tasks.ts`, `src/debug.ts` and
  `src/debugConfig.ts`.
- Anything that runs `qb64rust -x` without `-o` and expects the program beside the source: scripts in this repo
  and the game repos, and their READMEs. Most pass `-o`; the sweep is a task.
- Documents: `docs/MANUAL.md` ("The settings file", "Compiling and running a program"), `docs/QUICKSTART.md`
  (lines 74 and 154), `vscode/README.md` (lines 34 and 148). The language reference does not describe building,
  so no page of `docs/reference` changes.
- Civil War Strategy, in its own repo, then sets `[build]` with its `copy` list and drops `tools/playtest.py`.
