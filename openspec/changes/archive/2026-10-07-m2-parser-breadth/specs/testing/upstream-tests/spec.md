# Spec Delta

## MODIFIED Requirements

### Requirement: Lists are exact and regenerable
The lists `tests/known_false_errors.list`, `tests/known_unsupported_rejections.list` and
`tests/known_parse_gaps.list` SHALL name programs by a path relative to their input set root with a set prefix
(`corpus/`, `upstream/`, `snippets/`, `qbasic/`, `qb64pe-source/`), one per line, sorted, `#` comments allowed.
Setting `QB64RUST_UPDATE_LISTS=1` SHALL rewrite all three lists from the current results instead of failing.

#### Scenario: Regenerating
- **WHEN** `QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs` is run
- **THEN** the three lists hold exactly the programs that currently need an entry, and a following `cargo test`
  passes

### Requirement: Upstream progress
`tests/upstream/deferred.list` SHALL name the upstream programs that need a feature deferred to `SOMEDAY.md`, each
with that feature. `tests/upstream/pass.list` SHALL name the upstream programs the new compiler passes end to end.
Upstream progress SHALL be reported as the size of the pass list out of the programs not deferred. For a compiler
other than `qb64pe`, an upstream program with an `.err` file SHALL pass when its compile fails, writes no
executable, and reports at least one error not marked "not supported yet"; the old compiler's message text SHALL
NOT be compared.

#### Scenario: Tier-2 upstream run
- **WHEN** `run_legacy_tests.py --suite compile --qb64 target/release/qb64rust.exe --list tests/upstream/pass.list`
  is run with the clone present
- **THEN** every listed program passes

#### Scenario: Deferred and passing
- **WHEN** a program is in both `deferred.list` and `pass.list`
- **THEN** a tier-1 check fails

#### Scenario: Rejected program with the new compiler
- **WHEN** an upstream `.err` program is compiled by `qb64rust` and gets a real error with a different message
- **THEN** the runner counts it as passed

#### Scenario: Rejected only as not supported
- **WHEN** an upstream `.err` program gets only "not supported yet" errors from `qb64rust`
- **THEN** the runner counts it as failed and says so

## ADDED Requirements

### Requirement: Parse gaps
For every program the old compiler accepts (in every input set, and every file of the old compiler's sources),
`cargo test` SHALL fail if its parse reports a diagnostic or leaves an `Error` node, unless the program is named in
`tests/known_parse_gaps.list`; and SHALL fail if a named program no longer needs its entry.

#### Scenario: New parse gap
- **WHEN** a change makes an upstream program that parsed cleanly get an `Error` node
- **THEN** `cargo test` fails and names the program and the parser diagnostic

#### Scenario: Gap closed
- **WHEN** a program in `tests/known_parse_gaps.list` parses cleanly
- **THEN** `cargo test` fails and asks for the entry to be removed
