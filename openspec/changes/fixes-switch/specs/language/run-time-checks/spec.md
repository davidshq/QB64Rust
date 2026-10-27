# Spec Delta

## ADDED Requirements

### Requirement: Fix checking-on-service
With the fix `checking-on-service` on, an error raised below `$CHECKING:OFF` SHALL still wait through the
unchecked statements, and SHALL be handled before the first checked statement that runs after them (the statement
after `$CHECKING:ON`, or a checked label reached from an unchecked statement), so that `RESUME NEXT` goes on at
that statement and `RESUME` runs it. With the fix off, requirement "Statements without checks" holds.

#### Scenario: RESUME NEXT after the unchecked lines
- **WHEN** `ON ERROR GOTO h` is active, `ERROR 5` stands below `$CHECKING:OFF` followed by `PRINT "a"`, then
  `$CHECKING:ON` is followed by `PRINT "b"`, the handler executes `RESUME NEXT`, and the fix is on
- **THEN** `a`, the handler's line, then `b` are printed

#### Scenario: Fix off
- **WHEN** the same program is built with the fixes off
- **THEN** its output is the old compiler's (`verification\v50_c_checking`)
