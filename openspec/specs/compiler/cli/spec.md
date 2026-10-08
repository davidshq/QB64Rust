# compiler/cli Specification

## Purpose
The `qb64rust` command line: compatible with the parts of qb64pe's command line that the corpus runner and the
M1 extension use, plus options for inspecting each compiler stage.

## Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] [-f:<setting>=<value>]... <file.bas> [-o <exe>]` SHALL compile the file to the
executable (default: the source name with `.exe`, next to the source). `-x`, `-q`, `-m` and `-w` SHALL be accepted;
`-q` SHALL suppress progress output. `-f:OptimizeCppProgram=true|false` SHALL build the C++ with or without
optimisation as `qb64pe` does; `-f:StripDebugSymbols=<value>` SHALL be accepted and ignored; any other setting
SHALL be an error "setting not supported yet". Exit status SHALL be 0 when the executable was built and 1
otherwise; on failure no executable SHALL be left at the output path.

#### Scenario: Runner's compile line
- **WHEN** the corpus runner runs `qb64rust -q -m -x <prog>.bas -o <prog>.exe` on a slice program
- **THEN** `<prog>.exe` exists and the exit status is 0

#### Scenario: Program with an error
- **WHEN** the source has an error
- **THEN** the exit status is 1, the diagnostics are printed, and `<prog>.exe` does not exist

#### Scenario: Compile suite's settings
- **WHEN** the upstream compile suite runs `qb64rust -f:OptimizeCppProgram=true -f:StripDebugSymbols=false -q -m -x
  <prog>.bas -o <exe>` on a program in `tests/upstream/pass.list`
- **THEN** the executable is built and the exit status is 0

#### Scenario: Unknown setting
- **WHEN** `-f:GenerateLicenseFile=true` is given
- **THEN** the exit status is 1 with the message that the setting is not supported yet, and no executable exists

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column; a diagnostic marked "not supported yet" SHALL be printed as
`<file>:<line>:<column>: error: not supported yet: <message>`. A summary line with the error count SHALL follow,
saying how many of the errors are marked.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error: not supported yet:`

#### Scenario: Summary
- **WHEN** a program has three errors, two of them marked
- **THEN** the last line says `3 errors (2 not supported yet)`

### Requirement: Generate C++ only
`-z` SHALL stop after writing the C++ fragments to the build folder and print the folder's path.

#### Scenario: -z on a slice program
- **WHEN** `qb64rust -z p.bas` is run
- **THEN** the build folder holds `main0.txt` and the other fragments, and no executable is produced

### Requirement: Stage dumps
`--dump tokens|tree|typed|ir|cpp` SHALL print the named stage's output for the file to stdout and exit without
building, with status 0 if there were no errors and 1 otherwise.

#### Scenario: Typed dump
- **WHEN** `qb64rust --dump typed p.bas` is run on a valid program
- **THEN** each expression node is printed with its type, and the exit status is 0

### Requirement: Locating the reference runtime
The QB64pe reference clone SHALL be taken from `--qb64pe-root`, else from the environment variable
`QB64RUST_QB64PE_ROOT`, else from `..\QB64pe` next to the repository the binary was built from. If none exists,
the compiler SHALL fail with a message naming the option and the variable.

#### Scenario: No clone found
- **WHEN** no clone exists at any of the three places
- **THEN** compilation fails with exit status 1 and a message naming `--qb64pe-root` and `QB64RUST_QB64PE_ROOT`

### Requirement: Internal compiler error
When the compiler panics, it SHALL print `qb64rust: internal compiler error:` with the panic message, its source
location and the input file, and SHALL exit with code 3, which no other failure uses. It SHALL NOT leave an
executable behind.

#### Scenario: Forced panic
- **WHEN** `qb64rust` runs with `QB64RUST_TEST_PANIC=1`
- **THEN** it prints a line starting with `qb64rust: internal compiler error:` and exits with code 3

### Requirement: Compiler root for included files
The compiler root, against which an included file is looked up when it is not in the including file's folder,
SHALL be the folder holding the `qb64rust` executable, unless `--include-root <dir>` names another; a relative
`<dir>` is resolved once, against the working directory at start. Included files SHALL NOT be looked up relative
to the working directory itself (the old compiler changes to its own folder at start).

#### Scenario: Default root
- **WHEN** `qb64rust` runs without `--include-root`
- **THEN** included files not found next to the including file are looked up relative to the executable's folder

### Requirement: Language server subcommand
`qb64rust lsp` SHALL run the language server (spec `editor/language-server`) over stdin and stdout; no other
option applies to it. Every other invocation is unchanged.

#### Scenario: Subcommand
- **WHEN** `qb64rust lsp` is started with a client on its pipes
- **THEN** it answers `initialize` and exits with code 0 after `shutdown` and `exit`

#### Scenario: Not a file name
- **WHEN** `qb64rust lsp.bas` is run
- **THEN** `lsp.bas` is compiled as a file; only the exact word `lsp` as the first argument starts the server
