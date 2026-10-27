# Spec Delta

## MODIFIED Requirements

### Requirement: Locating the reference runtime
The QB64pe folder, relative to which `DECLARE LIBRARY` looks for files and the extra link flags name files, SHALL
be taken from `--qb64pe-root`, else from the environment variable `QB64RUST_QB64PE_ROOT`, else from `..\QB64pe`
next to the repository the binary was built from; when none of these exists, the include root takes its place,
and the build goes on. The C++ toolchain is found as spec `compiler/toolchain` says. The copy of libqb SHALL be
taken from `--libqb-root`, else from the environment variable `QB64RUST_LIBQB_ROOT`, else from `libqb` in the
repository the binary was built from; it is needed only to build an executable. If the copy is needed and not
found, the compiler SHALL fail with a message naming its option and its variable.

#### Scenario: No clone found
- **WHEN** no QB64pe folder exists at any of the three places, a toolchain is found, and a program without
  `DECLARE LIBRARY` is compiled to an executable
- **THEN** the executable is built

#### Scenario: A library without a clone
- **WHEN** no QB64pe folder exists, and a program at `p\prog.bas` compiled with `--include-root r` declares
  `DECLARE LIBRARY "lib/x"` with `r\lib\x.h` present
- **THEN** the header is found under the include root, as it would be under QB64pe's folder

#### Scenario: No copy of libqb found
- **WHEN** a program is compiled to an executable, a toolchain is found, and no copy of libqb exists at any of
  the three places
- **THEN** compilation fails with exit status 1 and a message naming `--libqb-root` and `QB64RUST_LIBQB_ROOT`

### Requirement: Debug adapter subcommand
`qb64rust dap` SHALL run the debug adapter (spec `editor/debug-adapter`) over stdin and stdout. The options it
takes SHALL be `--toolchain-root <dir>` and `--qb64pe-root <dir>`, which choose the toolchain whose `lldb-dap` it
starts as a build chooses it (spec `compiler/toolchain`). Every other invocation is unchanged.

#### Scenario: Subcommand
- **WHEN** `qb64rust dap` is started with a client on its pipes and a toolchain is found
- **THEN** it answers `initialize`, and exits with code 0 after `disconnect`

#### Scenario: Another option
- **WHEN** `qb64rust dap -x p.bas` is run
- **THEN** it prints that `dap` takes only `--toolchain-root` and `--qb64pe-root` and exits with status 1

#### Scenario: Not a file name
- **WHEN** `qb64rust dap.bas` is run
- **THEN** `dap.bas` is compiled as a file; only the exact word `dap` as the first argument starts the adapter

## ADDED Requirements

### Requirement: Toolchain subcommand
`qb64rust toolchain install` and `qb64rust toolchain path` SHALL behave as spec `compiler/toolchain` says. Only the
exact word `toolchain` as the first argument starts the subcommand; any other word after it SHALL print the two
forms and exit with status 1.

#### Scenario: Unknown action
- **WHEN** `qb64rust toolchain remove` is run
- **THEN** it prints the usage of `toolchain install` and `toolchain path` and exits with status 1

#### Scenario: A file named toolchain
- **WHEN** `qb64rust toolchain.bas` is run
- **THEN** `toolchain.bas` is compiled as a file
