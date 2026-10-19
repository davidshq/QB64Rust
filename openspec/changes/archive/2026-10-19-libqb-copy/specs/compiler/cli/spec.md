## MODIFIED Requirements

### Requirement: Locating the reference runtime
The QB64pe reference clone, whose toolchain builds the executable and relative to which `DECLARE LIBRARY` looks
for files, SHALL be taken from `--qb64pe-root`, else from the environment variable `QB64RUST_QB64PE_ROOT`, else
from `..\QB64pe` next to the repository the binary was built from. The copy of libqb SHALL be taken from
`--libqb-root`, else from the environment variable `QB64RUST_LIBQB_ROOT`, else from `libqb` in the repository the
binary was built from; it is needed only to build an executable. If either is needed and not found, the compiler
SHALL fail with a message naming its option and its variable.

#### Scenario: No clone found
- **WHEN** no clone exists at any of the three places
- **THEN** compilation fails with exit status 1 and a message naming `--qb64pe-root` and `QB64RUST_QB64PE_ROOT`

#### Scenario: No copy of libqb found
- **WHEN** a program is compiled to an executable, the clone is found, and no copy of libqb exists at any of the
  three places
- **THEN** compilation fails with exit status 1 and a message naming `--libqb-root` and `QB64RUST_LIBQB_ROOT`
