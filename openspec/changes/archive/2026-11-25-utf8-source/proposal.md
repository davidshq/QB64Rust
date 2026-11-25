# Proposal

## Why

The compiler copies a source file's bytes into the program, and the program's screen draws each byte as one
code page 437 character. So a source file that holds `─` or `é` must be saved as code page 437. Most editors
save UTF-8 instead, where `─` is three bytes, and the program then draws three wrong characters with no error
or warning. The files that are saved as code page 437 show as garbage on GitHub and in most tools: 138 source
files of the four game ports, counted on 2026-11-24. Claude's own edit tools write UTF-8, so each edit of such
a file risks damaging it.

Other languages with an old character set have the same step this change adds. Microsoft's C++ compiler
separates the source character set from the execution character set (`/source-charset`,
`/execution-charset`). FreeBASIC and PureBasic read UTF-8 source only with a byte order mark, which their own
users trip over. Python began with a declaration per file (PEP 263). QB64pe has no such step.

## What Changes

- A source file can be **declared UTF-8**. The compiler then stores each character of its string literals and
  `DATA` items as that character's code page 437 byte. The program is the one the same text saved as code
  page 437 gives: `LEN("─")` is 1, and the screen, files and every string function work as before.
- A file is declared in one of two ways:
  - a line `'$ENCODING:UTF8` in the file. It is a comment to QB64pe, so the file still compiles there;
  - `encoding = "utf-8"` in the `[project]` table of `qb64rust.toml`, for every file in the project's folder.
    A file's own line wins.
- **Nothing changes for a file that is not declared.** Its bytes are copied as today. Programs that hold UTF-8
  bytes in strings on purpose (for `_UPRINTSTRING` with encoding 8) keep working.
- In a declared file:
  - a character with no code page 437 byte in a string or a `DATA` item is a compile error that names the
    character, the line and the column;
  - a comment may hold any character;
  - a byte sequence that is not UTF-8 is a compile error;
  - `$VERSIONINFO` text stays UTF-8, as the executable's manifest needs.
- A new lint, `undeclared-utf8`: a file that declares nothing, is valid UTF-8, and has a character above 127
  in a string. This is the file a new user writes in an editor without the extension.
- A new command `qb64rust convert --to utf8|cp437 <files>` rewrites source files from one encoding to the
  other.
- The editor: the language server warns when a UTF-8 file is open in another encoding, and "QB64: New
  Project" makes a UTF-8 project.
- **Not in this change:**
  - Unicode strings at run time (printing `日本`, UTF-8 text files). A `SOMEDAY.md` row.
  - A way to write UTF-8 bytes into a string of a declared file, for `_UPRINTSTRING` with encoding 8 and
    `_TITLE`. Such text goes in a file that is not declared. A `SOMEDAY.md` row.
  - Converting the game ports' files. That is done in each game's repo, with the new command.
  - A change of the default. An existing program is never read as UTF-8 unless it says so.

## Capabilities

### New Capabilities
- `compiler/source-encoding`: how a file is declared UTF-8, what the compiler does with its characters, the
  errors, and the `convert` command.

### Modified Capabilities
- `compiler/pipeline`: "Source is bytes" excepts a file declared UTF-8, which must be valid UTF-8.
- `compiler/lints`: a tenth code, `undeclared-utf8`. The requirements that count or list the codes name it.
- `editor/language-server`: a new requirement for a UTF-8 file that is open in another encoding.
- `editor/new-project`: the new project's `qb64rust.toml` holds `encoding = "utf-8"`, and the folder gets a
  workspace setting so that VS Code saves its files as UTF-8.

## Impact

- `crates\base`: the code page 437 table with its reverse lookup, and each file's default encoding in
  `SourceFile`.
- `crates\syntax`: the `'$ENCODING` line; a byte order mark at the start of a file.
- `crates\sema`: the conversion where a string literal's or `DATA` item's bytes are taken, the two errors, and
  the lint.
- `crates\driver`: `settings.rs` (the `[project]` key), the loaders in `lib.rs` (a file's default), `main.rs`
  (the `convert` command).
- `crates\lsp`: the same default per file in `analysis.rs`, and the warning in `server.rs`.
- `vscode`: `newProject`, and `diagnostics.ts`, which reads a build's byte column as a character column.
- Documents: `DECISIONS.md`, `DIVERGENCES-QB64PE.md` (a declared file is read differently from QB64pe),
  `docs\MANUAL.md`, the reference chapters `pipeline.md` and `lints.md`, a new reference page for
  `$ENCODING`, and two `SOMEDAY.md` rows.
- No change to libqb, to the code generators, or to any recorded output.
