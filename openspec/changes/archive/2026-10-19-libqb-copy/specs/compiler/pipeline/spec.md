## MODIFIED Requirements

### Requirement: C++ output and build
The C++ emitter SHALL write every fragment that `qbx.cpp` includes into a build folder that is inside neither the
QB64pe reference clone nor this repo's copy of libqb, and the driver SHALL build the executable with the copy's
`Makefile`, `qbx.cpp` and libqb sources and the clone's toolchain, compiling generated code with `-fwrapv`. The
libqb objects a build needs SHALL be written into the copy's git-ignored places, never into the clone. A build
SHALL NOT change any tracked file of the reference clone or of the copy.

#### Scenario: Reference clone unchanged
- **WHEN** a slice program is built
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build

#### Scenario: The copy unchanged
- **WHEN** a slice program is built for the first time after the copy is checked out, so that libqb's objects
  are built
- **THEN** `git status --porcelain libqb` in this repo prints nothing

#### Scenario: Same output as the old compiler
- **WHEN** a program in `tests/corpus/slice.list` is compiled by `qb64rust` and run by the corpus runner
- **THEN** its output equals the `.output` recorded from `qb64pe.exe`

## ADDED Requirements

### Requirement: The copy of libqb equals the reference commit
This repo SHALL hold a copy of QB64pe's `Makefile`, `internal\c` (without the toolchain), `licenses` and
`COPYING.txt`, each file byte for byte as at the commit its `SOURCE.md` names, which SHALL be the reference
clone's commit. A tool SHALL write the copy from the clone and SHALL say, with `--check`, which files differ from
the commit, are missing or are extra.

#### Scenario: A file edited by hand
- **WHEN** a file of the copy is edited and the check is run
- **THEN** it exits with status 1 and names the file

#### Scenario: The copy as written
- **WHEN** the copy has just been written by the tool and the check is run
- **THEN** it exits with status 0
