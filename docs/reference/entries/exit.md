# EXIT

`EXIT` leaves a loop, a `SELECT CASE` block or a procedure before its end: `EXIT FOR`, `EXIT DO`, `EXIT WHILE`,
`EXIT SELECT`, `EXIT CASE`, `EXIT SUB`, `EXIT FUNCTION`.

## Syntax

```text
EXIT FOR
EXIT DO
EXIT WHILE
EXIT SELECT
EXIT CASE
EXIT SUB
EXIT FUNCTION
```

## Behavior

`EXIT FOR`, `EXIT DO` and `EXIT WHILE` continue after the innermost open loop of that kind, also from inside other
blocks nested in it. After `EXIT FOR` the loop variable keeps its value.
<!-- src: language/control-flow "EXIT from loops"; tests\reference\examples\exit_1 -->

`EXIT SELECT` continues after the `END SELECT` of the innermost `SELECT` with a `CASE` open around it, also from
inside an `IF` or a loop in that `CASE`. A loop around the `SELECT` goes on.
<!-- src: language/control-flow "EXIT SELECT and EXIT CASE"; tests\reference\examples\exit_1 -->

`EXIT CASE` does the same as `EXIT SELECT` in a plain `SELECT CASE`. In `SELECT EVERYCASE` it goes on with the next
`CASE`.
<!-- src: language/control-flow "EXIT SELECT and EXIT CASE" -->

`EXIT SUB` and `EXIT FUNCTION` return from the procedure at once.
<!-- src: tests\reference\examples\exit_1 -->

## Errors

These are compile errors:

- `EXIT FOR`, `EXIT DO` or `EXIT WHILE` outside such a loop: "`EXIT FOR` without `FOR`", "`EXIT DO` without `DO`",
  "`EXIT WHILE` without `WHILE`".
  <!-- src: language/control-flow "EXIT from loops" -->
- `EXIT SELECT` or `EXIT CASE` with no `CASE` open around it, such as in a SUB called from a `CASE`.
  <!-- src: language/control-flow "EXIT SELECT and EXIT CASE" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:exit_1 -->
```qb64
FOR i = 1 TO 10
    IF i * i > 20 THEN EXIT FOR
NEXT i
PRINT "The first square above 20 is of"; i
SELECT CASE i
    CASE 5
        PRINT "five"
        IF i > 4 THEN EXIT SELECT
        PRINT "not printed"
END SELECT
Check 0
Check 2
END

SUB Check (v)
    IF v = 0 THEN EXIT SUB
    PRINT "v ="; v
END SUB
```

Output:

```text
The first square above 20 is of 5 
five
v = 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`FOR`](for.md), [`DO`](do.md), [`WHILE`](while.md), [`SELECT CASE`](select-case.md), [`_CONTINUE`](_continue.md), [`SUB`](sub.md).
