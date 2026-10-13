# Spec Delta

## MODIFIED Requirements

### Requirement: Built-in coverage
Every built-in function and every built-in statement that `sema` compiles SHALL be used in at least one program of
`tests/corpus/slice.list` (so its output is compared with the old compiler's in tier 2) and in at least one
front-end test that pins it in tier 1: a `typed` test for a function (its result type and argument conversions),
an `ir` test for a statement (its slots, places and items). `cargo test` SHALL fail and name the built-in when
either is missing. For a built-in that no test runs (a window built-in, `language/console-and-window`; `BEEP`,
which rings the machine's bell, `language/sound`), the slice program
SHALL hold it as code that is compiled and not reached, and the built-in SHALL also have a call-site program for
each of its forms; the list of such built-ins SHALL be explicit, and `cargo test` SHALL fail when one of them has
no call-site program.

#### Scenario: Built-in added without a slice program
- **WHEN** `sema` starts compiling a built-in that no slice program calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Built-in added without a typed test
- **WHEN** `sema` starts compiling a built-in that no `typed` front-end test calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Statement added without an ir test
- **WHEN** `sema` starts compiling a built-in statement that no `ir` front-end test uses
- **THEN** `cargo test` fails and names that statement

#### Scenario: Window built-in without a call-site program
- **WHEN** `sema` starts compiling a window built-in that no program of `tests/callsite/` uses
- **THEN** `cargo test` fails and names that built-in

## ADDED Requirements

### Requirement: Real programs compile
`cargo test` SHALL require of every program under `tests/programs` that the front end reports no diagnostic
at all for it and that it is lowered, validated and emitted as C++ fragments. A test that needs the reference
clone SHALL build the executable of each such program and SHALL NOT run it.

#### Scenario: The game compiles
- **WHEN** `cargo test` runs
- **THEN** Civil War Strategy has no diagnostic, marked or not, and its C++ fragments are written

#### Scenario: A built-in is taken out
- **WHEN** a change makes the front end report "not supported yet" for a line of Civil War Strategy
- **THEN** `cargo test` fails and names the program and the line

#### Scenario: The game builds
- **WHEN** the test that needs the clone is run
- **THEN** the game's executable is built, with its icon, and the test does not start it
