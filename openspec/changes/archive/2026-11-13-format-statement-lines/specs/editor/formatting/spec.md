# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

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
