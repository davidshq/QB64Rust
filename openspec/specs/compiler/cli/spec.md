# compiler/cli Specification

## Purpose
The `qb64rust` command line: compatible with the parts of qb64pe's command line that the corpus runner and the
M1 extension use, plus options for inspecting each compiler stage.

## Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] [-f:<setting>=<value>]... <file.bas> [-o <exe>]` SHALL compile the file to the
executable (default: the source name with `.exe`, next to the source). `-x`, `-q`, `-m` and `-w` SHALL be accepted;
`-q` SHALL suppress progress output. `-f:OptimizeCppProgram=true|false` SHALL build the C++ with or without
optimisation as `qb64pe` does; `-f:ExtraLinkerFlags=<value>` SHALL add the value to the end of the link command as
`qb64pe` does; `-f:StripDebugSymbols=<value>` SHALL be accepted and ignored; `-f:GenerateLicenseFile=true|false`
SHALL, when true, write the licence file `<exe>.license.txt` beside the executable as `qb64pe` does, naming the
licences of the parts of libqb the program links; any other setting SHALL be an error
"setting not supported yet". Exit status SHALL be 0 when the executable was built and 1 otherwise; on failure no
executable SHALL be left at the output path.

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

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column; a diagnostic marked "not supported yet" SHALL be printed as
`<file>:<line>:<column>: error: not supported yet: <message>`. An error not so marked whose wording by the old
compiler is in the table made from recorded output SHALL be followed by `<file>:<line>:<column>: note: QB64pe:
<that wording>`, the names and numbers of this error filled in. A summary line with the error count SHALL follow,
saying how many of the errors are marked. A warning SHALL be printed as `<file>:<line>:<column>: warning:
<message>` only when `-w` is given, as `qb64pe` prints its warnings; warnings SHALL NOT be counted in the summary
and SHALL NOT change the exit status or stop the executable from being built.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error: not supported yet:`

#### Scenario: The old compiler's wording
- **WHEN** line 9 of `p.bas` is `m = _MEM()`
- **THEN** the error `p.bas:9:5: error: `_MEM` with empty parentheses` is followed by `p.bas:9:5: note: QB64pe:
  Expected (...)`

#### Scenario: A name in the old wording
- **WHEN** `OPTION _EXPLICIT` is set and line 2 is `count = 1`
- **THEN** the note reads `QB64pe: Variable 'count' (SINGLE) not defined`, with the name the error gives

#### Scenario: A form no recording shows
- **WHEN** line 3 is `count& = 1`, and every recorded pair of that message had a name without a type suffix
- **THEN** the error has no note (the old compiler drops the suffix there, which no recording showed)

#### Scenario: Summary
- **WHEN** a program has three errors, two of them marked
- **THEN** the last line says `3 errors (2 not supported yet)`

#### Scenario: Warning shown with -w
- **WHEN** `qb64rust -w -x p.bas` compiles a program whose line 2 is `CONST c = 2 ^ 3 ^ 2`
- **THEN** the output contains a line starting `p.bas:2:` with `warning:`, the exit status is 0, and the
  executable exists

#### Scenario: Warning hidden without -w
- **WHEN** the same program is compiled with `qb64rust -q -m -x p.bas`
- **THEN** no warning is printed and the executable exists

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
The QB64pe reference clone, whose toolchain builds the executable and relative to which `DECLARE LIBRARY` looks
for files, SHALL be taken from `--qb64pe-root`, else from the environment variable `QB64RUST_QB64PE_ROOT`, else
from `..\QB64pe` next to the repository the binary was built from. The copy of libqb SHALL be taken from
`--libqb-root`, else from the environment variable `QB64RUST_LIBQB_ROOT`, else from `libqb` in the repository the
binary was built from; it is needed only to build an executable. If either is needed and not found, the compiler
SHALL fail with a message naming its option and its variable.

#### Scenario: No clone found
- **WHEN** no clone exists at any of the three places
- **THEN** compilation fails with exit status 1 and a message naming `--qb64pe-root` and `QB64RUST_QB64PE_ROOT`

#### Scenario: No copy of libqb found
- **WHEN** a program is compiled to an executable, the clone is found, and no copy of libqb exists at any of the
  three places
- **THEN** compilation fails with exit status 1 and a message naming `--libqb-root` and `QB64RUST_LIBQB_ROOT`

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

### Requirement: Demand report
`qb64rust --report unsupported <file or folder>... [--include-root <dir>]` SHALL run the front end over every
program given (a file as given; of a folder every `.bas` file below it, in any letter case) and print what the set
still needs: a summary line with the number of programs, of those without errors, of those with only "not
supported yet" errors and of those with other errors; then one line per distinct "not supported yet" message with
the number of programs it occurs in, the number of programs in which it is the only message and no other error
occurs, and the number of times it occurs, ordered by those three numbers, largest first. The error cap of a
compile run SHALL NOT apply. A program with an `.err` file beside it SHALL be left out and counted in a line of its
own. A program with an error that is not marked SHALL be named after the table. Nothing SHALL be built. The exit
status SHALL be 0 when every input was read, whatever errors the programs hold, and 1 for an input that is neither
a file nor a folder or for any other word after `--report`. Without `--report`, more than one input file SHALL
stay an error. A program the inputs reach more than once, under whatever spelling of its path, SHALL be counted
once.

#### Scenario: A folder of programs
- **WHEN** a folder holds a program without errors, a program with one `LPRINT`, a program in a subfolder with 105
  of them, and a program with an `.err` file beside it, and `LPRINT` is not compiled yet
- **THEN** the report says 3 programs, 1 without errors, 2 with only "not supported yet" errors, 1 left out, and
  one row for `LPRINT` with 2 programs, 2 alone and 106 uses

#### Scenario: A program with another error
- **WHEN** the one program given has an error that is not marked
- **THEN** the summary counts it under "other errors", it is named after the summary, and the exit status is 0

#### Scenario: Unknown report
- **WHEN** `qb64rust --report typed p.bas` is run
- **THEN** the exit status is 1 with the message that the report is unknown
