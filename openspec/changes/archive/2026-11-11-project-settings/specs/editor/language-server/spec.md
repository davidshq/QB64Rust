# Spec Delta

## MODIFIED Requirements

### Requirement: Format settings from the client
The layout settings SHALL be taken from the client's `initializationOptions` under `format` (`autoIndent`,
`indentSize`, `indentSubs`, `autoLayout`, `keywordCase` of `camel`, `upper` or `lower`), and from
`workspace/didChangeConfiguration` under the same name. A setting not given SHALL take the default of spec
`compiler/format`. The request's own `FormattingOptions` (tab size, spaces) SHALL be ignored, since the old
compiler's layout has its own indent size. The `[format]` table of the `qb64rust.toml` found from the main file
of the document's program SHALL override the client's settings, key by key (spec `compiler/format`, "Format
table of the settings file").

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

### Requirement: Format failure
When the parser reports an error in the program, the server SHALL answer the formatting request with no edits,
SHALL publish the diagnostics as a check publishes them, and SHALL send a `window/showMessage` error that names
the first error's file and line. An error of a later stage SHALL NOT stop formatting (spec `compiler/format`,
"Program with an error"): the server SHALL answer with the edits, publish nothing and send no message. A
`qb64rust.toml` that cannot be used SHALL stop formatting: the server SHALL answer with no edits and send a
`window/showMessage` error that names `qb64rust.toml` and its line.

#### Scenario: Syntax error
- **WHEN** the document's line 2 is `x = 1 +` and the client asks to format it
- **THEN** the answer is an empty list of edits, an error on line 2 is published, and a message names line 2

#### Scenario: A SUB that is not defined
- **WHEN** the document's only line is `call   missing` and the client asks to format it
- **THEN** the answer replaces the line with `Call missing`, and no message is sent

#### Scenario: A bad settings file
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[format]` and `keyword-case = "uper"`, and the client asks to
  format `p.bas`
- **THEN** the answer is an empty list of edits, and a message names `qb64rust.toml` and its line 2

## ADDED Requirements

### Requirement: Lints in a check
A check SHALL publish the warnings of the lints that are on for the program, as the `[lints]` table of its
`qb64rust.toml` says (spec `compiler/lints`). A document that is not a file SHALL be checked with every lint on.

#### Scenario: A lint turned off
- **WHEN** `qb64rust.toml` beside the open `p.bas` holds `[lints]` and `unused-label = false`, and `p.bas` has an
  unused label
- **THEN** a check publishes no `unused-label` warning

#### Scenario: The table changes on disk
- **WHEN** a check shows an `unused-label` warning, and `unused-label = false` is added to `[lints]` on disk
- **THEN** the warning is no longer published, with no edit of `p.bas`
