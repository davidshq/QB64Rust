# Spec Delta

## ADDED Requirements

### Requirement: Language server subcommand
`qb64rust lsp` SHALL run the language server (spec `editor/language-server`) over stdin and stdout; no other
option applies to it. Every other invocation is unchanged.

#### Scenario: Subcommand
- **WHEN** `qb64rust lsp` is started with a client on its pipes
- **THEN** it answers `initialize` and exits with code 0 after `shutdown` and `exit`

#### Scenario: Not a file name
- **WHEN** `qb64rust lsp.bas` is run
- **THEN** `lsp.bas` is compiled as a file; only the exact word `lsp` as the first argument starts the server
