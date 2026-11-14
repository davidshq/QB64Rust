# Design

## Context

Motivation: `proposal.md`, "Why". What exists today:

- `crates\driver\src\fixes.rs` finds the nearest `qb64rust.toml` above the main file (`find_settings`), parses it
  with `toml::de::DeTable` and reads two things from it: the `[fixes]` table (`parse_settings`) and the
  `[project]` table's `main` (`project_main`, for the language server). Any other table is ignored.
- `fixes::with_cli` does not read the file at all when `--fixes on|off` is given. The test runner relies on
  that: `fixes_args` in `tools\legacy_tests\run_legacy_tests.py` adds `--fixes off` to every command that compiles
  or lays out a program.
- The layout settings are `qb64rust_base::layout::Settings`. `Settings::set(name, value)` reads one `-f:` setting.
  The two keyword settings are kept as a pair (`capitals`, `lowercase`), and the style is worked out from both.
- The language server gets its layout settings from the client. The driver's hook `format_for_server`
  (`crates\driver\src\lib.rs`) receives them with the program's main path, and already reads the settings file
  there for the fixes (`check_with_settings`).
- A lint warning is a diagnostic with a code. Five codes are given by `crates\sema\src\lints.rs`; some of QB64pe's
  four are given during the check (`crates\sema\src\check\decl.rs`). The nine codes are `lints::CODES`.
- With the language server off, `vscode\src\format.ts` runs `qb64rust -y` with the VS Code settings as `-f:`
  settings.

## Goals / Non-Goals

**Goals:**
- One read of the settings file gives the fixes, the layout overrides and the lint set, with one kind of error.
- The same file gives the same layout from the command line, the language server and the extension's
  `qb64rust -y` call.
- A test run cannot be changed by a settings file in or above the tree.

**Non-Goals:**
- No new layout behavior and no new lint.
- No change to `crates\lsp`: the driver's hooks carry the new behavior.
- `qb64pe -y` is not taught about the file.

## Decisions

### D1. One reader for the whole file: `crates\driver\src\settings.rs`

A new module `settings.rs` owns the file: `SETTINGS_FILE`, `SettingsError`, `find_settings`, the TOML parse, and
a `Project` value with three fields: the fixes (`Fixes`), the layout overrides (D2) and the lint set (D3).
`settings::load(main, no_settings)` returns the `Project` and the path of the file read. With `no_settings`, or
when no file is found, it returns every fix off, no layout override and every lint on.

`settings.rs` reads all three tables. `[fixes]` and `[lints]` share one reader of a table of switches
(`switches`). `fixes.rs` keeps the command line only: `CliFix`, the two option parsers and `apply_cli`. `main_of`
and `project_main` move to `settings.rs` with the other file code.
`with_cli` and `for_program` go away: their callers (`main.rs`, `lib.rs`, `report.rs`, `unit_tests\run.rs`) call
`settings::load` and then `fixes::apply_cli`.

*Why not keep adding to `fixes.rs`:* the file would be read once per table, and a module named for fixes would
own the format and lint tables.

### D2. Layout settings by layer: `crates\base\src\layout.rs`

A new `layout::Overrides` holds the five settings as `Option`s: `auto_indent`, `indent_size`, `indent_subs`,
`auto_layout`, `keywords` (a `KeywordStyle`). `Settings::apply(&Overrides)` sets each one that is `Some`.
`settings.rs` builds the `Overrides` from the `[format]` table: the keys are kebab-case, as fix names are, and the
values are TOML booleans, an integer and a string. The `-f:` forms (`on`, `yes`, `-1`, a size read as `VAL`
reads it) are QB64pe's command line and are not accepted in the file.

`-y` builds its settings in this order (`layout_settings` in `main.rs`):

1. `Settings::default()`;
2. each `--format-default`, through `Settings::set`;
3. the file's `Overrides`, through `Settings::apply`;
4. each `-f:` layout setting, through `Settings::set`.

`Settings::apply` also clears the keyword pair and the "style before" it. So the `-f:` keyword settings after it
start from the style the file left. `-f:KeywordCapitals=0 -f:KeywordLowercase=0` gives CamelCase over a file that
says `"upper"`, as the spec's "Order of layout settings" requires. `-f:KeywordCapitals=0` alone keeps the file's
style, as `qb64pe` keeps its `config.ini` style for it. A command line with no keyword setting keeps it too.

For the language server, `format_for_server` applies the file's `Overrides` to the client's `Settings` before it
calls `format_file`. The file is the one found from `main_path`, the main file of the document's program, so an
include file is laid out with its program's settings.

*Alternative considered:* have the extension read the TOML and merge it. Rejected: a second TOML reader in
TypeScript, and the two could disagree.

### D3. The lint set: `crates\driver\src\lints.rs`, filtered after the front end

A new `qb64rust_driver::lints::Lints` is a set over the nine codes of `qb64rust_sema::lints::CODES`, built like
`Fixes` (`ALL`, `OFF`, `with(code, on)`). `Lints::drop_from` drops the warnings of the lints that are off, through
a new `Diagnostics::drop_lints` in `crates\base\src\lib.rs`. A lint that is off still runs; the cost is small.

The filter runs in three places, after the front end: `compile::compile_exe` (a compile, `-z`, `--dump`),
`format` in `main.rs` (`-y`), and `check_with_settings` in `lib.rs`, which all three of the language server's hooks
call. `report.rs` does not filter, so `--report warnings` keeps counting every lint.

`[lints]` is read like `[fixes]`: `all` first, then each code. `--lints` and `--lint` mirror `--fixes` and
`--fix` (`CliLint` and `apply_cli` in `lints.rs`).

*Alternative considered:* pass the set into `qb64rust_sema::check_with` and filter at its end. Rejected when
building: every lint warning is pushed in one function (`sema::lints::run`), so `sema` gains nothing, and the set
would have to go through `frontend_at`, `frontend_explicit` and `check_program`, about 14 call sites.

### D4. The file is always read; `--no-settings` is the only way around it

`--fixes on|off` no longer skips the file: the file may hold `[format]` and `[lints]`, which `--fixes` does not
replace. `--no-settings` makes `settings::load` return the defaults without looking for a file. It is accepted
wherever the file is read today: a compile, `-y`, `-z`, `--dump` and `qb64rust test`.

The test runner's `fixes_args` adds `--no-settings` beside `--fixes off` and its first line says so. Tier 1's
`format_oracle.rs` calls `format_program` with its own `Settings` and never reads a settings file, so it needs no
change.

### D5. A bad settings file stops formatting

`-y` (`main.rs`): when `settings::load` fails, print the error and exit with status 1 before the front end runs.
A compile and `-y` already did this for a bad `[fixes]` table (`fixes_errors` in `cli.rs`), so only the language
server's formatting changes.

`format_for_server` (`lib.rs`): `check_with_settings` already turns a bad file into an error diagnostic on the
main file's first line, with the text `qb64rust.toml:<line>: <message>`. `format_for_server` returns `Err` with
the diagnostics in that case, as it does for a parser error. The server then answers with no edits and shows the
first error: "The program was not formatted: p.bas, line 1: qb64rust.toml:2: …". Its text names the settings file
and its line, so `crates\lsp` needs no change (`format_server::bad_settings_file`).

*Why:* before this change the file could not change the layout, so the server could safely format anyway. Now a misspelled
`keyword-case` would rewrite every keyword of the file in another style.

### D6. `--format-default`, for the extension's `qb64rust -y` call

`vscode\src\format.ts` passes the VS Code settings to `qb64rust -y` as `--format-default <name>=<value>` pairs
instead of `-f:` settings, so they rank below the file (D2, step 2). The call to `qb64pe -y` keeps `-f:`.
`--format-default` outside `-y` is a usage error, since it could only be a mistake.

*Alternative considered:* let `-f:` rank below the file. Rejected: the command line must be able to override the
file, for CI and for the test runner's format variants.

## Risks / Trade-offs

- [A script calls `qb64rust --fixes off` to ignore a broken settings file] → It now fails with the file's error.
  The message names the file and line, and `--no-settings` restores the old effect. The only known caller is the
  test runner, which this change updates.
- [A typo in a table's name, such as `[formt]`, is ignored without a message] → Unchanged from today, where any
  unknown table is ignored. See "Open Questions".
- [The extension and its bundled `qb64rust` disagree about `--format-default`] → They ship together in one
  package; `buildWith` `qb64pe` does not use the option.
- [`qb64rust.path` names a `qb64rust` older than this change] → With the language server off, Format Document
  fails with "unknown option `--format-default`". Only a development build or an old download is affected; the
  changelog says so. A version check is not worth its code for a development setting.
- [Two programs in one folder want different settings] → Not supported: the file is per folder, as for fixes.

## Open Questions

- Should an unknown top-level table be an error instead of being ignored? An error catches `[formt]`. Ignoring
  lets an older `qb64rust` open a project whose file has a newer table. This change keeps ignoring; the answer
  does not change its tasks.
- Is a `--list-lints` wanted, as `--list-fixes` exists? An unknown code's error already lists the nine codes.
