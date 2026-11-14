# Spec Delta

## MODIFIED Requirements

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
