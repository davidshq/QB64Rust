# Tasks

Standing rules: `CLAUDE.md`. Different here: the `v72` family is recorded with `qb64pe.exe` before any code
of the formatter changes (group 1), because design D4 rests on one of its results.

## 1. Measure with the old compiler first

- [ ] 1.1 Write the family `verification\v72_*` in pairs, one pair for each form of design D9 item 1:
      `v72_<form>.bas` packed, and `v72_<form>_r.bas` (or `_b`) written by hand as the spec says the
      formatter is to write it. Each prints what the form does over the same inputs. Include
      `x = 1: IF a THEN` followed by a block, `IF c GOTO n` as a block, a colon right after `THEN` and after
      `ELSE`, a `REM` comment moved above a single-line `IF`, and a comment moved above a numbered line that
      a `GOTO` names. Record them with `qb64pe.exe`. Verify: the two recordings of every pair are equal, and
      the family passes with `qb64rust` (about two dozen programs, run in the suites VM).
- [x] 1.2 If a recording shows the two forms differ, stop: write what differs in `design.md` under the
      decision it touches, change the spec scenario, and put the change of rule to the user. For
      `x = 1: IF a THEN` the fallback is already in D4. Verify: `design.md` states the outcome of each form
      under "Measured".
- [x] 1.3 Add the family's line to `verification\README.md` and its programs to
      `tests\verification\pass.list`. Verify: the list check of `just quick` passes.

## 2. The settings

- [x] 2.1 Add `one_statement_per_line` and `block_if` to `Settings` and `Overrides` in
      `crates\base\src\layout.rs`, with `OneStatementPerLine` and `BlockIf` in `Settings::set` and both in
      `Settings::apply`. Verify: unit tests in `layout.rs` for the names in any letter case, a bad value's
      message, and `apply` leaving an absent key alone.
- [x] 2.2 Read `one-statement-per-line` and `block-if` in `format_table`, and add them to `FORMAT_KEYS`
      (`crates\driver\src\settings.rs`). Verify: the tests `format_file_contents` and `format_file_errors`
      cover both keys, a value that is not a boolean, and the seven keys in the unknown-key message.
- [x] 2.3 Add the two keys to the `[format]` examples of `docs\MANUAL.md` and `vscode\README.md`, with a
      paragraph on what each does, what is left alone, and that `_ERRORLINE` and error messages name the new
      line numbers. Verify: the example table in `MANUAL.md` is accepted by `qb64rust -y` on a one-line
      program.

## 3. One statement per line

- [x] 3.1 Change `Plan::Layout` to hold a list of `(layout, Apply)` pairs, move the body of `plan`'s loop
      into `Fmt::segment`, and add `format_lines` to `crates\format\src\lib.rs` with `format_file` built on
      it (design D2). No rewrite yet. Verify: `cargo test -p qb64rust-format` and the format oracle test of
      `qb64rust-driver` pass with no change to a recording or to the list of known differences.
- [x] 3.2 Record `in_line_if` on `Event::Colon` in `Fmt::collect`, write `rewritable` (the lines of design
      D6 and D7 ruled out), and make the cuts of design D3 for `one_statement_per_line`: at a colon, after a
      `CASE` header, after a block's `ELSE`, after `ELSEIF … THEN`, not after a leading line number or
      label, and after a built-in statement's name and colon without the colon. Verify: one test in
      `crates\format\tests\it\rules.rs` for each scenario of "One statement per line", "A single-line IF is
      one statement", "The start of a rewritten line", "Lines the rewrites leave alone" and "Rewrites need a
      program without errors" in the delta of `compiler/format`.
- [x] 3.3 Write the comment of a cut line above it (design D5). Verify: tests for the scenarios of "The
      comment of a rewritten line" that need only this setting, and one for a comment above a line inside a
      `FOR` block, at the block's indent.

- [x] 3.4 Print the warning for a rewrite setting without effect in `format` of
      `crates\driver\src\main.rs` (design D6). Verify: tests in `crates\driver\tests\it\cli.rs` for the three
      scenarios of "A rewrite setting without effect", with the output file written and exit status 0.

## 4. Block IF

- [x] 4.1 Push `Event::LineIfEnd` in `Fmt::collect` and lay out an `IfStmt` as a block when `block_if` is on
      (design D4): the header as a segment with ` Then` added for `IF c GOTO x`, the level up, each branch,
      `Else` one level out, `GoTo` before an `ImplicitGoto`, `End If` and the level down. Verify: one test in
      `rules.rs` for each scenario of "Single-line IF to a block" and "A jump written as a line number",
      with `BlockIf` alone and with both settings.
- [x] 4.2 Add tests for a single-line `IF` three deep, one inside a `SELECT CASE` on one line, one after a
      line number, one with a `REM` comment, and one inside `'$FORMAT:OFF`. Verify: each output, compiled
      with `--dump typed`, equals its source's dump apart from line numbers.

## 5. Every oracle program

- [x] 5.1 Note the time of `formatter_equals_the_old_compilers_layout` before this task. In `run` of
      `crates\driver\tests\it\format_oracle.rs`, format each default recording's source with the first
      setting, the second, and both, and check each output as design D9 item 2 says: the typed dump, the C++
      with line numbers masked, the comments, the second formatting, and nothing left to rewrite. Verify: the
      test passes for all three variants of every program, and two temporary bugs (drop the `Else` segment;
      drop the comment of a cut line) each make it fail naming the program and the rewrite; remove them.
- [x] 5.2 Note the time after. If the test grew by more than 60 seconds, move the C++ comparison to a test
      that `just full` runs and `just quick` does not, and keep the typed tree in tier 1. Verify: both times
      and what was done are in `design.md` under D9.
- [x] 5.3 For each program that 5.1 shows a rewrite changes or misses, fix the rule or add the line to those
      left alone, with a test in `rules.rs` and a sentence in `design.md` under "Found while building".
      Verify: 5.1 passes with no program excluded by name.
- [x] 5.4 Add a test to `format_oracle.rs` that formats each packed program of `verification\v72_*` and
      requires the text of its `_r` file (both settings) or `_b` file (`BlockIf` alone). Verify: it passes;
      where the formatter's text differs from a hand-written pair, the pair is rewritten to the formatter's
      text and recorded again with `qb64pe.exe`, and the two recordings are still equal.

## 6. The language server

- [x] 6.1 Read `oneStatementPerLine` and `blockIf` in `format_settings`
      (`crates\lsp\src\server.rs`). Add `format_lines_for_server` to `crates\driver\src\lib.rs` and use it in
      `formatted` for a range: one edit for each touched source line whose output differs, holding all of its
      output lines (design D8). Verify: tests in `crates\lsp\tests\it\server.rs` for the two new scenarios
      of "Format settings from the client" and for "One packed line of several"; the existing range tests
      pass unchanged.
- [x] 6.3 Send the warning for a rewrite setting without effect from `formatted`, once until the client's
      settings or the settings file change (design D6). Verify: tests for the two scenarios of "Warning for a
      rewrite setting without effect".
- [x] 6.2 Answer `textDocument/codeAction` and `codeAction/resolve` (design D8): the list from the last parse
      through `rewritable`, the edit from a format job with one rewrite forced on, and the messages for an
      incomplete program and for `AutoIndent` or `AutoLayout` off. Verify: a test for each scenario of "Code actions for statement lines", and one
      that a request on a line with nothing to rewrite starts no format job.

## 7. The extension

- [x] 7.1 Add `qb64rust.format.oneStatementPerLine` and `qb64rust.format.blockIf` to `vscode\package.json`,
      `config.ts` and `DEFAULT_FORMAT`, send them to the server, pass them to `qb64rust -y` as
      `--format-default` and never to `qb64pe -y` (`vscode\src\format.ts`). For Format Selection with the
      server off, pass `-f:OneStatementPerLine=false -f:BlockIf=false`. Verify: unit tests for the argument
      lists of the three formatters, and for the two descriptions naming `qb64rust.toml` and the old
      compiler.
- [ ] 7.2 Add an integration test: with `qb64rust.format.oneStatementPerLine` on, Format Document turns
      `x = 0: y = 0` into two lines, and the code action "Convert to block IF" is offered on
      `IF a THEN x = 1` with the setting off. Verify: the test passes in the extension's test run.

## 8. Records and closing

- [x] 8.1 Add the `DECISIONS.md` row (the formatter can put each statement on its own line and turn a
      single-line `IF` into a block, both off by default; the comment goes above; only a complete program is
      rewritten). Add the four `SOMEDAY.md` rows of the proposal's "Out of scope", and a sentence to the row
      "`qb64rust -y` of an include file inside a named program" that the rewrites need it for an include
      file on the command line. Add the row of `docs\LAYOUT.md` if a file was added. Update `STATUS.md`'s
      "Next". Verify: `python tools\repo_check\check_repo.py` passes.
- [x] 8.2 Run `just quick` in the background and read its log. Verify: it passes, and the report says what
      ran, what stood and why.

- [ ] 8.3 Before the merge into `main`, once: format every corpus program that has a recording with both
      settings into a scratch folder, and ask the user to start the run in the suites VM that builds each
      with `qb64pe.exe` and compares its output with the source's recording (design D9 item 6). If the VM
      cannot run `qb64pe.exe` on a scratch folder, ask the user how to run it before trying another way.
      Verify: `design.md` holds the command, the counts, and each program that differs with its reason; a
      difference that is not a line number is a bug to fix before the merge.

## Workflow follow-up

- Sync the four delta specs into `openspec\specs\` and archive the change after the user's review.
- The suites that are large runs (`check full`) go in the suites VM, started by the user.
