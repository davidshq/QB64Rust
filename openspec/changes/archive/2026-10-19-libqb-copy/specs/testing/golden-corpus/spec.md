## MODIFIED Requirements

### Requirement: Runs of the new compiler share the reference clone
The runner, `callsite.py` and the CLI tests that build SHALL take the lock of the reference clone alone for a run
that uses `qb64pe.exe` in the clone (`callsite.py` among them). A run that builds with the new compiler only
SHALL take no lock of the clone (it reads only the clone's toolchain and the files `DECLARE LIBRARY` names, which
no compile writes), unless it runs programs in the clone's own test folders: then it SHALL take it shared. A run
that needs the clone's lock SHALL stop with exit code 3, or wait with `--wait`, while another holds it. A run of
the new compiler SHALL name the objects' lock in `QB64RUST_OBJECTS_LOCK` for every compile, and the driver SHALL
then take that lock around each `make`: shared when the copy's libqb objects for the build's flags are up to
date, alone otherwise. A run started in a git worktree of this repo SHALL take the lock files of the main
checkout.

#### Scenario: Two runs of the new compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64rust` starts
- **THEN** the corpus run goes ahead without waiting

#### Scenario: A run of the old compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64pe` starts without `--wait`
- **THEN** the corpus run of `qb64pe` goes ahead without waiting

#### Scenario: Programs in the clone's folders
- **WHEN** a compile-suite run of `qb64rust` on the clone's `tests\compile_tests` is running and a corpus run of
  `qb64pe` starts without `--wait`
- **THEN** the run of `qb64pe` stops with exit code 3

#### Scenario: Two runs of the old compiler
- **WHEN** a corpus run of `qb64pe` is running and a verification run of `qb64pe` starts without `--wait`
- **THEN** it stops with exit code 3

#### Scenario: A missing object
- **WHEN** a build's flags need a libqb object the copy does not have
- **THEN** the driver builds it holding the objects' lock alone, and a build that holds it shared waits until
  then

#### Scenario: A run in a worktree
- **WHEN** a corpus run of `qb64pe` is running in the main checkout and a run of `qb64pe` starts in a worktree
  without `--wait`
- **THEN** it stops with exit code 3

### Requirement: The build cache keys on what make reads
With `QB64RUST_BUILD_CACHE` set, the driver SHALL key a build on its fragments, the copy's `qbx.cpp` and
`Makefile`, its settings, the clone's toolchain executables, and the size and time of each file under the copy's
`internal\c` that no build writes. The files a build writes there (libqb objects and archives, `.tmp` files)
SHALL NOT be part of the key, and nothing a compile with `qb64pe.exe` writes in the clone SHALL be.

#### Scenario: A compile with the old compiler
- **WHEN** a program was built with the build cache, `qb64pe.exe` then compiled a program in the clone, and the
  program is built again
- **THEN** the build is answered from the cache

#### Scenario: A libqb source changed
- **WHEN** a source or header of libqb under the copy's `internal\c` changes
- **THEN** the key changes and the program is built with `make`
