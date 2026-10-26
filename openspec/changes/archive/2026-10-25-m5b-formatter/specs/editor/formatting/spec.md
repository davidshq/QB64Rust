# Spec Delta

## MODIFIED Requirements

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

### Requirement: Format failure leaves the document unchanged
If the formatter reports an error, the document SHALL NOT change, the error SHALL be shown as a message, and
diagnostics SHALL be updated as for a check.

#### Scenario: Syntax error
- **WHEN** the document contains `x = 1 +` and the user runs Format Document
- **THEN** the document is unchanged and an error message names line 2

## ADDED Requirements

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
runs: to the server as its format settings, and to either compiler as its `-f:` layout settings. The result SHALL
NOT depend on QB64pe's `config.ini`.

#### Scenario: Lower case keywords
- **WHEN** `qb64rust.format.keywordCase` is `lower` and the user formats `PRINT 1`
- **THEN** the document becomes `print 1`, whichever formatter runs

#### Scenario: Setting changed while the server runs
- **WHEN** the user changes `qb64rust.format.indentSize` to 2 and formats again
- **THEN** the document is indented two spaces a level, without restarting the server

### Requirement: Format selection
The extension SHALL provide Format Selection for `qb64rust` documents. It SHALL change only the lines that the
selection touches, laid out as in the whole formatted document. When the server is off, it SHALL format the whole
text with the chosen compiler's `-y` and keep only those lines.

#### Scenario: One line
- **WHEN** the user selects the line `print   i` inside a `FOR` loop and runs Format Selection
- **THEN** only that line changes, to `    Print i`
