# testing/compiler-tests Specification

## Purpose
How the new compiler is tested below the golden corpus: our own front-end tests with mode lines, snapshot tests
of the typed tree, IR and C++, and the tier-1 and tier-2 runs over the corpus (`study\19`).

## Requirements

### Requirement: Mode lines
Every `.bas` file under `tests/frontend/` SHALL start with a line `' TEST: <mode>`, where mode is one of
`parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`. The test harness SHALL run each file according to
its mode, and SHALL fail a file that has no valid mode line.

#### Scenario: check-fail test
- **WHEN** a file with `' TEST: check-fail` produces no diagnostics
- **THEN** the test fails

#### Scenario: Missing mode line
- **WHEN** a file under `tests/frontend/` does not start with a mode line
- **THEN** the harness reports that file as an error

### Requirement: Snapshot tests
Diagnostics (`check-fail`), typed-tree dumps (`typed`), IR (`ir`) and emitted fragments (`cpp`) SHALL be compared
with `insta` snapshot files committed in the repository. A changed output SHALL fail the test until the snapshot
is reviewed and accepted.

#### Scenario: Lowering changes
- **WHEN** a change alters the IR of a `ir` test
- **THEN** `cargo test` fails and shows the difference

### Requirement: Tier 1 corpus checks
`cargo test` SHALL, without compiling C++, parse and check every `.bas` file under `tests/corpus`, require no
panic and an exact round trip of the syntax tree for each, require no diagnostics for every program named in
`tests/corpus/slice.list` (however many it names), and apply the "No false errors" and "Rejected programs stay
rejected" checks of `testing/upstream-tests` to the corpus programs. Every input of tier 1 that the front end
accepts (corpus, upstream, snippets, and the clone's sets when the clone is present) SHALL also be lowered to the
IR, validated, and emitted as C++ fragments, without panicking and without a validation problem.

#### Scenario: Unsupported program
- **WHEN** a corpus program uses features the compiler does not support yet
- **THEN** tier 1 passes for it as long as the front end reports only marked diagnostics without panicking and
  round-trips the tree

#### Scenario: Rejected program stays rejected
- **WHEN** support for a new construct makes the front end accept a corpus program that has an `.err` file
- **THEN** `cargo test` fails and names that program

#### Scenario: Lowering bug in an accepted program
- **WHEN** the lowering of an accepted upstream program produces a jump to a label of another body
- **THEN** `cargo test` fails, naming the program and the jump, before any C++ is compiled

#### Scenario: Emitter panic
- **WHEN** the emitter panics on an accepted corpus program
- **THEN** `cargo test` fails and names that program and the panic message

### Requirement: Clean programs are listed
`cargo test` SHALL fail when a corpus or upstream program that the old compiler accepts compiles without an error
and is named in none of `tests/corpus/slice.list`, `tests/upstream/pass.list` and
`tests/known_clean_not_passing.list`, so that every program the new compiler builds an executable for is run in
tier 2 or has a recorded reason why not (`study\26` §3). `tests/known_clean_not_passing.list` SHALL be shrink-only
and kept by hand, one entry per program with its reason; `cargo test` SHALL fail for an entry that no longer
compiles cleanly or that a pass list names.

#### Scenario: Newly clean program
- **WHEN** a change makes an upstream program compile without an error and it is on no list
- **THEN** `cargo test` fails and names the program, until it is run in tier 2 and added to `pass.list` or, with
  the reason, to `tests/known_clean_not_passing.list`

#### Scenario: Known failure fixed
- **WHEN** a program of `tests/known_clean_not_passing.list` is added to a pass list
- **THEN** `cargo test` fails until its entry is removed

### Requirement: Tier 2 slice run
The corpus runner SHALL accept `--list <file>` to run only the programs named in it (one `<group>/<name>` per
line, `#` comments allowed). Running it with `--qb64 <qb64rust.exe> --list tests/corpus/slice.list` SHALL pass
every listed program.

#### Scenario: Slice list run
- **WHEN** `run_legacy_tests.py --suite corpus --qb64 target/release/qb64rust.exe --list tests/corpus/slice.list`
  is run
- **THEN** every listed program passes

### Requirement: Slice corpus group
`tests/corpus/slice/` SHALL hold programs written for this project, with a `SOURCE.md`, and expected output
recorded with `qb64pe.exe` using the corpus runner's `--record`, under the golden-corpus layout rules.

#### Scenario: Recorded by the old compiler
- **WHEN** the old compiler is checked against the corpus including the `slice` group
- **THEN** every slice program passes

### Requirement: Seeded mutation test
`cargo test` SHALL run a deterministic mutation test: from a fixed seed, apply byte-level edits (deletions,
insertions of arbitrary bytes, duplicated and swapped ranges) to corpus programs and run each result through the
front end, requiring no panic and an exact round trip. A failure SHALL print the seed and the mutated input's
location so it can be reproduced.

#### Scenario: Parser panic found
- **WHEN** a mutated program makes the parser panic
- **THEN** the test fails and prints the program, the mutation number and the seed

#### Scenario: Same run twice
- **WHEN** the test is run twice on the same tree
- **THEN** it checks the same mutated inputs

### Requirement: Built-in coverage
Every built-in function that `sema` compiles SHALL be called in at least one program of `tests/corpus/slice.list`
(so its output is compared with the old compiler's in tier 2) and in at least one `typed` front-end test (so its
result type and argument conversions are pinned in tier 1). `cargo test` SHALL fail and name the built-in when
either is missing.

#### Scenario: Built-in added without a slice program
- **WHEN** `sema` starts compiling a built-in that no slice program calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Built-in added without a typed test
- **WHEN** `sema` starts compiling a built-in that no `typed` front-end test calls
- **THEN** `cargo test` fails and names that built-in
