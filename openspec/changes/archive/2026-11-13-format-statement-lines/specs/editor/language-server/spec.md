# Spec Delta

## ADDED Requirements

### Requirement: Code actions for statement lines
The server SHALL answer `textDocument/codeAction` with up to two actions of kind `refactor.rewrite`: "Put each
statement on its own line" and "Convert to block IF". Each SHALL carry the edits that formatting gives for the
lines the request's range touches with that one rewrite on (spec `compiler/format`), whatever the settings
say. An action SHALL be offered only when its rewrite changes one of those lines. Lines outside the range
SHALL NOT change.

#### Scenario: A packed line
- **WHEN** the cursor is on `x = 0: y = 0` and the client asks for code actions
- **THEN** "Put each statement on its own line" is offered, its edit replaces the line with `x = 0` and
  `y = 0`, and "Convert to block IF" is not offered

#### Scenario: A single-line IF with two statements
- **WHEN** the cursor is on `IF a THEN b = 1: x = 2`
- **THEN** "Convert to block IF" is offered, with the edit `If a Then`, `    b = 1: x = 2` and `End If`, and
  "Put each statement on its own line" is not offered

#### Scenario: Nothing to rewrite
- **WHEN** the cursor is on `x = 0`
- **THEN** the answer holds neither action

#### Scenario: A syntax error
- **WHEN** the document has a syntax error
- **THEN** the answer holds neither action, and no message is sent

#### Scenario: An error of a later stage
- **WHEN** the document holds `call missing: x = 1`, no SUB `missing` is defined, and the user picks "Put each
  statement on its own line"
- **THEN** the document does not change, and a `window/showMessage` error names the first error's file and
  line

#### Scenario: Layout turned off for the project
- **WHEN** `qb64rust.toml` holds `[format]` and `auto-layout = false`, and the user picks "Convert to block IF"
  on `IF a THEN x = 1`
- **THEN** the document does not change, and a `window/showMessage` says that the rewrite needs `AutoIndent`
  and `AutoLayout`

#### Scenario: A selection of several lines
- **WHEN** the range covers three lines, of which two are packed
- **THEN** the edit of "Put each statement on its own line" rewrites those two, and the lines outside the
  range are unchanged

### Requirement: Warning for a rewrite setting without effect
When a formatting request finds a rewrite setting on and `AutoIndent` or `AutoLayout` off for the document's
program (spec `compiler/format`, "A rewrite setting without effect"), the server SHALL answer with the edits
and SHALL send a `window/showMessage` warning with that text. It SHALL send it once, and again only after the
client's format settings or the settings file change.

#### Scenario: Format on save
- **WHEN** the client's `format.blockIf` is true and `format.autoLayout` is false, and the document is formatted
  three times
- **THEN** the first answer comes with a warning that `BlockIf` has no effect while `AutoLayout` is off, and
  the next two with none

#### Scenario: After a settings change
- **WHEN** the client then sends `workspace/didChangeConfiguration` with `format.autoIndent` false and asks to
  format
- **THEN** the warning is sent again

## MODIFIED Requirements

### Requirement: Server start
`qb64rust lsp` SHALL run a Language Server Protocol server over stdin and stdout until `shutdown` and `exit`, and
SHALL advertise exactly these capabilities: full text synchronisation, document symbols, folding ranges, go to
definition, document formatting and range formatting, hover, references, rename with prepare, completion, and code
actions of kind `refactor.rewrite` with resolve.

#### Scenario: Initialize and exit
- **WHEN** a client sends `initialize`, `initialized`, `shutdown` and `exit`
- **THEN** the server answers the two requests and the process ends with exit code 0

### Requirement: Format settings from the client
The layout settings SHALL be taken from the client's `initializationOptions` under `format` (`autoIndent`,
`indentSize`, `indentSubs`, `autoLayout`, `keywordCase` of `camel`, `upper` or `lower`, `oneStatementPerLine`,
`blockIf`), and from `workspace/didChangeConfiguration` under the same name. A setting not given SHALL take the
default of spec `compiler/format`. The request's own `FormattingOptions` (tab size, spaces) SHALL be ignored,
since the old compiler's layout has its own indent size. The `[format]` table of the `qb64rust.toml` found from
the main file of the document's program SHALL override the client's settings, key by key (spec
`compiler/format`, "Format table of the settings file").

#### Scenario: Upper case keywords
- **WHEN** the client started the server with `format.keywordCase` `upper` and asks to format `print 1`
- **THEN** the answer reads `PRINT 1`

#### Scenario: Setting changed
- **WHEN** the client sends `workspace/didChangeConfiguration` with `format.indentSize` 2 and then asks to format
- **THEN** the formatted text is indented two spaces a level

#### Scenario: The project's file over the client
- **WHEN** the client's `format.keywordCase` is `camel` and `format.indentSize` is 2, `qb64rust.toml` beside the
  open `p.bas` holds `[format]` and `keyword-case = "upper"`, and the client asks to format `p.bas`
- **THEN** keywords are written in capitals and blocks are indented two spaces a level

#### Scenario: An include file takes its program's file
- **WHEN** `proj\qb64rust.toml` holds `keyword-case = "upper"`, `proj\game.bas` includes `proj\lib\turn.bm`, and
  the client asks to format `turn.bm`
- **THEN** its keywords are written in capitals

#### Scenario: One statement per line from the client
- **WHEN** the client started the server with `format.oneStatementPerLine` true and asks to format
  `x = 0: y = 0`
- **THEN** the answer reads `x = 0` and `y = 0`

#### Scenario: The project's file turns a rewrite off
- **WHEN** the client's `format.blockIf` is true, `qb64rust.toml` holds `[format]` and `block-if = false`, and
  the client asks to format `IF a THEN x = 1`
- **THEN** the answer reads `If a Then x = 1`

### Requirement: Range formatting
The server SHALL answer `textDocument/rangeFormatting` by formatting the whole program and returning edits only
for the lines that the range touches, so that indentation and the spelling of names are those of the whole
program. Lines outside the range SHALL NOT change. A line that a rewrite setting (spec `compiler/format`,
"Rewrite settings") turns into several lines SHALL be replaced by all of them.

#### Scenario: One line of a block
- **WHEN** the range covers only the line `print   i` inside a `FOR` loop
- **THEN** the answer replaces that line with `    Print i`, and the `FOR` and `NEXT` lines are unchanged

#### Scenario: One packed line of several
- **WHEN** `format.oneStatementPerLine` is true, lines 2 and 5 are both `x = 0: y = 0`, and the range covers
  only line 2
- **THEN** the answer replaces line 2 with `x = 0` and `y = 0`, and line 5 is unchanged
