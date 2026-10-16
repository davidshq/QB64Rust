# Spec Delta

## ADDED Requirements

### Requirement: Screen programs of the new compiler
`tests/corpus/slice.list` SHALL name every screen program the new compiler passes, and the tier 2 slice run SHALL
check each by its screen state as well as by its output. A screen program that the new compiler compiles without
an error and that is not on the list SHALL be named in `tests/known_clean_not_passing.list` with the reason, as
any other clean program (`testing/compiler-tests`, "Clean programs are listed").

#### Scenario: A listed screen program
- **WHEN** the tier 2 slice run reaches `screen/m01_screen_1`
- **THEN** it builds the program with the state file, runs it, and passes only if the state written equals
  `m01_screen_1.screen`

#### Scenario: A wrong pixel from the new compiler
- **WHEN** the new compiler's program draws one pixel the old compiler's program does not draw
- **THEN** the tier 2 slice run fails for that program at stage "screen"

#### Scenario: A clean screen program that is not listed
- **WHEN** a program of `tests/corpus/screen` compiles without an error and is on neither list
- **THEN** `cargo test` fails and names it

### Requirement: The state does not depend on the clock
The state file SHALL hold nothing the runtime derives from the time of day or from how long the program ran. In
`SCREEN 10` the runtime sets the first four palette entries of the displayed page at every refresh from the
mode's own palette (entries 4 to 7) and the clock; the state file SHALL write those four entries as 0.

#### Scenario: SCREEN 10 under load
- **WHEN** `tests\corpus\screen\m10_screen_10` is run several times at once with other screen programs
- **THEN** every run writes the same state, whether or not its screen was refreshed before it exited

### Requirement: A screen program for each screen built-in
Every statement and function of `language/screen-output` SHALL be used, in code that runs, by at least one screen
program of `tests/corpus/slice.list`, recorded with the old compiler.

#### Scenario: A statement the first 29 programs do not use
- **WHEN** `WINDOW` is compiled by the new compiler
- **THEN** a screen program recorded with `qb64pe.exe` draws through a `WINDOW` and is on `slice.list`
