# Proposal

## Why

Since 2026-11-21 a build without `-o` writes the program into a folder `build`, and the program is meant to run in
its source's folder. That is hard on a beginner who has only `game.bas` and `scores.txt`:

- The program is in a folder they did not make.
- Run in VS Code finds `scores.txt`, but a double click on `build\game.exe` does not. libqb makes the program's own
  folder the current one, and the data is not in `build`.
- A file the program writes lands beside the source from the editor and in `build` from a double click.
- The only way out is to learn `qb64rust.toml`, `[build]` and `copy`.

On 2026-11-27 an environment variable, `QB64RUST_RUN_IN`, was built to make Run work. It did not help a program
started any other way, and it was taken out the same day.

The user decided on 2026-11-27: "the default should be in the same folder", with a command that sets a project up
by asking a few questions, in a terminal and in VS Code, and that can also work without questions as `cargo init`
does. A person who wants `build` then chooses it, and does not have to remember the settings' names.

## What Changes

- **BREAKING** (for builds from `main` only) A build for the host without `-o` writes the program beside its source, unless the settings file
  names an output folder (`[build]` `output`) or has a `copy` list. With a `copy` list and no `output`, the output
  folder is still `build`, so Civil War Strategy's settings file works unchanged. This reverses the default of
  2026-11-21.
- A program always runs in the folder it is in. `run_in` of `--print-paths` is the folder of `exe`.
- `QB64RUST_RUN_IN` is not part of this: it was taken out of the working files on 2026-11-27, before any commit.
- A build for the wasm target keeps `build` as its default. It writes several files, and a page does not read the
  source's folder. This is Claude's choice, for the user to see.
- New: `qb64rust init [folder]` writes a `qb64rust.toml`.
  - In a terminal it asks up to three questions, each with a default: the main file, where the program goes, and
    the data files to copy. The keyword case and the fixes are options only.
  - With `--yes`, or when not run in a terminal, it asks nothing. A flag for each answer sets it.
  - In a folder with no `.bas` file it starts a new project with the files and settings that "QB64: New Project"
    writes today: `main.bas`, fixes on, UTF-8. In a folder with a program the fixes stay off and no encoding is set
    (the user's decisions of 2026-10-26 and 2026-11-24).
  - It never overwrites a `qb64rust.toml`.
  - The settings it did not set are in the file as commented lines.
  - In a git repository it prints the line to add to `.gitignore`, and changes no file there.
- A build whose program path is a folder (on Linux, `maps.bas` beside a folder `maps`) stops with a message that
  names the folder.
- New: the VS Code command "QB64: Set Up Project" asks the same questions as pick lists and has the compiler write
  the file.
- "QB64: New Project" does not change. It needs no compiler and works in an untrusted workspace, and it would lose
  both if it called `qb64rust init`. A test holds its settings equal to those of `init` in an empty folder.

## Capabilities

### New Capabilities
- `compiler/project-init`: the `qb64rust init` subcommand: what it asks, its flags, the file it writes, and what it
  refuses.
- `editor/project-setup`: the extension's "QB64: Set Up Project" command.

### Modified Capabilities
- `compiler/cli`: "The output folder" (the default), "The folder a program runs in" (always the program's own),
  "The paths of a program" (the printed paths).
- `editor/build-run`: "Run commands" (the program beside the source).
- `editor/debug-launch`: "Launch defaults" (the program beside the source).

## Impact

- **Compiler:** `crates/driver` (`settings.rs`, `main.rs`, `output.rs`, a new `init.rs`).
- **Extension:** `vscode/src` (`compiler/qb64rust.ts`, new `setUpPlan.ts` and `setUpProject.ts`), `package.json`,
  its tests.
- **Documents:** `MANUAL.md`, `QUICKSTART.md`, `vscode/README.md`, `vscode/CHANGELOG.md`,
  `DIVERGENCES-QB64PE.md` D-073, the reference page of `_CWD$`, `DECISIONS.md`, `LAYOUT.md`.
- **Users:** a project with no `[build]` table that built into `build` now builds beside its source. An old
  `build\game.exe` stays where it is; nothing deletes it. No release tag holds the 2026-11-21 commit (`881ff759`),
  so only people who build from `main` have met the `build` default.
- **Game repos:** none of the four needs an edit. Civil War Strategy has `copy` and keeps `build`; its `play.py`
  builds with `-o`.
