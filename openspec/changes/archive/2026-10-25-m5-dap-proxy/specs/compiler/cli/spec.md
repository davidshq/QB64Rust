# Spec Delta

## ADDED Requirements

### Requirement: Debug adapter subcommand
`qb64rust dap` SHALL run the debug adapter (spec `editor/debug-adapter`) over stdin and stdout. The only option
it takes SHALL be `--qb64pe-root <dir>`, the reference clone whose `lldb-dap` it starts, found as a build finds
the clone when it is not given (`QB64RUST_QB64PE_ROOT`, then the default). Every other invocation is unchanged.

#### Scenario: Subcommand
- **WHEN** `qb64rust dap` is started with a client on its pipes and the clone is found
- **THEN** it answers `initialize`, and exits with code 0 after `disconnect`

#### Scenario: Another option
- **WHEN** `qb64rust dap -x p.bas` is run
- **THEN** it prints that `dap` takes only `--qb64pe-root` and exits with status 1

#### Scenario: Not a file name
- **WHEN** `qb64rust dap.bas` is run
- **THEN** `dap.bas` is compiled as a file; only the exact word `dap` as the first argument starts the adapter
