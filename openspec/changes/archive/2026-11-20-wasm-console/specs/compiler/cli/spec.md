# Spec Delta

## ADDED Requirements

### Requirement: The target option
`--target <name>` SHALL choose what a build, a check (`-z`) and a dump are made for. The names are `wasm` and
`host`. Without the option the target is the host: the platform `qb64rust` runs on. Any other name SHALL be a
usage error with status 1 that lists the names. The usage text SHALL name the option and say that `wasm` is
experimental.

#### Scenario: An unknown name
- **WHEN** `qb64rust -x --target dos p.bas` runs
- **THEN** the exit status is 1, nothing is built, and the message lists `wasm` and `host`

#### Scenario: The default
- **WHEN** `qb64rust -x p.bas` and `qb64rust -x --target host p.bas` run on Windows
- **THEN** both write the same `p.exe`

### Requirement: The output of a wasm build
With `--target wasm` the default output SHALL be the source's name with `.js`, next to the source, and the
`.wasm` file SHALL have the same name beside it. `-o <file>` SHALL name the `.js` file and SHALL end in
`.js`; another ending is an error with status 1. On failure neither file SHALL be left at the output path.

#### Scenario: A named output
- **WHEN** `qb64rust -x --target wasm p.bas -o out/app.js` builds
- **THEN** `out/app.js` and `out/app.wasm` exist

#### Scenario: A wrong ending
- **WHEN** `qb64rust -x --target wasm p.bas -o p.exe` runs
- **THEN** the exit status is 1 and the message says the output of a wasm build ends in `.js`

#### Scenario: A failed build
- **WHEN** the C++ build of a wasm program fails and `p.js` and `p.wasm` of an earlier build exist
- **THEN** both are gone after the command, as an executable is after a failed build

### Requirement: Options that a wasm build does not take
With `--target wasm`, the debug build, the coverage build, the value-trace build, the unit-test run and
`qb64rust dap` SHALL be refused with status 1 and a message that names the option and the target. `$EXEICON`
and `$VERSIONINFO` SHALL be treated as on Linux.

#### Scenario: A debug build
- **WHEN** a debug build is asked for with `--target wasm`
- **THEN** the exit status is 1, nothing is built, and the message says debug builds are not available on the
  wasm target
