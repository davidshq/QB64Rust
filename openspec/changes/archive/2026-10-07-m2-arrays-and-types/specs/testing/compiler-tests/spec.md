# Spec Delta

## MODIFIED Requirements

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
