# CLEAR

`CLEAR` sets every variable of the main module and every `STATIC` and `SHARED` variable to zero or empty, releases
dynamic arrays and closes the open files.

## Syntax

```text
CLEAR [a][, [b][, c]]
```

## Behavior

After `CLEAR`, a `SHARED` LONG is 0, an element of a static array 0 and a string empty.
<!-- src: language/builtin-statements "CLEAR"; verification\v49_e_clear; tests\reference\examples\clear_1 -->

It also resets the `DATA` position, the error handler and the `RETURN` points.
<!-- src: language/builtin-statements "CLEAR"; verification\v49_e2_clear_gosub -->

Its arguments, QuickBASIC's memory sizes, are evaluated and ignored. `CALL CLEAR` is `CLEAR`.
<!-- src: language/builtin-statements "The forms of CLEAR" -->

## Errors

- `CLEAR` in a SUB or FUNCTION, with a parenthesis among its arguments, with a fourth argument or with a string is a
  compile error.
  <!-- src: language/builtin-statements "The forms of CLEAR"; verification\v49_x30_clear_in_sub; verification\v49_x32_clear_parens -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:clear_1 -->
```qb64
DIM SHARED total AS LONG
DIM a(1 TO 3) AS INTEGER
total = 5: a(2) = 7: s$ = "text"
CLEAR
PRINT "After CLEAR:"; total; a(2); "["; s$; "]"
END
```

Output:

```text
After CLEAR: 0  0 []

Press any key to continue
```
<!-- end generated -->

## See also

[`ERASE`](erase.md), [`RUN`](run.md), [`RESTORE`](restore.md).
