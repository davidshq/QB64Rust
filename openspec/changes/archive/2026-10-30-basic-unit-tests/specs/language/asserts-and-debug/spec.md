# Spec Delta

## MODIFIED Requirements

### Requirement: _ASSERT
`_ASSERT condition[, message$]` SHALL do nothing in a program without `$ASSERTS` (the condition and the message
are still evaluated where the old compiler evaluates them). With `$ASSERTS`, a condition that is 0 SHALL raise
runtime error 314; in a program with a console and with a message, the message SHALL be written to the console and
error 315 raised instead, as libqb does. As with the old compiler, `ON ERROR` SHALL NOT trap these errors (libqb
treats 300 to 315 as errors no handler takes): the program stops with the runtime error, and exits with code 1
where the old compiler's program exits 0 (`QB64PE-DIVERGENCES.md` D-012).

#### Scenario: Without $ASSERTS
- **WHEN** a program without `$ASSERTS` runs `_ASSERT 0, "never"` and `PRINT "after"`
- **THEN** it prints `after`

#### Scenario: Not trapped
- **WHEN** a program with `$ASSERTS` and `ON ERROR GOTO h` runs `_ASSERT 0` and the handler prints `ERR`
- **THEN** the handler does not run: the program stops with "_ASSERT failed" at the `_ASSERT`'s line, as the old
  compiler's program does

#### Scenario: A message in a console program
- **WHEN** a `$CONSOLE:ONLY` program with `$ASSERTS` runs `_ASSERT 0, "bad"`
- **THEN** its output equals the old compiler's program's, and its exit code is 1

#### Scenario: Wrong arguments
- **WHEN** a program has `_ASSERT` with no argument, or with a string as the condition
- **THEN** it is a compile error, as with the old compiler
