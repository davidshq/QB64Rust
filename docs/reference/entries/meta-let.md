# $LET

`$LET` sets a name that a `$IF` condition can test at compile time. It is not a variable: the program cannot read
it when it runs.

## Syntax

```text
$LET name = value
```

## Arguments

- `name`: letters, digits, `_` and `.`, starting with a letter, at most 40 characters.
- `value`: a number or a word, such as `1`, `-1`, `2.5` or `FAST`.

## Behavior

From its line on, in file order, a `$IF` sees `name` with this value. A later `$LET` of the same name changes it.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_conditions -->

A `$LET` in an included file reaches the lines of the including file below the `$INCLUDE`.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m7_let_reaches_main -->

Names and values are read without regard to letter case, and blanks around the `=` are allowed: after
`$LET S = hello`, `$IF S = HELLO THEN` is true.
<!-- src: verification\v16_m6_conditions; verification\v16_m6_let_forms -->

A `$LET` of an always-set name such as `WIN` does not change it: after `$LET WIN = 0`, `$IF WIN THEN` is still
true on Windows.
<!-- src: verification\v16_m6_let_forms; tests\corpus\slice\s23_preprocessor -->

A value is one word. After `$LET Q = two words`, `$IF Q = two THEN` is false, and `$IF Q = TWO WORDS THEN` is an
error.
<!-- src: verification\v16_m6_let_forms; verification\v16_m6_let_two_words -->

A `$LET` in a `$IF` branch that is not compiled sets nothing.
<!-- src: compiler/pipeline "Preprocessor"; tests\corpus\slice\s23_preprocessor -->

A file that includes itself can stop the second inclusion with a `$LET` and a `$IF`:
`$IF DONE = UNDEFINED THEN`, `$LET DONE = 1`, the `$INCLUDE`, `$END IF`.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m7_self_guarded -->

## Errors

These are compile errors:

- `$LET` with no `=`: "expected `$LET name = value`".
  <!-- src: verification\v16_m6_let_no_value -->
- A `$LET` after a statement and a colon: "a metacommand must stand at the start of a line".
  <!-- src: verification\v26_x46_let_after_colon -->
- A `$LET` after `THEN` in a single-line `IF`: "expected a statement".
  <!-- src: verification\v16_m6_in_single_if -->
- A tab before `$LET`: "a metacommand must not have a tab before it".
  <!-- src: verification\v26_x41_tab_before_let -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-let_1 -->
```qb64
$LET DEBUG = 1
$IF DEBUG THEN
    PRINT "DEBUG is 1: the debug lines are compiled."
$END IF
$LET DEBUG = 0
$IF DEBUG THEN
    PRINT "This line is left out."
$ELSE
    PRINT "DEBUG is now 0."
$END IF
$LET MODE = fast
$IF MODE = FAST THEN
    PRINT "MODE is FAST: the letter case of a value does not count."
$END IF
END
```

Output:

```text
DEBUG is 1: the debug lines are compiled.
DEBUG is now 0.
MODE is FAST: the letter case of a value does not count.

Press any key to continue
```
<!-- end generated -->

## See also

[`$IF`](meta-if.md), [`$INCLUDE`](meta-include.md), [`CONST`](const.md).
