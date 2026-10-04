# Spec Delta

Builds on the version of "Tier 1 corpus checks" in `m2-procedures-and-errors`, which is archived first.

## MODIFIED Requirements

### Requirement: Tier 1 corpus checks
`cargo test` SHALL, without compiling C++, parse and check every `.bas` file under `tests/corpus`, require no
panic and an exact round trip of the syntax tree for each, require no diagnostics for every program named in
`tests/corpus/slice.list` (however many it names), and apply the "No false errors" and "Rejected programs stay
rejected" checks of `testing/upstream-tests` to the corpus programs.

#### Scenario: Unsupported program
- **WHEN** a corpus program uses features the compiler does not support yet
- **THEN** tier 1 passes for it as long as the front end reports only marked diagnostics without panicking and
  round-trips the tree

#### Scenario: Rejected program stays rejected
- **WHEN** support for a new construct makes the front end accept a corpus program that has an `.err` file
- **THEN** `cargo test` fails and names that program

## ADDED Requirements

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
