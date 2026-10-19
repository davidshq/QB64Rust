## MODIFIED Requirements

### Requirement: C++ output and build
The C++ emitter SHALL write every fragment that `qbx.cpp` includes into a build folder that is inside neither the
QB64pe reference clone nor this repo's copy of libqb, and the driver SHALL build the executable with the copy's
`Makefile`, `qbx.cpp` and libqb sources and the clone's toolchain, compiling generated code with `-fwrapv`. The
driver's flags (`-fwrapv`, `-O2` for `OptimizeCppProgram`) SHALL reach the program's own objects only; libqb's
objects SHALL be built with none of them, whatever build makes them. The libqb objects a build needs SHALL be
written into the copy's git-ignored places, never into the clone, and a copy whose objects were built by another
toolchain or with other flags of the driver SHALL have them built again. A build SHALL NOT change any tracked file
of the reference clone or of the copy.

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

#### Scenario: An optimised build first
- **WHEN** the copy has no libqb objects and a program with `OptimizeCppProgram` is built first
- **THEN** `qbx.cpp` is compiled with `-O2 -fwrapv` and libqb's sources without either

#### Scenario: Objects of an earlier build
- **WHEN** the copy holds libqb objects built before its stamp was written, or by another toolchain, and a program
  is built
- **THEN** those objects are removed and built again, and the copy's stamp names the toolchain
