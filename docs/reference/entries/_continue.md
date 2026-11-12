# _CONTINUE

`_CONTINUE` skips the rest of a loop's body and goes on with the next pass of the innermost `FOR`, `DO` or `WHILE`
loop.

## Syntax

```text
_CONTINUE
```

## Behavior

`_CONTINUE` goes on with the next pass of the innermost open loop of the current body, whatever blocks lie
between, such as an `IF` or a `SELECT CASE`.
<!-- src: language/control-flow "_CONTINUE"; tests\reference\examples\_continue_1 -->

The loop's own test still runs. In a `FOR` loop the step is added and the limit tested. In a `DO … LOOP UNTIL c` the
test at the bottom runs, and in a `DO WHILE` or `WHILE` loop the test at the top.
<!-- src: language/control-flow "_CONTINUE" -->

It may stand after `THEN` or `ELSE` of a single-line `IF`, and in a `GOSUB` subroutine inside a loop.
<!-- src: language/control-flow "_CONTINUE"; verification\v47_x24; verification\v47_x25 -->

## Errors

These are compile errors:

- `_CONTINUE` outside a loop, also in a SUB whose only loop is around the SUB's call: "`_CONTINUE` outside a `DO`,
  `FOR` or `WHILE` loop".
  <!-- src: language/control-flow "_CONTINUE"; verification\v47_x01; verification\v47_x02 -->
- `_CONTINUE` with anything after it, as in `_CONTINUE 1`.
  <!-- src: language/control-flow "_CONTINUE"; verification\v47_x03 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_continue_1 -->
```qb64
FOR i = 1 TO 6
    IF i MOD 2 = 0 THEN _CONTINUE
    PRINT i;
NEXT i
PRINT
END
```

Output:

```text
 1  3  5 

Press any key to continue
```
<!-- end generated -->

## See also

`EXIT`, `FOR`, `DO`, `WHILE`.
