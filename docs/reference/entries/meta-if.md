# $IF

`$IF` chooses at compile time which lines of a program are compiled. It is how one source holds code for
Windows and for Linux, or a debug version beside the normal one. `$ELSEIF`, `$ELSE` and `$END IF` belong to it.

## Syntax

```text
$IF condition THEN
    lines
$ELSEIF condition THEN
    lines
$ELSE
    lines
$END IF
```

`$ELSEIF` may be written `$ELSE IF`, and `$END IF` may be written `$ENDIF`. Any number of `$ELSEIF` branches may
stand before the one `$ELSE`, and both are optional.

## Arguments

- `condition`: names, numbers and comparisons, joined by `AND`, `OR` or `XOR`. It is not a BASIC expression.

## Behavior

Only the first branch whose condition is true is compiled. When none is true, the `$ELSE` branch is compiled, if
there is one.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_conditions; tests\corpus\slice\s23_preprocessor -->

The lines of a branch that is not compiled are not checked at all. They may hold text that is not BASIC, a `SUB`
with no `END SUB`, a `FOR` with no `NEXT`, or a metacommand that does not exist. A `$IF` inside such a branch is
skipped with it, even when its own condition is true.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_ends; verification\v16_m6_unknown_meta; tests\corpus\slice\s23_preprocessor -->

A name in a condition is a name set by `$LET`, or one of these names that are always set:

| Name | Value |
|---|---|
| `WIN`, `WINDOWS` | -1 on Windows, 0 on Linux |
| `LINUX` | -1 on Linux, 0 on Windows |
| `MAC`, `MACOSX`, `32BIT`, `_ARM_` | 0 |
| `64BIT`, `_QB64PE_` | -1 |
| `VERSION` | `4.7.0`, the version of QB64pe the compiler follows |
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_conditions; read: crates\syntax\src\pp.rs:396-414 -->

A name alone is true when it is set to a value other than 0. A name that no `$LET` sets is false, and so is a
number 0. Names and values are compared without regard to letter case: `S = HELLO` is true after
`$LET S = hello`.
<!-- src: verification\v16_m6_conditions; verification\v16_m6_ends -->

The comparisons are `=`, `<>`, `<`, `>`, `<=` and `>=`. `name = DEFINED` is true when a `$LET` has set the name,
and `name = UNDEFINED` when none has. `VERSION` is compared as a version, one number between dots at a time:
`VERSION >= 4.7.0` and `VERSION > 3.0` are true.
<!-- src: verification\v16_m6_conditions; tests\corpus\slice\s23_preprocessor -->

The comparisons are worked out first. Then `AND`, `OR` and `XOR` are worked out from left to right, with no
order among them.
<!-- src: read: crates\syntax\src\pp.rs:630-702; verification\v16_m6_conditions -->

There are no parentheses: `$IF (A = 1) THEN` is false even when `A` is 1. `DEFINED(A)` is false too. Write
`A = 1` and `A = DEFINED`.
<!-- src: verification\v16_m6_parens; verification\v16_m6_defined_func -->

The precompiler flags `_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_` and `_SOCKETS_` may be
named too. Each has the value the whole program gives it, on every line, also above the line that sets it.
`_CONSOLE_` is 1 with `$CONSOLE` and 2 with `$CONSOLE:ONLY` alone. A flag that nothing sets is 0.
<!-- src: compiler/pipeline "Precompiler flags"; verification\v26_h_flag_console; verification\v26_x28_flag_explicit -->

A `$LET` or `$IF` works in file order across included files. An `$INCLUDE` in a branch that is not compiled is not
read.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m7_let_reaches_main; verification\v16_m7_metacommand_form -->

A `$IF` may stand inside a block such as `IF` or `FOR`, and a whole `SUB` may stand inside a branch.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_block_inside; verification\v16_m6_sub_in_meta; tests\corpus\slice\s23_preprocessor -->

Each line of a `$IF` must stand at the start of its line. Spaces before it are allowed.
<!-- src: verification\v16_m6_ends; tests\corpus\slice\s23_preprocessor -->

The lints give no `used-once`, `unused-procedure` or `unused-label` warning for a name used in a branch that is not
compiled, and no `unreachable` warning for the code after a `$IF`, `$ELSEIF`, `$ELSE` or `$END IF` line.
<!-- src: compiler/lints "Names used in a skipped branch"; compiler/lints "Code that may run after all" -->

## Errors

These are compile errors:

- `$IF` with no `THEN` at the end: "`$IF` without `THEN`".
  <!-- src: verification\v16_m6_no_then -->
- `$IF` with no `$END IF`: "`$IF` without `$END IF`", at the `$IF`. An `$END IF` with no `$IF` is "`$END IF`
  without `$IF`".
  <!-- src: verification\v16_m6_missing_end; verification\v26_x40_tab_before_if -->
- A second `$ELSE` in one `$IF`: "this `$IF` already has an `$ELSE`".
  <!-- src: verification\v16_m6_two_else -->
- A condition that is not names, numbers and comparisons, such as `NOT WIN` or a value of two words: "cannot
  evaluate this `$IF` condition (one word or comparison expected per operand)".
  <!-- src: verification\v16_m6_not; verification\v16_m6_let_two_words -->
- A `$IF` line after a statement and a colon: "a metacommand must stand at the start of a line".
  <!-- src: verification\v16_m6_after_colon; verification\v26_x45_if_after_colon -->
- A tab before a `$IF` line: "a metacommand must not have a tab before it".
  <!-- src: verification\v26_x40_tab_before_if; verification\v26_x49_tab_before_if_and_end -->
- A block opened in a branch and closed after the `$END IF`, such as an `IF` header in a branch and its `END IF`
  below: "`$END IF` while the `IF` on line n is still open; expected `END IF` first".
  <!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_one_header; verification\v16_m6_split_block -->

## Differences

On Linux, `WIN` and `WINDOWS` are 0 and `LINUX` is -1, so the other branch of a platform's `$IF` is compiled, as
QB64pe does on Linux.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m6_conditions -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-if_1 -->
```qb64
$LET LEVEL = 2
$IF LEVEL = 1 THEN
    PRINT "Level 1: the short game."
$ELSEIF LEVEL = 2 THEN
    PRINT "Level 2: the long game."
$ELSE
    PRINT "Some other level."
$END IF
$IF TRACE = UNDEFINED THEN
    PRINT "TRACE is not defined, so the trace lines are left out."
$ELSE
    PRINT "Tracing."
$END IF
$IF 64BIT THEN
    PRINT "Compiled for 64 bits."
$END IF
END
```

Output:

```text
Level 2: the long game.
TRACE is not defined, so the trace lines are left out.
Compiled for 64 bits.

Press any key to continue
```
<!-- end generated -->

## See also

[`$LET`](meta-let.md), [`$ERROR`](meta-error.md), [`$INCLUDE`](meta-include.md), [`$CONSOLE`](meta-console.md).
