## ADDED Requirements

### Requirement: Runs of the new compiler share the reference clone
The runner, `callsite.py` and the CLI tests that build SHALL take the lock of the reference clone before they
use it: alone for a run that uses `qb64pe.exe` in the clone (`callsite.py` among them), shared for a run that
builds with the new compiler only. A run SHALL stop with exit code 3, or wait with `--wait`, while another holds the lock in a way that excludes
it. A run that takes it shared SHALL name the objects' lock in `QB64RUST_OBJECTS_LOCK` for every compile, and the
driver SHALL then take that lock around each `make`: shared when the clone's libqb objects for the build's flags
are up to date, alone otherwise.

#### Scenario: Two runs of the new compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64rust` starts
- **THEN** the corpus run goes ahead without waiting

#### Scenario: A run of the old compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64pe` starts without `--wait`
- **THEN** it stops with exit code 3

#### Scenario: A missing object
- **WHEN** a build's flags need a libqb object the clone does not have
- **THEN** the driver builds it holding the objects' lock alone, and a build that holds it shared waits until
  then

### Requirement: A failure beside other builds runs again alone
With the new compiler and `--jobs` above 1, the runner SHALL run each program that failed once more after the
others and report both outcomes. A program that passes then SHALL count as passed; the run SHALL fail unless the
program is timing-sensitive (a screen program, or one whose source names a statement that waits in real time,
reads the clock, plays sound, or reads the keyboard or the mouse while it runs).

#### Scenario: A timing-sensitive program
- **WHEN** a program that plays a sound fails with `--jobs 8` and passes alone
- **THEN** it counts as passed, a note names it, and the run does not fail on its account

#### Scenario: Another program
- **WHEN** a program that only prints fails with `--jobs 8` and passes alone
- **THEN** it counts as passed, a note names it, and the run fails

### Requirement: The pass cache in the compile suite
`--pass-cache` SHALL work with the compile suite as with the corpus suite: a program that passed with the same
executable, source, expected output, divergence sidecar and files of its folder SHALL be compiled and not run
again.

#### Scenario: Nothing changed
- **WHEN** an upstream program passed with `--pass-cache` and is run again with the same executable
- **THEN** it passes with "not run: passed before" and is not run

#### Scenario: A file of its folder changed
- **WHEN** a file is added to the program's category folder
- **THEN** the next run with `--pass-cache` runs it again
