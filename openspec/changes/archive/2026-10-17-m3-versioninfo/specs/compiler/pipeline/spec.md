# Spec Delta

## ADDED Requirements

### Requirement: The version resource in the build
A program with `$VERSIONINFO` SHALL be built with the version resource and the manifest the old compiler writes
for it, the same bytes, after the icon of `$EXEICON` when it has one, and with the make flag the old compiler
passes for them. Two builds whose version resources differ SHALL NOT share a cached executable.

#### Scenario: The files of every measured program
- **WHEN** the front end and the driver write the resource files of each accepted `verification\v40_*` program
- **THEN** the resource file, its header and the manifest equal those `qb64pe.exe` wrote for it, apart from the
  manifest's file name, which follows the executable's

#### Scenario: Read back from the executable
- **WHEN** a slice program with `$VERSIONINFO` lines is built and prints its own version resource
- **THEN** its output equals the old compiler's program's

#### Scenario: With an icon
- **WHEN** a program has `$EXEICON:'./icon.ico'` and `$VERSIONINFO:ProductName='p'`
- **THEN** the executable carries both, and its resource file has the icon line first, as the old compiler's

#### Scenario: A changed value
- **WHEN** a `$VERSIONINFO` value changes and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache
