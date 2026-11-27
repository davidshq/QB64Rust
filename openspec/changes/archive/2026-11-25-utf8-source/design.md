# Design

## Context

Motivation: `proposal.md`, "Why". The user asked for the proposal on 2026-11-24, and chose it as written that day
over detecting UTF-8 from the bytes (`DECISIONS.md` 2026-11-24). What the code does today:

- **Source is bytes.** `crates\base\src\lib.rs` says so at its top (design D2 of the first front end), and
  `SourceFile::line_col` gives byte columns. Spec `compiler/pipeline`, "Source is bytes", requires it.
- **Files enter the source map in a few places.** `FileLoader::add_file` and two more calls in
  `crates\driver\src\lib.rs`, `crates\lsp\src\analysis.rs` (an open document, and an included file read from
  disk), and the unit-test loader in `crates\driver\src\unit_tests\`.
- **A file name in `$INCLUDE` is already read as UTF-8** (`include_name` in `crates\syntax\src\program.rs`).
- **`$VERSIONINFO` text must already be UTF-8.** `manifest_refusal` in `crates\sema\src\check\mod.rs` makes a
  byte that is not UTF-8 a compile error (user, 2026-10-16). `verification\v40_g7_utf8_in_name.bas` holds
  `'café'` in UTF-8.
- **The language server already converts.** It gets a document's text from the editor and turns it into bytes
  with the document's encoding (`crates\lsp\src\encoding.rs`: a single-byte table, or UTF-8). Positions go back
  to the editor the same way. Its code page 437 table is generated from iconv-lite
  (`crates\lsp\src\encoding_tables.rs`).
- **The extension** sets `files.encoding` to `cp437` for the `qb64rust` language (`vscode\package.json`). A
  workspace setting wins over it (spec `editor/language-support`). `vscode\src\diagnostics.ts` (`lineRange`)
  reads a build's column as a byte column that equals the character column.
- **Lints** have nine codes (`CODES` in `crates\sema\src\lints.rs`). They are printed only with `-w` and are
  published by the language server.
- **The settings file** has a `[project]` table with `main` and `exclude` (`crates\driver\src\settings.rs`).
- **Counted on 2026-11-24:** of 10,922 programs under `tests\` and `verification\`, 10,903 are ASCII only, 4
  are code page 437, and 15 are valid UTF-8 with a byte above 127. Thirteen of the 15 have it only in a
  comment; two have it in `$VERSIONINFO`. QB64pe's own `tests\` folder has 24 code page 437 files and no UTF-8
  one.

Two of the user's rulings bound the design:

- A program QB64pe 4.7.0 accepts behaves here as it does there (2026-11-11).
- A form QB64pe rejects is accepted only behind the extensions switch (2026-11-11). That switch is not built.

## Goals / Non-Goals

**Goals:**

- A file declared UTF-8 gives the same program as the same text saved as code page 437.
- A file that declares nothing compiles to the same C++ as before, byte for byte.
- The declaration can be read by QB64pe without an error.

**Non-Goals:**

- Guessing a file's encoding from its bytes. The guess is used only for a warning.
- Character columns in diagnostics.
- Any change at run time: libqb, the code generators and the wasm target are not touched.

## Decisions

### D1. The bytes stay UTF-8 in the source map; the conversion happens where text is taken

A UTF-8 file's bytes go into the `SourceMap` unchanged. The lexer and parser already pass a byte above 127
through inside a string, a `DATA` item and a comment. `sema` converts when it takes the bytes of a string
literal or a `DATA` item.

*Why:* everything that works on bytes keeps working with no change: the lossless tree, the formatter, byte
offsets, the language server's positions. A comment can hold any character, because a comment is never taken.
The two errors are reported at the exact character.

*Alternative, rejected:* convert the whole file to code page 437 bytes when it is loaded. It is one place
instead of several. But a comment with `…` or `日本語` would then be an error or be lost, the formatter could
not write the file back, and every position would need a map back to the file. Thirteen of our own test
programs have such a comment.

*Where:* a function `cp437::text_of(bytes, utf8) -> Result<Vec<u8>, BadChar>` in `crates\base`. Its callers
are found in task 2.1. The known ones are the string token's value in `crates\sema\src\check` and
`consteval::literal` in `crates\sema\src\consteval.rs`, and the reader of `DATA` items. The code generators
must take a literal's bytes from the IR, never from the source span; task 2.1 checks this.

*The takers, listed by task 2.1 (2026-11-24).* `consteval::literal` reads numbers only, so it is not one. These
take a string literal's or a `DATA` item's bytes from the source and are converted:

- `Checker::expr_as` in `crates\sema\src\check\expr.rs`, the `ast::Expr::Literal` arm: every string literal used
  as a value. This covers `PRINT`, `SELECT CASE`, `PRINT USING`'s format, arguments, and `_EMBEDDED$`'s handle,
  whose rule then allows only ASCII letters and digits.
- `Checker::const_value` in `crates\sema\src\check\constants.rs`: a string literal in a `CONST` expression.
- `Checker::console_source` in `crates\sema\src\check\io.rs`: the prompt of a console `INPUT` and `LINE INPUT`.
- `Checker::data` in `crates\sema\src\check\io.rs`: each `DATA` item, quoted or not.

These take a string literal's bytes and are left as written, as file or C names (D8):
`Checker::library` (the names of `DECLARE LIBRARY`) and `Checker::c_name` (an `ALIAS`), both in
`crates\sema\src\check\library.rs`. The text of a metacommand (`$VERSIONINFO`, `$EMBED`, `$EXEICON`,
`$INCLUDE`) is never a `StringLit` token and is not converted.

`crates\codegen-cpp` does not depend on `crates\syntax` and has no source map: it takes every literal's bytes
from the IR (`ExprKind::Str`, `Program::data`). It reads spans only for line numbers (`coverage.rs`,
`call_trace.rs`, `debug.rs`). The wasm target uses the same code generator.

### D2. A file's encoding is found in two steps

- **The default** is set when the file enters the source map. `SourceFile` gets a field for it. The loader
  sets UTF-8 when the program's settings file has `encoding = "utf-8"` and the file's real path is in the
  settings file's folder or below it. `Loader::folders` already knows each file's folder. A file with no real
  path (a test's in-memory file) gets code page 437.
- **The file's own line** is found by the parser as a property of the file's tree, and overrides the default.

`sema` asks one function for a file's encoding. *Where:* `crates\base\src\lib.rs` (`SourceFile`),
`crates\driver\src\lib.rs` (`FileLoader::add_file` and the main file), `crates\lsp\src\analysis.rs`,
`crates\driver\src\unit_tests\loader.rs`, and a new module in `crates\syntax` for the line.

*Why the folder rule:* a library's files from the compiler's folder, or another project's, were written by
someone else and are code page 437 unless they say otherwise.

### D3. The line is a comment: `'$ENCODING:UTF8`

It follows QuickBASIC's own metacommands, which are written in comments (`'$DYNAMIC`, `'$INCLUDE:`). QB64pe
reads it as a comment, so a file with the line still compiles there. That keeps this change outside the
extensions switch: no form that QB64pe rejects becomes accepted, except a byte order mark in a UTF-8 file (D6).
Task 1.1 measures what `qb64pe` does with the line before anything is built on it.

Since QB64pe reads every such line as a comment, also `'$ENCODING:LATIN1` (`verification\v74_a4`), a line with
another value, or a second line that disagrees with the first, is a warning and not an error (user, 2026-11-24).
The program is built as QB64pe builds it.

`$ENCODING:UTF8` outside a comment is not accepted. It would be a new form, which needs the switch. A
`SOMEDAY.md` row names it.

Python's PEP 263 is the model for a declaration in a comment that an older tool ignores
(https://peps.python.org/0263).

*Alternative, rejected:* a byte order mark as the declaration. FreeBASIC and PureBasic do this, and FreeBASIC's
forum has reports of garbled text from files without one. Most editors, VS Code among them, write no mark.

*Alternative, rejected:* read every valid UTF-8 file as UTF-8. It breaks programs that hold UTF-8 bytes in a
string on purpose, for `_UPRINTSTRING` with encoding 8, which QB64pe accepts. The user chose the declaration
(`DECISIONS.md` 2026-11-24).

### D4. The table is iconv-lite's, plus the 32 symbols

Bytes 128 to 255 map as in `encoding_tables.rs`, so that a file VS Code saved as `cp437` and the same text
saved as UTF-8 give the same bytes. For the bytes 1 to 31 and 127, iconv-lite gives the control characters.
Code page 437's screen draws symbols for them (`☺`, `♥`, `►`, `⌂`). A game's source may hold `♥` in a string,
so the reverse lookup also takes those 32 symbols. `convert --to utf8` writes a byte below 128 as it is, so a
round trip is exact.

*Where:* a new module `cp437.rs` in `crates\base`, with the table and the reverse lookup. A unit test in
`crates\lsp` checks that its generated `cp437` table has the same 256 entries.

### D5. Columns stay byte columns

A diagnostic's column in a UTF-8 file counts bytes. The language server already turns byte offsets into the
editor's positions by the document's encoding, so the editor shows the right place. `diagnostics.ts` must do
the same for a build's output: for a UTF-8 document it counts the column in UTF-8 bytes.

*Why:* a character column would need a second kind of position through `base`, `sema` and every test that
prints one. The cost is a column on the command line that is too high by the extra bytes of the accents before
it on the line. The line number is always right.

### D6. The byte order mark

The lexer makes a leading 0xEF 0xBB 0xBF one trivia token, in every file. When the file is not UTF-8, the
parser gives an error of its own at line 1, column 1 (`scan` in `crates\syntax\src\encoding.rs`). The program
still fails at line 1, as it did before this change and as it does in QB64pe. Task 1.1 records QB64pe's
error, so the unchanged-behavior requirement can be tested.

Measured (task 1.1): QB64pe rejects every file that starts with the mark, "Unexpected character on line" at line
1, also with `'$ENCODING:UTF8` after it (`verification\v74_d1_bom`, `v74_d2_bom_declared`). Accepting it in a
UTF-8 file is therefore a form QB64pe rejects, accepted outside the extensions switch. The user chose that
(2026-11-24): some Windows tools write the mark, and such a file fails in QB64pe anyway. It is a
`DIVERGENCES-QB64PE.md` row with the declared file's, and `v74_d2` gets a `diverged\` output.

### D7. The lint runs on bytes

`undeclared-utf8` needs no parse of its own: for each file with the default encoding and no line, check that
the whole file is valid UTF-8, then find the first string literal or `DATA` item with a byte above 127. It is
one warning per file. *Where:* `crates\sema\src\lints.rs`, a tenth entry of `CODES`.

The lint is silent when a settings file names an encoding, even `"cp437"`, and when the file holds
`'$ENCODING:CP437`. That line is how a program that wants UTF-8 bytes in its strings says so.

### D8. `$VERSIONINFO` and file names

Both are left alone, which is right in a UTF-8 file: the manifest wants UTF-8, and file names are already read
as UTF-8. So a UTF-8 file can hold `ProductName='café'`, which a code page 437 file cannot.

### D9. The convert command

`qb64rust convert --to utf8|cp437 <files>` is a subcommand beside `test` and `toolchain`. It reads every file
first and writes only when all can be converted. It works on whole files and knows nothing of BASIC: a comment
is converted like a string. With `--to cp437`, a comment that holds a character outside the table stops the
conversion and is named.

It does not add the `'$ENCODING` line, since a project usually sets the key in `qb64rust.toml` instead.

*Where:* `crates\driver\src\main.rs` and a new module `convert.rs`.

### D10. The editor

- The server publishes a warning when a UTF-8 file is open in another encoding (spec delta
  `editor/language-server`). It still parses the bytes the client's encoding gives. For `cp437` these are the
  file's own bytes, since a single-byte table loses nothing, so every other diagnostic stays right.
  *Where:* `crates\lsp\src\server.rs`.
- "QB64: New Project" writes the key and a `.vscode\settings.json` that sets
  `"[qb64rust]": { "files.encoding": "utf8" }`. A workspace setting wins over the extension's default (spec
  `editor/language-support`, "User override"). *Where:* `vscode\src`, the `newProject` command.
- *Checked for task 5.3 (2026-11-24).* A folder's `.vscode\settings.json` applies only once that folder is
  opened as the workspace; the command opens `main.bas` in the current window. So it also opens `main.bas` with
  `workspace.openTextDocument(uri, { encoding: "utf8" })`, which `@types/vscode` 1.100 (the extension's
  `engines.vscode`) declares, and whose `TextDocument.encoding` is "the file encoding of this document that will
  be used when the document is saved" (`vscode\node_modules\@types\vscode\index.d.ts`). *Not checked:* that
  a `"[qb64rust]"` block may hold `files.encoding` in a workspace file. VS Code's page "User and workspace
  settings" (https://code.visualstudio.com/docs/configure/settings) names language-specific settings, but the
  search did not show whether `files.encoding` is one, and the page could not be fetched without a subagent
  (`CLAUDE.md` working rule 1). The extension's own `package.json` already sets it for the language the same way,
  as a default.
- The extension's default stays `cp437`. A file opened alone, outside a project, is still most likely an old
  program.

## Risks / Trade-offs

- [A place that takes literal bytes is missed, and a UTF-8 file's text reaches the program unconverted] →
  Task 2.1 lists the places before any is changed. The test that compares the C++ of the two savings covers
  string literals, `DATA`, `CONST` strings and a string in a `SELECT CASE`.
- [The formatter measures a line's width in bytes when it lines things up] → Task 4.1 checks
  `crates\format` for it. If it does, a UTF-8 line with accents is laid out a few columns off; the task
  records what was found and fixes it only if the layout is wrong. *Found (2026-11-24):* it measures no width.
  `apply.rs` writes a line's indent as spaces and copies the rest of its layout, as QB64pe's
  `apply_layout_indent$` does, and nothing is aligned to a column. Letter case is changed only in ASCII. The
  one fix needed was the byte order mark, which no layout wrote (`format_lines` in `crates\format\src\lib.rs`).
- [A user declares a project UTF-8 while some files are still code page 437] → "A UTF-8 file must be valid"
  stops the build and names the file and the byte. `convert` fixes it.
- [The lint is not seen on the command line without `-w`] → It is always shown in the editor. This is how
  every lint works; the manual's section on new files says to use `-w`.
- [A file declared UTF-8 is drawn wrong by QB64pe] → It is a `DIVERGENCES-QB64PE.md` row. The line compiles
  there, and the manual says that a UTF-8 project builds correctly only with QB64Rust.
- [Two cp437 tables drift apart] → The unit test of D4.

## Open Questions

These two were put to the user on 2026-11-24. Both are answered: the user chose the proposal as written
(`DECISIONS.md` 2026-11-24).

1. **Its own key and line now, or wait for the extensions switch?** Answered: its own key and line (user,
   2026-11-24, `DECISIONS.md`).
2. **Does a new project default to UTF-8?** Answered: yes (spec delta `editor/new-project`).

One more, which can wait:

3. **`convert` in the compiler, or a script?** Taken: in the compiler, so that a game's repo needs no Python
   to convert its files (the user's ruling of 2026-11-16, that a BASIC program should not need another
   language). The user may cut it; nothing else depends on it.
