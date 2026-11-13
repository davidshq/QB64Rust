# Tasks

Standing rules: CLAUDE.md.

## 1. One reader for the settings file

- [x] 1.1 Add `crates\driver\src\settings.rs` (design D1): move `SETTINGS_FILE`, `SettingsError`,
      `find_settings`, `main_of`, `project_main` and the TOML parse out of `fixes.rs`; add `Project` and
      `settings::load(main, no_settings)`. `fixes.rs` keeps the `[fixes]` table reader, `CliFix`, the option
      parsers and `apply_cli`. Replace `with_cli` and `for_program` at their four callers (`main.rs`, `lib.rs`,
      `report.rs`, `unit_tests\run.rs`). Verify: the moved tests of `fixes.rs` pass unchanged in their new
      module (`cargo test -p qb64rust-driver`).
- [x] 1.2 Add `--no-settings` to a compile, `-y`, `-z`, `--dump` and `qb64rust test`, and to the usage text
      (design D4). The file is now read and checked when `--fixes` is given. Verify with CLI tests in
      `crates\driver\tests\it\cli.rs`: a bad file with `--fixes off` fails with exit status 1; the same file with
      `--no-settings` builds; a file that is not valid TOML with `--no-settings` builds.
- [x] 1.3 Add `--no-settings` to `fixes_args` in `tools\legacy_tests\run_legacy_tests.py` and to the runner's
      first line; update `tools\legacy_tests\README.md` where it names `--fixes off`. Verify: the runner's own
      tests pass, and one corpus program and one `format-corpus` program named one by one pass on the host with a
      scratch `qb64rust.toml` (`[format]`, `keyword-case = "upper"`) placed above them and removed after.

## 2. The `[format]` table

- [x] 2.1 Add `layout::Overrides`, `Settings::apply` and the method that starts the `-f:` layer to
      `crates\base\src\layout.rs` (design D2). Verify with unit tests there: each of the five settings applied
      alone; `-f:KeywordCapitals=0 -f:KeywordLowercase=0` over `Upper` gives CamelCase; no keyword setting keeps
      `Upper` (`cargo test -p qb64rust-base`).
- [x] 2.2 Read `[format]` in `settings.rs` into `Overrides`, with the errors of the spec's "A bad format table"
      (unknown key with the five keys named, wrong type, `indent-size` out of range, unknown `keyword-case`).
      Verify with unit tests in `settings.rs`: each key, a dotted key, no table, and each error with its line.
- [x] 2.3 Add `--format-default` and build the `-y` settings in the four steps of design D2; `--format-default`
      outside `-y` is a usage error; a bad settings file stops `-y` (design D5). Verify with CLI tests: the eight
      scenarios of the `compiler/format` delta ("Capitals for the project", "Only one key", "No table", the three
      of "Order of layout settings", the two of "Format default option") and "A bad settings file" (exit status
      1, no output file).
- [x] 2.4 Apply the file's `Overrides` in `format_for_server`, and return `Err` for a bad settings file (design
      D5). Verify with tests of the driver's hook in a temp folder: the client's `camel` and indent 2 with a file
      saying `"upper"` gives capitals and indent 2; an include file takes the file of its program's folder; a bad
      file gives `Err` whose first error names `qb64rust.toml` and its line.

## 3. The `[lints]` table

- [x] 3.1 Add `crates\driver\src\lints.rs` (`Lints` over the nine codes of `sema::lints::CODES`) and
      `Diagnostics::drop_lints` in `crates\base\src\lib.rs` (design D3, as revised while building). Verify with
      unit tests: every code is a bit, one code off, `all` off with one on (`lints.rs`), and a warning without a
      code and the errors kept (`drop_lints_keeps_the_rest` in `base`).
- [x] 3.2 Read `[lints]` in `settings.rs`; add `crates\driver\src\lints.rs` with `--lints` and `--lint` and
      their `apply_cli`; add both to a compile, `-y`, `-z`, `--dump` and the usage text; `report.rs` passes
      `Lints::ALL`. Verify with unit tests (the table's contents and errors, the command line over the file) and
      CLI tests: the scenarios of the `compiler/lints` delta, and "A lint off in a format run" of the
      `compiler/cli` delta.
- [x] 3.3 Pass the file's lint set through `check_with_settings` for the language server's three hooks. Verify
      with a test of `check_for_server` in a temp folder: an unused label with `unused-label = false` gives no
      warning; the same program with no file gives one.

## 4. The extension

- [x] 4.1 In `vscode\src\format.ts`, pass the layout settings to `qb64rust -y` as `--format-default` pairs; keep
      `-f:` for `qb64pe -y` (design D6). Update the file's header comment. Verify with the extension's unit test
      of the argument list, for both compilers.
- [x] 4.2 (Passed in CI on 2026-11-13, run 43338868555 at `ddaae1a7`: "the project's qb64rust.toml wins over
      the settings" twice, on Windows and on Linux, VS Code stable. The step on the oldest VS Code did not run,
      because a debugger test failed before it.)
      Verify in the extension's integration tests, with a workspace `qb64rust.toml` holding `[format]` and
      `keyword-case = "upper"`: Format Document gives `PRINT 1` with the server on and with it off; with
      `keyword-case = "uper"` the document is unchanged and the message names `qb64rust.toml`. If the server's
      message does not name the settings file, change `crates\lsp\src\server.rs` to show the diagnostic's text
      (design D5) and add the case to `crates\lsp\tests\it\server.rs`.
- [x] 4.3 Add to each `qb64rust.format.*` description in `vscode\package.json` that a project's `qb64rust.toml`
      overrides it; describe `[format]` and `[lints]` in `vscode\README.md`; add a `vscode\CHANGELOG.md` entry.
      Verify: `npm run lint` and the extension's tests pass.

## 5. Records

- [x] 5.1 Describe `[format]`, `[lints]`, the order of the layers and the four new options in `docs\MANUAL.md`,
      beside its section on `qb64rust.toml`. Verify: each example in the text is one of the CLI tests' cases.
- [x] 5.2 Add a `DECISIONS.md` row: format and lints are options of `qb64rust.toml`; the project's file wins
      over VS Code settings and the command line wins over the file; a bad settings file stops formatting;
      `--fixes` no longer skips the file. Narrow the two `SOMEDAY.md` rows: `project-settings` (what is left:
      `[language]`, layout as you type, `[dependencies]`) and "Silencing a lint" (what is left: a comment on a
      line, lints for included files). Add a row for the two open questions of `design.md` if they are still
      open.
- [x] 5.3 Update `docs\LAYOUT.md` for `crates\driver\src\settings.rs` if it lists files at that level, and
      `STATUS.md`'s "Next". Verify: `openspec validate project-settings --strict` passes.

## 6. Closing checks

- [x] 6.1 Run `just quick` in the background and read its log. Say what ran and what stood.
- [x] 6.2 Run the `format-corpus` suite and the unit-test family in the suites VM (`check <step> --where vm`),
      to show that `--no-settings` reaches every command there. Read the results from the log.

## Workflow follow-up

- Sync the delta specs into `openspec\specs\` and archive the change after the user's review.
