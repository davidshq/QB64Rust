# Spec Delta

## MODIFIED Requirements

### Requirement: Selecting and caching the compare
The compare SHALL take program names, `--glob`, `--list <file>` (one program name per line, `#` comments
allowed), `--jobs`, `--build-cache` and `--pass-cache` as the corpus suite does. A program passed before SHALL
not be built or run again while the C++ the compiler writes for it, libqb's sources and edits, the C++
toolchain, the runner, its source, its recorded files and its sidecars are all unchanged; the executable's bytes
SHALL NOT be part of the key. A program on `known.list` as `varies` SHALL never be served from the cache.
`--check-list` SHALL judge a list from the last full compare (`results-verification.json`).

#### Scenario: Pass cache
- **WHEN** a program passed and is compared again with `--pass-cache` and nothing it reads has changed
- **THEN** its C++ is written and found unchanged, it is neither built nor run, and it is reported as passed before

#### Scenario: The compiler rebuilt, the C++ unchanged
- **WHEN** the compiler is built again from the same sources, or from a change that leaves the program's C++ as
  it was, and the list runs with `--pass-cache`
- **THEN** every program whose C++ is unchanged is reported as passed before

#### Scenario: libqb edited
- **WHEN** a file of libqb's copy or its edits changes
- **THEN** the next run with `--pass-cache` builds and runs every program again
