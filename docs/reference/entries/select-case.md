# SELECT CASE

`SELECT CASE` runs one block of statements out of several, chosen by comparing a value with the items of each
`CASE`. `SELECT EVERYCASE` runs every block whose items match.

## Syntax

```text
SELECT CASE selector
CASE item[, item]...
    statements
[CASE item[, item]...
    statements]...
[CASE ELSE
    statements]
END SELECT
```

`SELECT EVERYCASE selector` takes the same `CASE` lines. An item is one of:

- `value`: matches when the selector equals `value`;
- `low TO high`: matches from `low` to `high`, both included;
- `IS op value`: matches when `selector op value` is true, where `op` is `=`, `<>`, `<`, `<=`, `>` or `>=`.

A `CASE` line may also be written on one line with its statements after a colon: `CASE 1: PRINT "one"`.

## Behavior

`SELECT CASE` tests the `CASE` lines in order and runs the statements of the first one with an item that matches.
When none matches, it runs the statements of `CASE ELSE`, if there is one. Then it goes on after `END SELECT`.
<!-- src: language/control-flow "SELECT CASE"; verification\v20_h_select -->

`SELECT EVERYCASE` runs the statements of every `CASE` with a matching item, in order. It runs `CASE ELSE` only when
no `CASE` matched.
<!-- src: language/control-flow "SELECT EVERYCASE"; verification\v16_m4_loop_forms -->

The selector may be a number of any type or a string: `CASE "a" TO "m"` matches `"dog"`. A fixed-length string
selector is compared with its padding spaces.
<!-- src: language/control-flow "SELECT CASE"; language/fixed-length-strings "Assignment to a fixed-length string" -->

The selector is computed once. A selector that is a plain variable is the exception: it is read again for each
test, so a `CASE` item that calls a FUNCTION which changes the variable changes what later items are compared with.
<!-- src: language/control-flow "SELECT CASE"; verification\v20_h_select -->

An item is compared with the selector in the selector's type. A fractional item against an integer selector is first
rounded half to even: `CASE 2.4` matches a selector of 2. An integer item against an unsigned selector is compared
as written, so `CASE -1` matches an `_UNSIGNED LONG` holding 4294967295 but not an `_UNSIGNED INTEGER` holding 65535.
<!-- src: language/control-flow "SELECT CASE"; verification\v21_d_select -->

`EXIT SELECT` leaves the innermost `SELECT` whose `CASE` it stands in, also from inside an `IF` or a loop in that
`CASE`. `EXIT CASE` does the same in `SELECT CASE`; in `SELECT EVERYCASE` it goes on with the next `CASE` line.
<!-- src: language/control-flow "EXIT SELECT and EXIT CASE"; verification\v30_f_exit_select; verification\v30_x62_exit_case_in_plain_select -->

The selector of a `SELECT CASE` is held in one place per `SELECT` statement, not one per call. In a recursive
procedure, a call made from a `CASE` item before the matching one changes the value that the later items are
compared with.
<!-- src: language/control-flow "SELECT CASE"; verification\v04_accidental; Q-002 -->

When an item raises a run-time error under a handler that resumes with the next statement, the item counts as
matching and its `CASE` runs, as a test that raises does in `IF`.
<!-- src: language/control-flow "Errors in SELECT CASE" -->

## Errors

These are compile errors:

- A string item for a numeric selector, or a number for a string selector, also after `IS` or `TO`.
  <!-- src: language/control-flow "SELECT CASE"; verification\v20_x16_select_mismatch; verification\v20_x17_select_string_number; verification\v20_x25_select_is_string_number; verification\v20_x26_select_to_mixed -->
- A `CASE` after `CASE ELSE`, and a second `CASE ELSE`.
  <!-- src: verification\v20_x31_case_after_else; verification\v20_x32_case_else_twice -->
- A statement between `SELECT CASE` and the first `CASE`. Only comments may stand there.
  <!-- src: verification\v16_m4_select_stmt_before_case; verification\v68_e3_select_before -->
- `SELECT CASE` without `END SELECT`.
  <!-- src: verification\v16_m4_end_select_missing -->
- `EXIT SELECT` or `EXIT CASE` with no `CASE` open around it: outside any `SELECT`, before the first `CASE`, or in a
  SUB called from a `CASE`.
  <!-- src: language/control-flow "EXIT SELECT and EXIT CASE" -->

With `-w`, a `SELECT CASE` with no `CASE` at all gets the warning "Empty SELECT CASE block".
<!-- src: compiler/lints "QB64pe's warnings"; verification\v68_e1_select_empty -->

## Differences

From QuickBASIC 4.5: QB 4.5 keeps a copy of the selector for each call of a procedure, so a recursive call does not
change it. QB64Rust keeps one copy per statement, as QB64pe does (`DIVERGENCES-QB45.md` Q-002).

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:select-case_1 -->
```qb64
FOR score = 0 TO 100 STEP 25
    SELECT CASE score
        CASE 0
            PRINT score; "nothing yet"
        CASE 1 TO 49
            PRINT score; "keep going"
        CASE 50, 70
            PRINT score; "a round number"
        CASE IS >= 100
            PRINT score; "full marks"
        CASE ELSE
            PRINT score; "nearly there"
    END SELECT
NEXT

word$ = "dog"
SELECT CASE word$
    CASE "a" TO "m": PRINT word$; " is in the first half of the alphabet"
    CASE ELSE: PRINT word$; " is in the second half"
END SELECT

' SELECT EVERYCASE runs every CASE that matches.
SELECT EVERYCASE 12
    CASE IS > 10: PRINT "more than 10"
    CASE IS > 5: PRINT "more than 5"
    CASE IS > 20: PRINT "more than 20"
END SELECT
END
```

Output:

```text
 0 nothing yet
 25 keep going
 50 a round number
 75 nearly there
 100 full marks
dog is in the first half of the alphabet
more than 10
more than 5

Press any key to continue
```
<!-- end generated -->

## See also

`IF`, `ON … GOTO`, `EXIT`.
