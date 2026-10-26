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
executable as `qb64pe` does, naming the licences of the parts of libqb the program links; the layout settings of
spec `compiler/format` SHALL be checked as `-y` checks them and SHALL NOT change the build, as with `qb64pe`; any
other setting SHALL be an error "setting not supported yet". Exit status SHALL be 0 when the executable was built
and 1 otherwise; on failure no executable SHALL be left at the output path.

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

#### Scenario: A layout setting in a build
- **WHEN** a program is built with `-f:AutoIndent=false`
- **THEN** the executable is built as without the setting, and the exit status is 0

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
