# Spec Delta

## MODIFIED Requirements

### Requirement: Server start
`qb64rust lsp` SHALL run a Language Server Protocol server over stdin and stdout until `shutdown` and `exit`, and
SHALL advertise exactly these capabilities: full text synchronisation, document symbols, folding ranges, go to
definition, document formatting and range formatting.

#### Scenario: Initialize and exit
- **WHEN** a client sends `initialize`, `initialized`, `shutdown` and `exit`
- **THEN** the server answers the two requests and the process ends with exit code 0

## ADDED Requirements

### Requirement: Formatting
The server SHALL answer `textDocument/formatting` for an open document by formatting its program as `qb64rust -y`
does (spec `compiler/format`). The program SHALL be read as a check reads it: open documents first, then the disk.
The answer SHALL be one edit that replaces the whole document, or no edit when nothing changes. No file SHALL be
written.

#### Scenario: Unsaved text
- **WHEN** an open document holds the unsaved line `for i=1 to 3:next` and the client asks to format it
- **THEN** the answer replaces it with `For i = 1 To 3: Next`, and nothing is written to disk

#### Scenario: Unsaved include
- **WHEN** an open, unsaved `inc.bi` declares `DIM SHARED total AS LONG` and the including document writes `TOTAL = 1`
- **THEN** the formatted document reads `total = 1`

#### Scenario: Already formatted
- **WHEN** the document is already formatted
- **THEN** the answer is an empty list of edits

### Requirement: Format settings from the client
The layout settings SHALL be taken from the client's `initializationOptions` under `format` (`autoIndent`,
`indentSize`, `indentSubs`, `autoLayout`, `keywordCase` of `camel`, `upper` or `lower`), and from
`workspace/didChangeConfiguration` under the same name. A setting not given SHALL take the default of spec
`compiler/format`. The request's own `FormattingOptions` (tab size, spaces) SHALL be ignored, since the old
compiler's layout has its own indent size.

#### Scenario: Upper case keywords
- **WHEN** the client started the server with `format.keywordCase` `upper` and asks to format `print 1`
- **THEN** the answer reads `PRINT 1`

#### Scenario: Setting changed
- **WHEN** the client sends `workspace/didChangeConfiguration` with `format.indentSize` 2 and then asks to format
- **THEN** the formatted text is indented two spaces a level

### Requirement: Format failure
When the program has an error, the server SHALL answer the formatting request with no edits, SHALL publish the
diagnostics as a check publishes them, and SHALL send a `window/showMessage` error that names the first error's
file and line.

#### Scenario: Syntax error
- **WHEN** the document's line 2 is `x = 1 +` and the client asks to format it
- **THEN** the answer is an empty list of edits, an error on line 2 is published, and a message names line 2

### Requirement: Range formatting
The server SHALL answer `textDocument/rangeFormatting` by formatting the whole program and returning edits only
for the lines that the range touches, so that indentation and the spelling of names are those of the whole
program. Lines outside the range SHALL NOT change.

#### Scenario: One line of a block
- **WHEN** the range covers only the line `print   i` inside a `FOR` loop
- **THEN** the answer replaces that line with `    Print i`, and the `FOR` and `NEXT` lines are unchanged
