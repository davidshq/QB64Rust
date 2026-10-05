# testing/upstream-tests Specification

## Purpose
Uses QB64pe's own test suite, its classic QBasic programs, its compiler sources and QB64Fresh's test snippets as
inputs for the new compiler's tests, with the old compiler's verdict on each, and measures progress on the
upstream suite against the part of it the plan can reach.

## Requirements

### Requirement: Copy of the upstream test suite
`tests/upstream/compile_tests/` SHALL hold the text files of `tests/compile_tests` of the pinned QB64pe commit
(programs, includes, `.output`, `.err` and sidecar files) under the same relative paths, with QB64pe's licence and
a `SOURCE.md` naming the commit. Binary assets SHALL NOT be copied.

#### Scenario: Copy is current
- **WHEN** the reference clone is present at the pinned commit
- **THEN** every copied file equals the clone's file byte for byte, and every text file of the clone's suite is
  in the copy

#### Scenario: No clone
- **WHEN** the reference clone is missing (as in CI)
- **THEN** the tier-1 checks still run over the copy, and only the comparison with the clone is skipped

### Requirement: Third-party programs are read, not copied
`qbasic_testcases` and the old compiler's own sources SHALL be read from the reference clone only and SHALL NOT be
copied into the repository. Tier-1 checks over them SHALL be skipped, with a note in the test output, when the
clone is missing.

#### Scenario: Clone present
- **WHEN** `cargo test` runs with the clone present
- **THEN** every `.bas`, `.bi` and `.bm` file of `qbasic_testcases`, `source` and `internal/support` goes through
  the front end

### Requirement: QB64Fresh snippets as labelled inputs
`tests/snippets/qb64fresh/` SHALL hold the BASIC snippets of QB64Fresh's inline tests, one deduplicated program per
file, extracted by a script, with a `SOURCE.md`. A snippet that `qb64pe.exe` rejects SHALL have an `.err` file with
the old compiler's message; one it accepts SHALL have none.

#### Scenario: Labelled by the old compiler
- **WHEN** the extraction script is rerun with the same QB64Fresh revision and `qb64pe.exe`
- **THEN** no file under `tests/snippets/qb64fresh/` changes

### Requirement: No false errors
For every program the old compiler accepts (in the corpus, the upstream copy, the snippets, `qbasic_testcases`,
and the old compiler's main source), `cargo test` SHALL fail if the front end reports an error without the
"not supported yet" marker, unless the program is named in `tests/known_false_errors.list`.

#### Scenario: New false error
- **WHEN** a change makes the parser report "expected the end of the statement" on an upstream program that
  `qb64pe` compiles and that is not in the list
- **THEN** `cargo test` fails and names the program and the diagnostic

#### Scenario: Stale entry
- **WHEN** a program in `tests/known_false_errors.list` no longer gets an unmarked error
- **THEN** `cargo test` fails and asks for the entry to be removed

### Requirement: Rejected programs stay rejected
For every program the old compiler rejects (an `.err` file in the corpus, the upstream copy or the snippets),
`cargo test` SHALL fail if the front end reports no error, or if it reports only marked errors and the program is
not named in `tests/known_unsupported_rejections.list`.

#### Scenario: Accepted by mistake
- **WHEN** support for a construct makes the front end accept an upstream `.err` program
- **THEN** `cargo test` fails and names it

#### Scenario: Properly rejected
- **WHEN** a program in `tests/known_unsupported_rejections.list` starts getting an unmarked error
- **THEN** `cargo test` fails and asks for the entry to be removed

### Requirement: Lists are exact and regenerable
Both lists SHALL name programs by a path relative to their input set root with a set prefix
(`corpus/`, `upstream/`, `snippets/`, `qbasic/`, `qb64pe-source/`), one per line, sorted, `#` comments allowed.
Setting `QB64RUST_UPDATE_LISTS=1` SHALL rewrite both lists from the current results instead of failing.

#### Scenario: Regenerating
- **WHEN** `QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs` is run
- **THEN** both lists hold exactly the programs that currently need an entry, and a following `cargo test` passes

### Requirement: Upstream progress
`tests/upstream/deferred.list` SHALL name the upstream programs that need a feature deferred to `SOMEDAY.md`, each
with that feature. `tests/upstream/pass.list` SHALL name the upstream programs the new compiler passes end to end.
Upstream progress SHALL be reported as the size of the pass list out of the programs not deferred.

#### Scenario: Tier-2 upstream run
- **WHEN** `run_legacy_tests.py --suite compile --qb64 target/release/qb64rust.exe --list tests/upstream/pass.list`
  is run with the clone present
- **THEN** every listed program passes

#### Scenario: Deferred and passing
- **WHEN** a program is in both `deferred.list` and `pass.list`
- **THEN** a tier-1 check fails
