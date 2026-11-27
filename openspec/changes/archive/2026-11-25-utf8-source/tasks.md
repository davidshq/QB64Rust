# Tasks

Standing rules: CLAUDE.md. This change has no `qb64pe.exe` to compare a UTF-8 file with: the check is that a
UTF-8 file and its code page 437 twin write the same C++.

## 1. Measure and decide

- [x] 1.1 Measure with `qb64pe.exe` and with `qb64rust`, as a new verification family: a file whose first line
  is `'$ENCODING:UTF8`; a file with `$ENCODING:UTF8` outside a comment; a file that starts with the bytes
  0xEF 0xBB 0xBF; a UTF-8 `PRINT "─"` that declares nothing. Add the family's line to
  `verification\README.md`. Verify: the recorded outputs exist, and `qb64pe` accepts the comment line. If it
  does not, stop and put design D3 to the user.
  Done 2026-11-24: family `v74`, nine programs, all on `pass.list`. QB64pe accepts the line. It rejects a file
  that starts with a byte order mark, declared or not (`v74_d1`, `v74_d2`), and so does QB64Rust today.
- [x] 1.2 Record the user's choice of the declaration, the line and the key, and a UTF-8 new project, in
  `DECISIONS.md`. Done 2026-11-24: the row of that date.

## 2. The compiler

- [x] 2.1 List every place that takes the bytes of a string literal or a `DATA` item from the source: start
  from the string token's value in `crates\sema\src\check`, `consteval::literal` in
  `crates\sema\src\consteval.rs`, and the reader of `DATA` items. Check that `crates\codegen-cpp` and the wasm
  path take a literal's bytes from the IR, not from a source span. Write the list into `design.md` under D1.
  Verify: the list names each function and file.
- [x] 2.2 Add `crates\base\src\cp437.rs`: the table, the reverse lookup with the 32 symbols, and `text_of`
  (design D1, D4). Add the unit test in `crates\lsp` that its `cp437` table has the same entries. Verify: unit
  tests for each of the 256 bytes there and back, for `♥`, and for a character outside the table;
  `cargo test -p qb64rust-base -p qb64rust-lsp`.
- [x] 2.3 Add the default encoding to `SourceFile`, and set it in `FileLoader::add_file`, the main file's
  load and the unit-test loader, from the new `encoding` key of `[project]` in
  `crates\driver\src\settings.rs` and the folder rule (design D2). Verify: unit tests of the key (both values,
  a bad value with its line) and CLI tests of the four scenarios of "A file's encoding".
  Done 2026-11-24: `SourceFile::project_encoding`, set by `FileLoader::with_encoding` and the new
  `Loader::main_encoding`; tests in `crates\driver\tests\it\encoding.rs`.
- [x] 2.4 Parse the `'$ENCODING` line in `crates\syntax` as a property of the file's tree, with its errors
  (design D3), and make a leading byte order mark a trivia token (design D6). Verify: frontend snapshots for
  each scenario of "The $ENCODING line" and "A byte order mark"; the existing snapshots are unchanged.
  Done 2026-11-24: `crates\syntax\src\encoding.rs`, `Tree::encoding`, the token `ByteOrderMark`. A frontend
  program needs `' TEST:` on its first line, so the mark is tested in `encoding.rs` (`a_byte_order_mark`).
- [x] 2.5 Convert at each place of task 2.1, and report the two errors: a character outside code page 437,
  and a file that is not valid UTF-8. Verify: frontend snapshots for each scenario of "Text of a UTF-8 file",
  "The characters of bytes 1 to 31 and 127", "A character outside code page 437", "Comments of a UTF-8 file",
  "A UTF-8 file must be valid" and "Text that stays as written". Edit `docs\reference\chapters\pipeline.md`,
  and add the page `docs\reference\entries\$encoding.md` with `reference.py skeleton`.
- [x] 2.6 Add the twin test: one program with string literals, `DATA`, a `CONST` string and a string
  `SELECT CASE`, saved as code page 437 and as UTF-8, each with its `'$ENCODING` line. Verify: a test in
  `crates\driver\tests\it` compares the two C++ files and finds them equal. Add the UTF-8 file to the corpus
  with its twin's recorded output, as a known-output program.
  Done 2026-11-24: `verification\v74_f1_twin_cp437` and `v74_f2_twin_utf8`, the second judged by
  `diverged\v74_f2_twin_utf8.out.txt`, a copy of the first's recording (D-077).
- [x] 2.7 Check that every program of `tests\` and `verification\` still writes the same C++. Verify:
  `cargo test -p` for `syntax`, `sema` and `driver` passes with no snapshot changed; the 15 UTF-8 programs
  counted in `design.md` are among them.
  Done 2026-11-24: no snapshot changed. No program but the new ones has an `'$ENCODING` line or starts with a
  byte order mark, so the 15 take the code page 437 path, which keeps a string's bytes. The format oracle found
  that `-y` dropped a leading mark (`v74_d2`); `format_lines` keeps it now.

## 3. The lint

- [x] 3.1 Add `undeclared-utf8` to `CODES` in `crates\sema\src\lints.rs` and write the check (design D7).
  Update the messages and tests that say "nine codes". Verify: `tests\frontend` snapshots for the four
  scenarios of the lint's requirement and one for an included file; the unit tests of
  `crates\driver\src\lints.rs` and `settings.rs`. Edit `docs\reference\chapters\lints.md`.

## 4. The formatter and the convert command

- [x] 4.1 Check where `crates\format` measures a line's width, and whether it counts bytes. Record what was
  found in `design.md`. Add a format test for the scenario of "Formatting a UTF-8 file" and one for a string
  with a character outside code page 437. Verify: `cargo test -p qb64rust-format`.
- [x] 4.2 Add `qb64rust convert` in `crates\driver\src\main.rs` and `convert.rs` (design D9), with its help
  text. Verify: CLI tests for the three scenarios of "The convert command", for a file of ASCII only, and for
  `--to cp437` on a file with `日` in a comment.

## 5. The editor

- [x] 5.1 Set each file's default encoding in `crates\lsp\src\analysis.rs` as the driver does, and publish the
  warning of the spec delta `editor/language-server` in `server.rs` (design D10). Verify: server tests for its
  three scenarios, and one that the document's other diagnostics are the same with and without the warning.
- [x] 5.2 In `vscode\src\diagnostics.ts`, read a build's column as a count of UTF-8 bytes when the document's
  encoding is UTF-8 (design D5). Verify: a unit test with `PRINT "é": PRINT x(`.
- [x] 5.3 Make "QB64: New Project" write the `encoding` key and `.vscode\settings.json` (spec delta
  `editor/new-project`). Confirm in VS Code's documentation that a workspace `"[qb64rust]"` setting of
  `files.encoding` applies to a file opened afterwards, and name the page in `design.md`. Verify: the
  extension's tests for the four scenarios.
  Done 2026-11-24, see design D10: `main.bas` is also opened as `utf8`. The integration tests are written and
  compile; they run in CI, not on the host, since they open a VS Code window.

## 6. Documents and closing

- [x] 6.1 Write the manual: `docs\MANUAL.md`, "The settings file" (`encoding`), a new section "UTF-8 source
  files" (the line, what is converted, the two errors, the lint, `convert`, and that such a project builds
  correctly only with QB64Rust), and the options table. Verify: each command in the section runs as written.
- [x] 6.2 Add the `DIVERGENCES-QB64PE.md` row for a declared file, and the `SOMEDAY.md` rows: Unicode strings
  at run time; UTF-8 bytes in a string of a declared file; `$ENCODING` outside a comment, behind the
  extensions switch. Update `docs\LAYOUT.md` for the new files. Run `reference.py render` and
  `reference.py --check`.
- [x] 6.3 Run `just quick` in the background and read its log. Update "Next" in `STATUS.md`. Tell the user
  that converting each game's files is a step in that game's repo.
