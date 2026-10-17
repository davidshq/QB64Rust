## MODIFIED Requirements

### Requirement: Runs of the new compiler share the reference clone
The runner, `callsite.py` and the CLI tests that build SHALL take the lock of the reference clone before they
use it: alone for a run that uses `qb64pe.exe` in the clone (`callsite.py` among them), shared for a run that
builds with the new compiler only. A run SHALL stop with exit code 3, or wait with `--wait`, while another holds the lock in a way that excludes
it. A run that takes it shared SHALL name the objects' lock in `QB64RUST_OBJECTS_LOCK` for every compile, and the
driver SHALL then take that lock around each `make`: shared when the clone's libqb objects for the build's flags
are up to date, alone otherwise. A run started in a git worktree of this repo SHALL take the lock files of the
main checkout.

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

#### Scenario: A run in a worktree
- **WHEN** a corpus run of `qb64rust` is running in the main checkout and a run of `qb64pe` starts in a worktree
  without `--wait`
- **THEN** it stops with exit code 3

## ADDED Requirements

### Requirement: The build cache keys on what make reads
With `QB64RUST_BUILD_CACHE` set, the driver SHALL key a build on its fragments, the clone's `qbx.cpp` and
`Makefile`, its settings, the toolchain's executables, and the size and time of each file under the clone's
`internal\c` that no build writes. The files a build of either compiler writes there (libqb objects and
archives, `.tmp` files, `qbx<N>.cpp` and `qbx*.o` in `internal\c`) SHALL NOT be part of the key.

#### Scenario: A compile with the old compiler
- **WHEN** a program was built with the build cache, `qb64pe.exe` then compiled a program in the clone, and the
  program is built again
- **THEN** the build is answered from the cache

#### Scenario: A libqb source changed
- **WHEN** a source or header of libqb under `internal\c` changes
- **THEN** the key changes and the program is built with `make`

### Requirement: A run tests a copy of the compiler
A run of the runner with the new compiler, and `callsite.py`, SHALL compile with a copy of the executable made at
the run's start under `target\test-bin`, named by the executable's SHA-256, and SHALL print a line naming that
hash and the time the run started. `tools\close\close.py --check` SHALL judge a closing step that prints the line
by the hash against the current release build and by the start against the changed test inputs.

#### Scenario: A build during a run
- **WHEN** the release build is replaced while a run of the slice list goes on
- **THEN** the run's later programs are compiled by the same compiler as its first, and `close.py --check` names
  the step stale when the new build differs

#### Scenario: The same bytes built again
- **WHEN** the release build is built again with the same bytes after a closing run ended
- **THEN** `close.py --check` still counts the run
