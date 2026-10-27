# compiler/fixes Specification

## Purpose
The fixes switch: named fixes of QB64pe behaviours that the compiler keeps by default, turned on or off for a
program by `qb64rust.toml` and the command line, so the old behaviour stays available and the new one can be
chosen.

## Requirements

### Requirement: Named fixes
Each fix SHALL have a fixed kebab-case name. The names in this release SHALL be `resume-after-block`,
`dim-empty-error`, `static-local-dynamic`, `whole-array-value`, `resume-in-procedure`, `run-in-procedure` and
`checking-on-service`. A fix SHALL change nothing while it is off: the program SHALL compile and run as with no
switch at all.

#### Scenario: Off is QB64pe's
- **WHEN** a program of the verification suite is built with every fix off
- **THEN** its output is its `qb64pe.exe` recording, as before this change

### Requirement: The settings file
The compiler SHALL read the fixes from the first file named `qb64rust.toml` found in the main `.bas` file's folder
or a folder above it. Its `[fixes]` table SHALL hold `all = true|false` (the master switch) and `<name> =
true|false` for single fixes; a single fix's setting SHALL override `all`. Other tables SHALL be ignored.

#### Scenario: Master switch with an override
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[fixes]`, `all = true` and `resume-after-block = false`
- **THEN** `p.bas` is built with every fix on except `resume-after-block`

#### Scenario: Found in a folder above
- **WHEN** `qb64rust.toml` is in `proj\` and the program is `proj\src\p.bas`, with none in `proj\src\`
- **THEN** the file in `proj\` is used

#### Scenario: The nearest file wins
- **WHEN** both `proj\qb64rust.toml` and `proj\src\qb64rust.toml` exist
- **THEN** only `proj\src\qb64rust.toml` is read for `proj\src\p.bas`

### Requirement: Default off
A program with no `qb64rust.toml` above it, or whose file has no `[fixes]` table, SHALL be built with every fix off.

#### Scenario: No file
- **WHEN** `p.bas` has no `qb64rust.toml` in its folder or above
- **THEN** it is built with every fix off

### Requirement: A bad settings file
An unknown fix name in `[fixes]`, a value that is not `true` or `false`, or a file that is not valid TOML SHALL be
an error that names the file, the line and, for a name, the known names. The program SHALL NOT be built.

#### Scenario: A misspelt name
- **WHEN** `qb64rust.toml` holds `[fixes]` and `resume-after-blok = true`
- **THEN** the compile fails with an error naming `qb64rust.toml`, its line 2 and the known fix names, and exit
  status 1

### Requirement: Command-line fixes
`--fixes on|off` SHALL set the master switch and `--fix <name>=on|off` a single fix, either one any number of
times. The command line SHALL override the file: `--fixes` replaces the file's whole fix set, then each `--fix`
overrides its fix. An unknown name or value SHALL be an error with exit status 1.

#### Scenario: Off overrides the file
- **WHEN** `qb64rust.toml` turns every fix on and `qb64rust --fixes off p.bas` is run
- **THEN** `p.bas` is built with every fix off

#### Scenario: One fix on
- **WHEN** `qb64rust --fixes off --fix whole-array-value=on p.bas` is run
- **THEN** only `whole-array-value` is on

### Requirement: Listing the fixes
`qb64rust --list-fixes` SHALL print one line per fix, its name and what it changes, and exit with status 0.

#### Scenario: The list
- **WHEN** `qb64rust --list-fixes` is run
- **THEN** it prints seven lines, the first beginning `resume-after-block`, and exits with status 0

### Requirement: Fixes and the build cache
A build reused from the build cache SHALL have been made with the same fix set.

#### Scenario: Built again with a fix turned on
- **WHEN** a program is built with the fixes off, then built again with `--fixes on` and the build cache set
- **THEN** the second build is not the first one's executable
