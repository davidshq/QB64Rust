# Spec Delta

## ADDED Requirements

### Requirement: A program recorded with a compiler root of its own
A verification program with a sidecar `<name>.root`, which names a folder of `verification\` that holds a
`libraries\` folder, SHALL be recorded with `qb64pe.exe` run from a copy of the release under `target\` whose
`libraries\` is that folder, and SHALL be compiled by any other compiler with that folder as its compiler root
(`--include-root`). The clone SHALL NOT be written to, and the copy SHALL be made again when the clone's
`qb64pe.exe` changes.

#### Scenario: A library found only through the sidecar
- **WHEN** `v58_u01` has `v58_u01.root` naming `v58_root`, and `v58_root\libraries\descriptors\test\a.ini` exists
- **THEN** its recording is the old compiler's run with that library, and the new compiler's compile finds the
  same library

#### Scenario: The clone untouched
- **WHEN** the programs with a `.root` sidecar are recorded
- **THEN** `git status` of the clone shows no change, and no `libraries\` folder exists in it

#### Scenario: Without the sidecar
- **WHEN** a program with `$USELIBRARY` has no `.root` sidecar
- **THEN** it is recorded with the clone, whose release has no `libraries\` folder, and its recording is the old
  compiler's "Library descriptor … not found"
