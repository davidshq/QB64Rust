# editor/formatting Specification

## Purpose
Formats QB64 source the way the QB64pe IDE lays it out (indentation, spacing, keyword case) by delegating to the
compiler's formatting mode.

## Requirements

### Requirement: Format document
The extension SHALL format a whole `qb64rust` document with the formatter that the next requirement chooses,
which lays it out as `qb64pe -y` does. The current editor contents, including unsaved changes, SHALL be
formatted. The file on disk SHALL NOT be modified by formatting.

#### Scenario: Indent and keyword case
- **WHEN** the document is `for i=1 to 3` / `print   i` / `next` and the user runs Format Document
- **THEN** the document becomes `For i = 1 To 3` / `    Print i` / `Next`

#### Scenario: Unsaved edits
- **WHEN** the document has unsaved changes and the user runs Format Document
- **THEN** the formatted result is based on the unsaved text and the document stays dirty

#### Scenario: Relative includes
- **WHEN** a saved `main.bas` contains `'$INCLUDE:'inc.bi'` and `inc.bi` is in the same folder
- **THEN** formatting succeeds (includes resolve relative to the document's folder, as in a build)

#### Scenario: Temporary files are removed
- **WHEN** formatting by a compiler's `-y` finishes, successfully or not
- **THEN** no temporary file created for formatting remains in the document's folder

### Requirement: Format keeps line endings and encoding
The formatted text SHALL use the document's existing end-of-line sequence and SHALL be read back with the same
encoding it was written with, so that CP437 bytes in string literals survive.

#### Scenario: LF document
- **WHEN** the document uses LF line endings and the compiler writes CRLF
- **THEN** the formatted document still uses LF

#### Scenario: Extended characters
- **WHEN** a string literal contains `╔══╗` in a CP437 document
- **THEN** the same characters are present after formatting

### Requirement: Format failure leaves the document unchanged
If the formatter reports an error, the document SHALL NOT change, the error SHALL be shown as a message, and
diagnostics SHALL be updated as for a check.

#### Scenario: Syntax error
- **WHEN** the document contains `x = 1 +` and the user runs Format Document
- **THEN** the document is unchanged and an error message names line 2

### Requirement: Format on save is opt-in
Formatting SHALL run on save only through VS Code's own `editor.formatOnSave` setting; the extension SHALL NOT turn
it on by default.

#### Scenario: Default
- **WHEN** a user installs the extension and saves a file
- **THEN** the file is not reformatted

### Requirement: Which formatter
When the language server runs, the extension SHALL format through it (spec `editor/language-server`,
"Formatting"). Otherwise it SHALL run `qb64rust -y` when the `qb64rust` binary is found and `qb64rust.buildWith`
is not `qb64pe`. It SHALL run `qb64pe -y` only when `qb64rust.buildWith` is `qb64pe`, or when the binary is not
found. It SHALL NOT fall back from one formatter to another after a failure.

#### Scenario: Server running
- **WHEN** the language server runs and the user runs Format Document
- **THEN** the edits come from the server, and no compiler process is started and no temporary file is written

#### Scenario: Old compiler chosen
- **WHEN** the server is off and `qb64rust.buildWith` is `qb64pe`
- **THEN** the document is formatted by `qb64pe -y` with the extension's layout settings given as `-f:` settings

#### Scenario: Error is not retried
- **WHEN** `qb64rust -y` reports an error
- **THEN** the document is unchanged and `qb64pe -y` is not run

### Requirement: Layout settings
The settings `qb64rust.format.autoIndent` (default true), `qb64rust.format.indentSize` (1 to 64, default 4),
`qb64rust.format.indentSubs` (default true), `qb64rust.format.autoLayout` (default true) and
`qb64rust.format.keywordCase` (`camel`, the default, `upper` or `lower`) SHALL be passed to whichever formatter
runs: to the server as its format settings, to `qb64rust -y` as `--format-default` settings, and to `qb64pe -y`
as its `-f:` layout settings. The result SHALL NOT depend on QB64pe's `config.ini`. The `[format]` table of the
program's `qb64rust.toml` SHALL override these settings when the server or `qb64rust -y` formats; `qb64pe -y`
does not read that file. Each setting's description SHALL say that a project's `qb64rust.toml` overrides it.

#### Scenario: Lower case keywords
- **WHEN** `qb64rust.format.keywordCase` is `lower` and the user formats `PRINT 1`
- **THEN** the document becomes `print 1`, whichever formatter runs

#### Scenario: Setting changed while the server runs
- **WHEN** the user changes `qb64rust.format.indentSize` to 2 and formats again
- **THEN** the document is indented two spaces a level, without restarting the server

#### Scenario: The project's file wins
- **WHEN** `qb64rust.format.keywordCase` is `camel`, `qb64rust.toml` in the workspace folder holds `[format]` and
  `keyword-case = "upper"`, and the user formats `print 1`
- **THEN** the document becomes `PRINT 1`, with the server on and with the server off

#### Scenario: A bad settings file
- **WHEN** that file's line reads `keyword-case = "uper"` and the user runs Format Document
- **THEN** the document is unchanged and an error message names `qb64rust.toml`

### Requirement: Format selection
The extension SHALL provide Format Selection for `qb64rust` documents. It SHALL change only the lines that the
selection touches, laid out as in the whole formatted document. When the server is off, it SHALL format the whole
text with the chosen compiler's `-y` and keep only those lines; `qb64rust -y` SHALL then be run with
`OneStatementPerLine` and `BlockIf` false, so that each line of the document has one line of the formatted
text.

#### Scenario: One line
- **WHEN** the user selects the line `print   i` inside a `FOR` loop and runs Format Selection
- **THEN** only that line changes, to `    Print i`

#### Scenario: A packed line with the server on
- **WHEN** `qb64rust.format.oneStatementPerLine` is true, the server runs, and the user selects the line
  `x = 0: y = 0` and runs Format Selection
- **THEN** that line becomes `x = 0` and `y = 0`, and no other line changes

#### Scenario: A packed line with the server off
- **WHEN** the same setting is true, the server is off, and the user selects the line `x=0:y=0` and runs Format
  Selection
- **THEN** that line becomes `x = 0: y = 0`

### Requirement: Format an included file
Format Document and Format Selection SHALL work on a `qb64rust` document that is an included file (`.bi`, `.bm`),
whether or not the program that includes it is open or found. When the language server runs, the result SHALL be
the server's (spec `editor/language-server`, "Formatting an included file"). An error that is not a syntax error
SHALL NOT stop formatting and SHALL NOT show a message.

#### Scenario: Include file of a program on disk
- **WHEN** `turn.bm` is open, `CWSTRAT.BAS` beside it includes it and is not open, and the user runs Format
  Document
- **THEN** `turn.bm` is formatted, with each name in the letter case of its declaration in the program

#### Scenario: Include file with no program
- **WHEN** `lib.bm` calls a SUB that no file defines, no program includes it, and the user runs Format Document
- **THEN** `lib.bm` is indented and spaced, its keywords are in the keyword case, and no error message is shown

### Requirement: Rewrite settings in the extension
The extension SHALL have `qb64rust.format.oneStatementPerLine` and `qb64rust.format.blockIf`, both false by
default. They SHALL be passed to the server as its format settings and to `qb64rust -y` as `--format-default`
settings. They SHALL NOT be passed to `qb64pe -y`, which has no such settings. Each description SHALL say that
a project's `qb64rust.toml` overrides it and that the old compiler's formatter ignores it.

#### Scenario: Format Document with the server
- **WHEN** `qb64rust.format.oneStatementPerLine` is true and the user formats `x = 0: y = 0`
- **THEN** the document becomes the two lines `x = 0` and `y = 0`

#### Scenario: The old compiler's formatter
- **WHEN** `qb64rust.format.blockIf` is true, `qb64rust.buildWith` is `qb64pe`, the server is off, and the user
  formats `IF a THEN x = 1`
- **THEN** `qb64pe -y` is run without a `BlockIf` setting, and the document becomes `If a Then x = 1`

#### Scenario: The project's file wins
- **WHEN** `qb64rust.format.blockIf` is false, `qb64rust.toml` in the workspace folder holds `[format]` and
  `block-if = true`, and the user formats `IF a THEN x = 1`
- **THEN** the document becomes `If a Then`, `    x = 1` and `End If`, with the server on and with the server
  off
