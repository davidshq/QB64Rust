## ADDED Requirements

### Requirement: The runner on Linux
`run_legacy_tests.py` SHALL run its corpus, compile, upstream and verification suites on Linux with the new
compiler, as on Windows. Executables have no extension, a program's standard input comes from the null device or its
`.stdin` file, and `QB64PE_NOPROMPT=y` is set. A suite whose programs need Windows (`DECLARE LIBRARY` of a
DLL, the Windows console) SHALL skip each with the reason, from the suite's Linux list.

#### Scenario: The slice list on Linux
- **WHEN** `run_legacy_tests.py --suite corpus --list tests/corpus/slice.list` runs on Linux with the new compiler
- **THEN** each program is built and judged, and the summary counts pass, fail, known failure and skipped

#### Scenario: A Windows-only program
- **WHEN** a program of the corpus calls `kernel32` and is listed in its suite's `known.linux.list` as `windows`
- **THEN** it is skipped on Linux with the reason `windows` and counted as skipped, not as a failure

### Requirement: Recordings per platform
A test MAY have `<name>.linux.output` or `<name>.linux.err` beside its `<name>.output` or `<name>.err` (in the
verification suite, `<name>.linux.out.txt` beside `<name>.out.txt`). On Linux the
runner SHALL judge the program against the Linux file when there is one, else against the recording shared by both
platforms. On Windows it SHALL never read the Linux files. A Linux file SHALL exist only where QB64pe's Linux build
gives another result than the recording.

#### Scenario: A comma on the console
- **WHEN** a `$CONSOLE:ONLY` program prints with commas, its `.output` holds the Windows zones and its
  `.linux.output` the Linux spacing
- **THEN** the run on Linux passes against `.linux.output`, and the run on Windows against `.output`

#### Scenario: A Linux file equal to the shared one
- **WHEN** a `.linux.output` equals the program's `.output` byte for byte
- **THEN** the runner's check of the recordings fails, naming the file to delete

### Requirement: Recording on Linux
`--record --platform linux` SHALL record with `qb64pe` built on Linux from the reference commit (its folder given by
`--qb-root`). It SHALL write `<name>.linux.output` or `.linux.err` only when both runs agree and the result differs
from the shared recording, and remove a Linux file the result no longer needs. The Linux
build of `qb64pe` SHALL be made by a script in a scratch folder, never in the reference clone.

#### Scenario: Same result as Windows
- **WHEN** a program recorded on Linux prints what its `.output` holds
- **THEN** no `.linux.output` is written, and one left by an earlier recording is removed

#### Scenario: The clone stays read-only
- **WHEN** the Linux `qb64pe` is built for recording
- **THEN** no file under the reference clone is written
