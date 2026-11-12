# IF

`IF` runs statements only when a condition is true. A block `IF` can test several conditions in turn with
`ELSEIF` and end with `ELSE`; a single-line `IF` fits on one line.

## Syntax

```text
IF condition THEN
    statements
[ELSEIF condition THEN
    statements]...
[ELSE
    statements]
END IF

IF condition THEN statements [ELSE statements]
IF condition GOTO label
```

`END IF` may be written `ENDIF`.

## Arguments

- `condition`: a number. 0 is false, any other value is true. A comparison gives -1 or 0.

## Behavior

A block `IF` runs the first branch whose condition is true, testing `IF` and then each `ELSEIF` in order. When
none is true it runs the `ELSE` branch, if there is one.
<!-- src: language/control-flow "IF"; tests\reference\examples\if_1 -->

A condition is true when its value is not 0: `IF lives THEN` runs while `lives` is 2.
<!-- src: language/control-flow "IF"; tests\reference\examples\if_1 -->

In a single-line `IF`, every statement after `THEN` up to `ELSE` belongs to the `THEN` part, and every statement
after `ELSE` to the end of the line belongs to the `ELSE` part: `IF 0 THEN PRINT "a" ELSE PRINT "b": PRINT "c"`
prints `b` and `c`.
<!-- src: language/control-flow "IF" -->

`IF c GOTO label` jumps when `c` is true. After `THEN` or `ELSE`, a line number alone is a jump to it:
`IF c THEN 100 ELSE 200`.
<!-- src: language/control-flow "IF"; language/control-flow "Line numbers as jump targets" -->

A whole `FOR … NEXT` may stand in either part of a single-line `IF`. `IF c THEN REM text` is a single-line `IF`
with an empty body.
<!-- src: language/control-flow "IF"; verification\v16_m4_line_if_block -->

Both sides of `AND` and `OR` in a condition are evaluated, even when the left side already decides it.
<!-- src: Q-026; verification\qb45\q45_and_short_circuit -->

## Errors

These are compile errors:

- A string condition: "the condition of `IF` must be a number, not a string".
  <!-- src: language/control-flow "IF" -->
- A block `IF` with no `END IF`: "`IF` without `END IF`", at the `IF`. An `END IF` or `ELSE` with no `IF`:
  "`END IF` without `IF`", "`ELSE` without `IF`".
  <!-- src: compiler/pipeline "Block structure" -->
- A second `ELSE` in one block: "this `IF` already has an `ELSE`".
  <!-- src: compiler/pipeline "Block structure" -->

## Differences

From BASIC PDS 7.1: it skips the right side of `AND` or `OR` in some conditions. QB64Rust evaluates both sides, as
QB 4.5 and QB64pe do (`DIVERGENCES-QB45.md` Q-026).
<!-- src: Q-026 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:if_1 -->
```qb64
score = 72
IF score >= 90 THEN
    PRINT "Grade A"
ELSEIF score >= 70 THEN
    PRINT "Grade B"
ELSE
    PRINT "Grade C"
END IF
IF score > 50 THEN PRINT "Passed." ELSE PRINT "Failed."
lives = 2
IF lives THEN PRINT "Any value but 0 is true:"; lives
END
```

Output:

```text
Grade B
Passed.
Any value but 0 is true: 2 

Press any key to continue
```
<!-- end generated -->

## See also

`SELECT CASE`, `_ANDALSO`, `_IIF`, `$IF`.
