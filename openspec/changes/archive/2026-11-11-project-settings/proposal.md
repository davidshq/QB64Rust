# Proposal

## Why

The formatter's settings live only in VS Code, and the lints cannot be turned off at all. A project cannot say
"keywords in capitals" or "no `unused-label` warnings" once, in a file committed with the program, so that every
editor and every command-line run agrees. `qb64rust.toml` already holds the project's fixes; the user asked for
format and lints as options in the same file (2026-11-11, and the `project-settings` row of `SOMEDAY.md`,
2026-11-09).

## What Changes

- **A `[format]` table in `qb64rust.toml`** with the five layout settings: `auto-indent`, `indent-size`,
  `indent-subs`, `auto-layout` and `keyword-case` (`"camel"`, `"upper"` or `"lower"`). A key that is absent
  leaves that setting to the layer below.
- **A `[lints]` table in `qb64rust.toml`**: `all = true|false` and `<code> = true|false` for each of the nine
  lint codes, read the way `[fixes]` is. Every lint stays on by default, as today.
- **Order of the layers.** For `qb64rust -y`: QB64pe's defaults, then `[format]`, then the `-f:` settings of the
  command line. In the editor: the defaults, then VS Code's `qb64rust.format.*` settings, then `[format]`. The
  project's file wins over a user's VS Code settings; the command line wins over the file.
- **New command-line options.**
  - `--lints on|off` and `--lint <code>=on|off`, as `--fixes` and `--fix` work.
  - `--format-default <setting>=<value>`: a layout setting that ranks below `[format]`. The extension uses it
    when it runs `qb64rust -y` without the language server, so that the project's file still wins.
  - `--no-settings`: no `qb64rust.toml` is looked for. The test runner passes it, so that a settings file in or
    above the tree changes no test result.
- **The settings file is always read and checked**, unless `--no-settings` is given. Today `--fixes on|off`
  skips reading it. **BREAKING** for one case: `qb64rust --fixes off p.bas` with a `qb64rust.toml` that has an
  error now fails; before, the file was not read.
- **A settings file with an error stops the language server's formatting.** `-y` already stops on one (exit
  status 1, no output file). The language server formats anyway, which was safe while the file could not change
  the layout. Now a misspelled `keyword-case` would silently lay the file out in another style, so the server
  answers with no edits and a message, as `-y` does.
- The language server reads `[format]` and `[lints]` from the settings file of the document's program, as it
  reads `[fixes]`. A change of the file on disk already makes it check again.

Out of scope, each staying a `SOMEDAY.md` row:
- New formatter behavior (one statement per line, single-line `IF` to a block). This change only moves the
  existing settings into the file.
- Lints for `$INCLUDE` files, and a comment that silences one line's lint.
- `[language]` (features beyond QB64pe) and layout as you type.
- `qb64pe -y`: it does not read `qb64rust.toml`. When the extension formats with the old compiler, only the VS
  Code settings apply.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/format`: "Default settings" (the settings file is read) and "Program with an error" (a bad settings
  file stops `-y`); new requirements for `[format]` and for `--format-default`.
- `compiler/lints`: new requirements for `[lints]` and for `--lints` and `--lint`.
- `compiler/fixes`: "The settings file" names the other tables; "Command-line fixes" says that the file is read
  and checked even with `--fixes`.
- `compiler/cli`: "Fixes options" becomes the settings options: adds `--lints`, `--lint`, `--format-default` and
  `--no-settings`.
- `editor/language-server`: "Format settings from the client" (the file's `[format]` wins) and "Format failure"
  (a bad settings file); a new requirement for lints in a check.
- `editor/formatting`: "Layout settings" (the project's file wins; how `qb64rust -y` is called).
- `compiler/unit-tests`: "Build options of a test run" adds `--no-settings` to the options `qb64rust test` takes.
- `testing/verification-suite`: "Suites build with the fixes off" (the runner passes `--no-settings`).

## Impact

- `crates\driver`: a new `settings.rs` that reads the whole file once; a new `lints.rs` (the set of lints that
  are on, and its options); `fixes.rs` keeps the fix options; `main.rs` (options, `-y`, `-w`), `lib.rs` (the
  language server's hooks), `compile.rs`, `report.rs`, `unit_tests\run.rs`.
- `crates\base`: `layout.rs` (settings given by layer), `lib.rs` (`Diagnostics::drop_lints`).
- `crates\lsp`: no change expected; the driver's hooks read the file.
- `vscode\`: `format.ts` (the `qb64rust -y` call), `package.json` (the settings' descriptions), `README.md`,
  `CHANGELOG.md`.
- `tools\legacy_tests\run_legacy_tests.py` and `tools\sandbox\run.py`: `--no-settings` beside `--fixes off`.
- Documents: `docs\MANUAL.md` (the settings file), `DECISIONS.md`, `SOMEDAY.md` (two rows narrowed).
- No change to a built program: neither table changes the C++ written or the build cache's key.
