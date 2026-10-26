# Tasks

Standing rules: CLAUDE.md. What this change does differently:

- Its oracle is recorded with `qb64pe.exe` (group 2) before any layout rule is written. Recordings are made in one
  batch, before the closing runs.
- The facts it rests on that are not measured yet (task 1.1) go into `design.md` under a new section "Measured".
- Beside tier 1, it runs the extension's tests: `npm run test:unit`, `npm run lint` and `npm run test:integration`
  with `QB64RUST_TEST_QB64RUST` set (`vscode\DEVELOPMENT.md`).

## 1. Measured first (design "Risks", D4)

- [x] 1.1 With the clone's `qb64pe.exe` and the five defaults given explicitly, measure the following on scratch
  programs in the scratchpad, then write each answer into `design.md` "Measured", and amend the spec
  `compiler/format` where an answer differs from it:
  - whether a name declared in an included file respells the main file;
  - whether user type names and member names are respelled (D3);
  - what happens to a last line with no line ending, and to a line that is only spaces;
  - the indent of a label line and of a line-number line;
  - the spacing before a comment that follows a statement;
  - `$IF` inside a SUB;
  - the output of a program with an error that appears only in the second pass.

  Verify: each answer is in "Measured" with the scratch program's text.

## 2. The oracle recorded (D6; spec `testing/format-oracle`)

- [x] 2.1 Add `run_legacy_tests.py --suite format-corpus`. With `qb64pe`, it records `tests\format\<set>\<path>`
  for the accepted programs of the corpus, upstream, snippets and Civil War Strategy, each in its own folder with
  `-y -m` and the five defaults. With any other compiler, it compares, line endings taken as LF, and reads a
  `.divergences` sidecar. Add tests of the recording and comparison rules to `test_run_legacy_tests.py`, and a
  section to `tools\legacy_tests\README.md`.
  Verify: `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` passes.
- [x] 2.2 Record the oracle with `qb64pe`. Add `tests\format\README.md`, saying what is recorded, with which
  settings, how to record it again, and the count written by `tools\legacy_tests\counts.py`.
  Verify: a second recording changes no file; `tools\repo_check\check_repo.py --untracked` passes over
  `tests\format\`.
- [x] 2.3 Run the "same program" check on the recordings themselves: `qb64rust --dump typed` of each source
  against the dump of its recorded output, with spans removed and names compared without regard to case, using a
  script in the scratchpad. A difference is a case for D8: add a `DIVERGENCES.md` row, an `UPSTREAM.md` row and a
  sidecar.
  Verify: the result is in "Measured", either "none" or the rows that were added.

## 3. Settings and `-y` on the command line (D4, D9; specs `compiler/format`, `compiler/cli`)

- [x] 3.1 Add `crates\base\src\layout.rs`: `Settings`, `KeywordStyle`, `Settings::set(name, value)` with QB64pe's
  names, values, range check and keyword-style rule, and `Default` set to QB64pe's built-in defaults. Add unit
  tests for each setting, a bad value, the size range, and both keyword settings.
  Verify: `cargo test -p qb64rust-base`.
- [x] 3.2 Add the crate `crates\format`: `Cargo.toml` in the workspace, `lib.rs` `format_file`, which at this
  stage writes every line as read, and `apply.rs`, the port of `apply_layout_indent$`. Add unit tests of
  `apply.rs` against the steps of `format.bas`: `sp`/`sp2` outside and inside strings, the indent from leading
  spaces, `AutoIndent` off, `AutoLayout` off with case carried over, `?`, and the closing quote.
  Verify: `cargo test -p qb64rust-format`; `cargo deny check` (no new outside dependency).
- [x] 3.3 Add to the driver `-y` and the check of the layout settings in a build. In `build_settings`, the layout
  names go through `Settings::set` and are then ignored. Add `format_program`, which runs `check_program` and then
  `format_file` on the main file and writes CRLF lines to `-o`. Also cover "requires an output file", exit 1
  with diagnostics, no output on error, and the usage text. Add CLI tests to `crates\driver\tests\cli.rs` (not
  `--ignored`, since nothing is built), one for each scenario of `compiler/format` "Format command", "Layout
  settings" and "Program with an error", and for `compiler/cli` "A layout setting in a build".
  Verify: `cargo test -p qb64rust-driver`.
  *The two scenarios that need the layout itself, "Runner's format line" (`runners_format_line`, `#[ignore]`d: it
  reads the clone's `format_tests`) and "Both keyword settings" (`format_both_keyword_settings`), came with group
  4.*
- [x] 3.4 Add `crates\driver\tests\format_oracle.rs`, the tier 1 check of D6: equal to the recording, the same
  typed tree, idempotent. Programs that differ go into `tests\format\known_differences.list`, which is
  shrink-only and starts by naming every program, since layout is not built yet. The test prints the count of
  statement kinds no recorded program reached.
  Verify: `cargo test -p qb64rust-driver --test format_oracle` passes, with every program on the list; record its
  time in the test's comment.

## 4. The layout rules (D2, D3)

- [x] 4.1 Add `tools\format\layout_words.py`, which writes `crates\format\src\words.rs` from the `SCase$`/`SCase2$`
  literals of the clone's `qb64pe.bas`, with `--check`, its tests (`test_layout_words.py`) and a `README.md`.
  Verify: the tool's tests pass; `--check` passes.
- [x] 4.2 Add the symbol kinds `Type` and `Member` in `crates\sema\src\symbols.rs`, recorded where `check\`
  resolves a type name and a member. Extend `dump_symbols` and add front-end tests (`tests\frontend`, a `typed`
  or symbol mode) that show a type and a member with their references.
  Verify: `cargo test`; the existing symbol tests are unchanged.
- [x] 4.3 Write `line.rs` (labels, line numbers, `:`, the comment after statements), `expr.rs` (expressions) and
  the indent by block depth with `IndentSubs`, `TYPE` and `DECLARE LIBRARY`, `ELSE`/`CASE`/`$ELSE`, and `$IF`.
  Write the keyword case from `words.rs` and the built-in table, and the names from the symbol table. Add unit
  tests for each rule, each test naming the handler of `qb64pe.bas` it follows.
  Verify: `cargo test -p qb64rust-format`; QB64pe's `format_tests` through `--suite format --qb64
  target\release\qb64rust.exe` pass 24 of 24.
  *Built 2026-10-25: the lines, their indent, labels, `:` and comments are in `walk.rs` (the line pass) and
  `stmt\mod.rs` rather than a `line.rs` (design D9 says so); the rules' tests are `crates\format\tests\rules.rs`
  (31, each with `qb64pe -y`'s output for its input).*
- [x] 4.4 Write the statement families in `stmt\*.rs`, one family at a time. For each family, take the programs
  that pass off `known_differences.list`. The forms whose handler sets no layout go into `KEPT`, as the oracle
  shows them. A program that stays on the list keeps a one-line reason.
  Verify: `cargo test -p qb64rust-driver --test format_oracle`. At the end of the group, the list holds only
  programs with a reason that is a decided divergence or a recorded QB64pe oddity, ideally none.
- [x] 4.5 Add `crates\README.md`'s crate map row and graph edge for `format`, the build and test commands for
  `-y`, and the `tools\format\` row in `CLAUDE.md`'s layout table and `tests\format\`.
  Verify: `tools\repo_check\check_repo.py --untracked` passes.

## 5. The language server (D5; spec `editor/language-server`)

- [x] 5.1 In `crates\lsp`: add the `Format` hook in `lib.rs` and in `serve`/`run_stdio`; advertise formatting and
  range formatting in `capabilities()`; add the settings from `initializationOptions.format` and
  `workspace/didChangeConfiguration`. Add the two requests on the check worker, with cancellation by a newer edit
  (`ContentModified`), the whole-document edit, the line edits of a range, and on failure the diagnostics plus
  `window/showMessage`. In the driver, add `format_for_server` and pass it in `main.rs`.
  Verify: `cargo test -p qb64rust-lsp`.
  *Built 2026-10-25: `serve` and `run_stdio` take `Hooks { check, format }` (the driver's `server_hooks()`); a newer
  edit is seen by the document's version when the answer comes.*
- [x] 5.2 Add protocol tests in `crates\lsp\tests\` for each scenario of the delta spec `editor/language-server`:
  unsaved text, unsaved include, already formatted, upper case keywords, a setting changed, syntax error, one line
  of a block. Update the capability test of "Server start".
  Verify: `cargo test -p qb64rust-lsp`.
  *Built 2026-10-25 in two places, as the check's tests are: the protocol's rules with a stand-in formatter in
  `crates\lsp\tests\server.rs` (whole edit with each line's own line end, no edit, settings from the client and a
  change of them, failure with diagnostics and the message, range lines, `ContentModified` after an edit), and the
  seven scenarios with the driver's formatter in `crates\driver\tests\format_server.rs` (the real output needs
  `sema`, which `lsp` never depends on).*

## 6. The extension (D7; spec `editor/formatting`)

- [x] 6.1 Add the settings `qb64rust.format.*` in `package.json` and `config.ts` `formatSettings()`; send the
  `format` object in `server\protocol.ts` `initializationOptions`; send `didChangeConfiguration` on a change. Add
  `compiler\qb64rust.ts` `format`, and `-f:` settings for `compiler\qb64pe.ts` `format`. Add unit tests of the
  arguments and the options object.
  Verify: `npm run test:unit`, `npm run lint`.
- [x] 6.2 `format.ts`: implement the choice of formatter, Format Selection by kept lines, and no fallback after a
  failure. In `extension.ts` and `languageServer.ts`, register the extension's provider only while the server is
  not running.
  Verify: `npm run compile`, `npm run lint`, `npm run test:unit`.
  *Built 2026-10-25: the choice and the kept lines are in `formatPlan.ts` (no VS Code import, unit-tested in
  `test\unit\format.test.ts`); `LanguageServer.onDidChangeRunning` drives `registerWhileServerOff`.*
- [x] 6.3 Integration tests (`vscode\test\`): format with the server running, where no temporary file appears and
  no formatter choice is asked; with the server off through `qb64rust -y`; with `buildWith` `qb64pe` through
  `qb64pe -y` with the settings; keyword case `lower` in both paths; Format Selection; a syntax error. The
  existing format suite keeps passing.
  Verify: `npm run test:integration` with `QB64RUST_TEST_QB64RUST` set.
- [x] 6.4 In `vscode\README.md` "Formatting", cover the formatter used in each case, the settings, that
  `config.ini` is no longer read, and Format Selection. Add an entry to `CHANGELOG.md` under "Unreleased".
  Verify: the sections exist.

## 7. Close

- [x] 7.1 Update `tools\close\close.py`: when `crates\format`, `tests\format\`, `tools\format\`, `crates\sema`'s
  symbols or the driver's `-y` changed, the plan adds `--suite format` and `--suite format-corpus` with the new
  compiler. Add the tests in `test_close.py` and a line in `tools\close\README.md`.
  Verify: `python -m unittest tools/close/test_close.py`.
- [x] 7.2 Record a `DECISIONS.md` row: the formatter is the new compiler's; the extension's layout settings
  replace `config.ini`; the verification families are left out of the oracle.
  Verify: the row names `design.md` D1, D6 and D7.
- [x] 7.3 Do the closing runs: `python tools\close\close.py --plan`, running each command on its own, then
  `tools\close\close.py --check` passes. After that, change `STATUS.md`'s "Next" (M5b's first change is built;
  the next is `m5b-language-features`), take `tests\*\README.md` counts from `counts.py`, and run
  `python tools\testtime\test_time.py`. Report the statement kinds the oracle never reached.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`). At the archive, change the Purpose of
  `openspec\specs\editor\formatting\spec.md`: it formats with the new compiler, and goes to QB64pe's `-y` only
  when the build is QB64pe's.
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
- Propose `m5b-language-features` (hover, references, rename, completion).
- The oracle holds the default settings only (review, 2026-10-25): record a small set with `qb64pe -y` under each
  of the five other settings (AutoIndent off, AutoIndentSize 2, IndentSubs off, AutoLayout off, keywords in
  capitals and in lower case), about 30 programs with blocks, comments, `?`, unclosed strings and `$FORMAT:OFF`,
  checked in tier 1 beside the oracle. At the start of `m5b-language-features`, or as a small change of its own.
- At the merge into `main`: tick `ROADMAP.md` M5b's first item (working rule 11).
