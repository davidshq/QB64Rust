# Spec Delta

## MODIFIED Requirements

### Requirement: Tier 1 corpus checks
`cargo test` SHALL, without compiling C++, parse and check every `.bas` file under `tests/corpus`, require no
panic and an exact round trip of the syntax tree for each, require no diagnostics for every program named in
`tests/corpus/slice.list` (however many it names), and require at least one error for every program that has an
`.err` file (the old compiler rejects it).

#### Scenario: Unsupported program
- **WHEN** a corpus program uses features the compiler does not support yet
- **THEN** tier 1 passes for it as long as the front end reports diagnostics without panicking and round-trips
  the tree

#### Scenario: Rejected program stays rejected
- **WHEN** support for a new construct makes the front end accept a corpus program that has an `.err` file
- **THEN** `cargo test` fails and names that program
