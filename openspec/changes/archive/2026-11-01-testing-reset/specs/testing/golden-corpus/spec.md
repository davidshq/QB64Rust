# Spec Delta

## ADDED Requirements

### Requirement: The pass cache of the corpus suite
With `--pass-cache`, a corpus program that passed before SHALL not be built or run again while the C++ the
compiler writes for it, libqb's sources and edits, the C++ toolchain, the runner, its source, its expected files,
its sidecars and its `.files` folder are all unchanged; the executable's bytes SHALL NOT be part of the key.

#### Scenario: Unchanged C++
- **WHEN** a slice program passed with `--pass-cache` and the compiler is rebuilt without changing its C++
- **THEN** the program is reported as passed before, neither built nor run

### Requirement: Programs the pass cache never serves
A program with an `.external` sidecar beside its `.bas` (one line: why its result depends on the clock, a
device, the network, sound or the desktop) SHALL run on every run that reaches it. A failed program SHALL never
be remembered as passed.

#### Scenario: A failure is not remembered
- **WHEN** a program fails with `--pass-cache` and the list runs again with nothing changed
- **THEN** the program runs again

#### Scenario: A screen program
- **WHEN** a screen program with an `.external` sidecar passed and the list runs again with nothing changed
- **THEN** it is built and run again

## MODIFIED Requirements

### Requirement: The pass cache in the compile suite
`--pass-cache` SHALL work with the compile suite as with the corpus suite: a program that passed with the same
C++ written by the compiler, libqb's sources and edits, the C++ toolchain, the runner, source, expected output,
divergence sidecar and files of its folder SHALL be neither built nor run again; the executable's bytes SHALL NOT
be part of the key.

#### Scenario: Nothing changed
- **WHEN** an upstream program passed with `--pass-cache` and is run again with the same C++ written for it
- **THEN** it passes with "not run: passed before" and is neither built nor run

#### Scenario: A file of its folder changed
- **WHEN** a file is added to the program's category folder
- **THEN** the next run with `--pass-cache` runs it again

### Requirement: A run tests a copy of the compiler
A run of the runner with the new compiler, and `callsite.py`, SHALL compile with a copy of the executable made at
the run's start under `target\test-bin`, named by the executable's SHA-256, and SHALL print a line naming that
hash and the time the run started. A `check` step's result SHALL rest on the fingerprint of its inputs as the
plan read them, not on the commit or on when its log was written (spec `testing/check-workflow`).

#### Scenario: A build during a run
- **WHEN** the release build is replaced while a run of the slice list goes on
- **THEN** the run's later programs are compiled by the same compiler as its first, and the next `check` plans the
  step again when the build's inputs differ

#### Scenario: The same bytes built again
- **WHEN** the release build is built again from the same inputs after a run ended
- **THEN** the step's result stands
