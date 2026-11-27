# Spec Delta

## ADDED Requirements

### Requirement: Call-trace build
`qb64rust --call-trace` with a compile line SHALL build a program that adds a call stack to a runtime error's
message (spec `compiler/call-trace`). Without the call trace the C++ written SHALL be the same as before this
change. The option SHALL combine with `--coverage`, `--trace-values`, `--debug` and `--target wasm`.

#### Scenario: The option
- **WHEN** a program whose `SUB fail` runs `ERROR 5` is built with `--call-trace` and run with
  `QB64PE_NOPROMPT=y`
- **THEN** stderr holds a `Call stack:` block that names `SUB fail`

#### Scenario: The build without the option
- **WHEN** the same program is compiled with no option and no settings file
- **THEN** its C++ is byte for byte the C++ written before this change, and stderr holds no `Call stack:`

#### Scenario: With the other builds
- **WHEN** a program is built with `--call-trace --coverage --trace-values`
- **THEN** the coverage record and the value-trace record are those of the build without `--call-trace`

#### Scenario: A wasm build
- **WHEN** a `$CONSOLE:ONLY` program is built with `--target wasm --call-trace` and run under Node
- **THEN** its error message holds the same `Call stack:` block as the desktop build's

### Requirement: When a build has the call trace
`call_trace = true` in the `[build]` table of `qb64rust.toml` SHALL turn the call trace on for the project's
builds, `qb64rust test` among them. A `--debug` build SHALL have it unless it is turned off. `--no-call-trace`
SHALL turn it off. The command line SHALL go over the settings file, and the settings file over `--debug`.

#### Scenario: The settings file
- **WHEN** `qb64rust.toml` has `[build]` with `call_trace = true` and the program is built with no option
- **THEN** the program's error message holds the `Call stack:` block

#### Scenario: The command line over the settings file
- **WHEN** the same project is built with `--no-call-trace`
- **THEN** the program's error message holds no `Call stack:`

#### Scenario: A debug build
- **WHEN** the program is built with `--debug` and nothing else
- **THEN** its error message holds the `Call stack:` block; with `--debug --no-call-trace`, or with
  `call_trace = false` in the settings file, it does not

## MODIFIED Requirements

### Requirement: A debug build is the same program
A program built with `--debug` SHALL behave as the build without it, apart from the call trace (requirement
"Call-trace build"): its output and its screen are the same, and with `--no-call-trace` the C++ written SHALL be
the same. A debugger reading the executable's debug information SHALL find each statement at its BASIC file and
line.

#### Scenario: Same program
- **WHEN** a slice program is built with `--debug` and run
- **THEN** its output equals the `.output` recorded from `qb64pe.exe`, except that a runtime error's message
  also holds its `Call stack:` block

#### Scenario: The same C++ without the call trace
- **WHEN** a program is compiled with `--debug --no-call-trace` and with no option
- **THEN** both write the same C++

#### Scenario: Line information
- **WHEN** a program is built with `--debug` and the toolchain's `lldb` sets a breakpoint at `p.bas` line 11
- **THEN** the breakpoint resolves to an address in the program's code, the program stops there when run, and the
  frame's source location is `p.bas:11`

### Requirement: A bad build table
A `[build]` table SHALL be an error that stops the build, as a mistake in any other table does, when it has a
key other than `output`, `copy` and `call_trace`, when `output` is not a string or is empty, when `copy` is not
a list of strings, when `call_trace` is not `true` or `false`, or when a path has a `\`. A `copy` entry SHALL
NOT be or hold the output folder, so `copy` SHALL be an
error with `output = "."`. A `copy` entry SHALL be a path inside the settings file's folder: an absolute path or
one that goes above the folder with `..` is an error, since its copy would be outside the output folder.

#### Scenario: An unknown key
- **WHEN** `[build]` has `out = "build"`
- **THEN** the exit status is 1, and the message names the line and says the keys are `output`, `copy` and
  `call_trace`

#### Scenario: A call_trace that is not a switch
- **WHEN** `[build]` has `call_trace = "yes"`
- **THEN** the exit status is 1, and the message names the line and says the value is `true` or `false`

#### Scenario: The output folder inside a copied folder
- **WHEN** `output = "data/build"` and `copy = ["data"]`
- **THEN** the exit status is 1, and the message says that `data` holds the output folder

#### Scenario: The output folder reached another way
- **WHEN** `proj/qb64rust.toml` has `output = "../proj/data/out"`, or the same folder as an absolute path, and
  `copy = ["data"]`
- **THEN** the exit status is 1, and the message says that `data` holds the output folder: the paths are compared
  once joined to the settings file's folder

#### Scenario: A copy list with the settings file's folder as the output
- **WHEN** `output = "."` and `copy = ["GAME.CFG"]`
- **THEN** the exit status is 1, and the message says that the files are already in the output folder
