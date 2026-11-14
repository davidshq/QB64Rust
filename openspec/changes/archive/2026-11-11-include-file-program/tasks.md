# Tasks

Standing rules: `CLAUDE.md`. The decision that allows this change is already recorded (`DECISIONS.md`,
2026-11-11).

## 1. Which symbols a statement declares (design D2)

- [x] 1.1 Add the set of declared symbols and `Symbols::is_declared` to `crates\sema\src\symbols.rs`, filled at
  procedure headers, `DECLARE`, `DIM`, `REDIM`, `COMMON`, `SHARED`, `STATIC`, `CONST`, parameters, `TYPE` blocks
  and members, and labels. Verify with unit tests in `crates\sema`: one declared and one implicit symbol of each
  kind, and `cargo test -p qb64rust-sema` passes.

## 2. Formatting a program with errors (design D1, D3)

- [x] 2.1 Give `format_file` the `complete` argument and apply the incomplete rule in `Fmt::name`
  (`crates\format\src\lib.rs`, `fmt.rs`). Verify with tests in `crates\format\tests\it\rules.rs` for the four
  scenarios of "Names in an incomplete program", and that every existing format test passes unchanged.
- [x] 2.2 Add the driver's helper that decides `complete`, and make `format` (`crates\driver\src\main.rs`) stop
  only on a parser's error: print the diagnostics, write the file, exit 0. Verify with CLI tests in
  `crates\driver\tests\it\cli.rs`: `format_of_a_program_with_an_error` still passes, a new test for
  `call   missing` gets exit 0 and `Call missing`, and one for a file named `lib.bm`.
- [x] 2.3 Add the `DIVERGENCES-QB64PE.md` row: `-y` writes the file for a program whose errors are not syntax
  errors. Verify by running the reference `qb64pe.exe -y` once on a file that holds `call missing`, and record
  in the row what it prints and that it writes no file.
- [x] 2.4 Document the behavior in `docs\MANUAL.md` where `-y` is described. Verify the documented command runs
  as written.
- [x] 2.5 Add a fixture program of three files (`main.bas`, a `.bi` with `COMMON SHARED` names in capitals, a
  `.bm` with SUBs that use them in lower case and call SUBs of `main.bas`) and the agreement test: the `.bm`
  formatted alone and inside the program are equal but for letter case, and every name the first recased has
  the same case in the second. Verify the test passes.
- [x] 2.6 Add a test that formats every program of `tests\verification` recorded `(not compiled)` whose parse
  has no error: no panic, and the output's tokens equal the input's without regard to letter case and blanks.
  Verify it passes, and note the count of programs in `design.md`.

## 3. The server formats with errors, and a named file (design D1, D4)

- [x] 3.1 Add the `FileId` argument to `qb64rust_lsp::Format` and to `format_for_server`; the worker finds it by
  the document's key. Stop only on a parser's error; send no message and publish nothing otherwise
  (`formatted`, `crates\lsp\src\server.rs`). Verify with server tests in `crates\lsp\tests\it\server.rs` for both
  scenarios of "Format failure", and `crates\driver\tests\it\format_server.rs` for a lone `lib.bm` that calls a
  missing SUB.

## 4. The lookup of a document's program (design D5, D7)

- [x] 4.1 Add `main_of` to `crates\driver\src\fixes.rs` and as a hook of `qb64rust_lsp::Hooks`. Verify with unit
  tests: a `main`, no table, a value that is not a string, a missing file, a settings file one folder up.
- [x] 4.2 Document the `[project]` table in `docs\MANUAL.md` beside `[fixes]`. Verify the example file is
  accepted by `qb64rust -c`.
- [x] 4.3 Read `workspaceFolders` from `initialize` and keep them in `Server`. Verify with a server test that
  starts the server with one folder.
- [x] 4.4 Add `crates\lsp\src\owner.rs` with `find_owner` (settings, open, disk, with the limits) and the
  `owners` map, `Doc::opened` and `program_of` in `Server`. Verify with unit tests on scratch folders for each
  scenario of "Program of an included file", "Main program in the settings file" and "Program found on disk",
  and one that stops at the file limit. The scenarios include a `.bas` file that another includes, a chain of
  `.bas` files, and a `.bas` file that `main` names.
- [x] 4.5 Run the lookup as a job on the worker at the events of design D7, and release and parse again when the
  owner changes. Verify with server tests: the includer opens later, the includer closes, the `$INCLUDE` line
  is removed, `qb64rust.toml` changes.

## 5. Answers from the program (design D5, D6, D9)

- [x] 5.1 Build a job's `main` from the disk when the program is not open (`program_main`), and key
  `analyses`, `indexes`, `pending`, `checks` and `latest_job` by the program. Drop a program that is not open
  when its last owned document closes. Verify with a server test: open `lib.bm` alone, edit it, close it, and no
  analysis is left.
- [x] 5.2 Publish only for open documents when the main file is not open. Verify with the scenario "Error
  elsewhere in a program that is not open".
- [x] 5.3 Route `qb64rust/check`, the index requests, document symbols, folding ranges and the rename path
  through `program_of`. Verify with server tests for each scenario of "Answers from the document's program",
  and a rename from `lib.bm` that edits `main.bas` on disk.
- [x] 5.4 Route formatting through `program_of`. Verify with the scenario of "Formatting an included file" and
  a range formatting test.
- [x] 5.5 A lone include file's check publishes and counts the parser's diagnostics only. Verify with both
  scenarios of "Included file without a program".
- [x] 5.6 Measure on the host, with the built server, the time from an edit of one include file of
  `..\civil-war-strategy` to its diagnostics, with `CWSTRAT.BAS` not open. Also measure the lookup when one
  `.bas` program of `tests\verification` opens, a folder of thousands of programs. Record both numbers in
  `design.md`.
  Verify by hand that Format Document on `turn.bm` there formats it and shows no message.

## 6. The program in the editor (design D8)

- [x] 6.1 Send `qb64rust/program` from the server. Verify with server tests for both scenarios of "Program
  notification" and for a change of owner.
- [x] 6.2 Add the notification's name to `vscode\src\server\protocol.ts` and the `qb64rust.program` language
  status item. Verify with the extension's tests (`npm test` in `vscode`) and by hand on `turn.bm`.
- [x] 6.3 Describe include files in `vscode\README.md` and add the entry to `vscode\CHANGELOG.md`. Verify both
  name the `[project]` table as `MANUAL.md` does.

## 7. Closing

- [x] 7.1 Add the `SOMEDAY.md` rows: Build and Run from an include file build its program; `qb64rust -y` of an
  include file inside a named program; a command to choose among several programs; `[project]` `main` as the
  program `qb64rust` builds when no file is named; modules in place of textual `$INCLUDE`. Mark the
  `language-server` row of 2026-10-08 (files that change on disk while not open) as also covering a program
  that is not open. Verify each row names this change as its source.
- [x] 7.2 Add the `DECISIONS.md` row for what this change built, and update `STATUS.md`'s "Next". Verify
  `openspec validate include-file-program --strict` passes.
- [x] 7.3 Run `just quick` in the background and read its log. Verify it passes, and say what ran.

## Workflow follow-up

- Sync the delta specs into `openspec\specs\` and archive the change after the user's review.
