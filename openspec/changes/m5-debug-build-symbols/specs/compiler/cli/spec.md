# Spec Delta

## MODIFIED Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] [-f:<setting>=<value>]... <file.bas> [-o <exe>]` SHALL compile the file to the
executable (default: the source name with `.exe`, next to the source). `-x`, `-q`, `-m` and `-w` SHALL be accepted;
`-q` SHALL suppress progress output. `-f:OptimizeCppProgram=true|false` SHALL build the C++ with or without
optimisation as `qb64pe` does; `-f:ExtraLinkerFlags=<value>` SHALL add the value to the end of the link command as
`qb64pe` does; `-f:StripDebugSymbols=true|false` SHALL strip or not strip the executable's symbols as `qb64pe`
does (true, the default, strips; any other value is an error as for the other boolean settings);
`-f:GenerateLicenseFile=true|false` SHALL, when true, write the licence file `<exe>.license.txt` beside the
executable as `qb64pe` does, naming the licences of the parts of libqb the program links; any other setting SHALL
be an error "setting not supported yet". Exit status SHALL be 0 when the executable was built and 1 otherwise; on
failure no executable SHALL be left at the output path.

#### Scenario: Runner's compile line
- **WHEN** the corpus runner runs `qb64rust -q -m -x <prog>.bas -o <prog>.exe` on a slice program
- **THEN** `<prog>.exe` exists and the exit status is 0

#### Scenario: Program with an error
- **WHEN** the source has an error
- **THEN** the exit status is 1, the diagnostics are printed, and `<prog>.exe` does not exist

#### Scenario: Compile suite's settings
- **WHEN** the upstream compile suite runs `qb64rust -f:OptimizeCppProgram=true -f:StripDebugSymbols=false -q -m -x
  <prog>.bas -o <exe>` on a program in `tests/upstream/pass.list`
- **THEN** the executable is built unstripped, prints what it printed before, and the exit status is 0

#### Scenario: Unstripped on request
- **WHEN** a program is built with `-f:StripDebugSymbols=false` and once without the setting
- **THEN** the first executable keeps its local symbols (`nm` lists symbols of type `t`) and the second has none;
  both keep their global ones, as the `Makefile`'s `objcopy --strip-unneeded` leaves them (measured at task 1.3)

#### Scenario: Unknown setting
- **WHEN** `-f:MaxCompilerProcesses=2` is given
- **THEN** the exit status is 1 with the message that the setting is not supported yet, and no executable exists

#### Scenario: The licence file
- **WHEN** `-f:GenerateLicenseFile=true` is given for a program that names `_INFLATE$`
- **THEN** `<exe>.license.txt` exists and equals, but for line ends, the file the old compiler writes for it

#### Scenario: No licence file unless asked
- **WHEN** a program is built without the setting, or with `-f:GenerateLicenseFile=false`
- **THEN** no `<exe>.license.txt` is written, and one an earlier build left at that path stays, as with the old
  compiler

#### Scenario: An extra source file on the link line
- **WHEN** `-f:ExtraLinkerFlags=<path>/screen_state.cpp` is given and the program runs with `QB64_SCREEN_STATE`
  naming a file
- **THEN** the executable is built with that file linked in, and the run writes the state file

#### Scenario: The extra file changes
- **WHEN** a build cache is in use and the file named by `-f:ExtraLinkerFlags` has changed since the cached build
- **THEN** the program is built again

### Requirement: Stage dumps
`--dump tokens|tree|typed|ir|cpp|debug-info` SHALL print the named stage's output for the file to stdout and exit
without building, with status 0 if there were no errors and 1 otherwise. `debug-info` prints the debug symbol file
(spec `compiler/debug-info`).

#### Scenario: Typed dump
- **WHEN** `qb64rust --dump typed p.bas` is run on a valid program
- **THEN** each expression node is printed with its type, and the exit status is 0

#### Scenario: Debug-info dump
- **WHEN** `qb64rust --dump debug-info p.bas` is run on a valid program
- **THEN** the debug symbol file's JSON is printed, and the exit status is 0

## ADDED Requirements

### Requirement: Debug build
`qb64rust --debug` with a compile line SHALL build a program for a debugger: its own objects compiled with debug
information (`-g`) and without optimisation, the executable left unstripped, and the debug symbol file written
beside it (spec `compiler/debug-info`). `OptimizeCppProgram=true` given with it SHALL be overridden, with a line
on the console unless `-q`.

#### Scenario: Optimisation overridden
- **WHEN** `--debug -f:OptimizeCppProgram=true` is given
- **THEN** the program's objects are compiled without `-O2`, and the console says the optimisation was left out

#### Scenario: The build cache
- **WHEN** a build cache is in use and a program built without `--debug` is built with it
- **THEN** the program is built again, and both builds are kept apart in the cache

### Requirement: A debug build is the same program
A program built with `--debug` SHALL behave as the build without it: the C++ written SHALL be the same, so its
output and its screen are the same. A debugger reading the executable's debug information SHALL find each
statement at its BASIC file and line.

#### Scenario: Same program
- **WHEN** a slice program is built with `--debug` and run
- **THEN** its output equals the `.output` recorded from `qb64pe.exe`

#### Scenario: Line information
- **WHEN** a program is built with `--debug` and the toolchain's `lldb` sets a breakpoint at `p.bas` line 11
- **THEN** the breakpoint resolves to an address in the program's code, the program stops there when run, and the
  frame's source location is `p.bas:11`
