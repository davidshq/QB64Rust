# ON

`ON n GOTO` and `ON n GOSUB` jump to the n-th label of a list: `ON choice GOSUB first, second, third`. `ON` also
starts other statements, each with its own page: `ON ERROR`, and the event traps such as `ON TIMER`.

## Syntax

```text
ON n GOTO label1 [, label2]...
ON n GOSUB label1 [, label2]...
```

## Arguments

- `n`: a number. A floating-point value is rounded half to even.
- The labels: labels or line numbers of the same body.

## Behavior

The program jumps to the n-th label, or `GOSUB`s to it. `ON 2 GOTO a, b, c` goes to `b`, and `ON 1.5 GOTO a, b`
also goes to `b`.
<!-- src: language/control-flow "ON GOTO and ON GOSUB"; tests\reference\examples\on_1 -->

When `n` is 0 or larger than the number of labels, the program goes on with the next statement. After an
`ON … GOSUB`, `RETURN` goes back to the statement after it.
<!-- src: language/control-flow "ON GOTO and ON GOSUB"; tests\reference\examples\on_1 -->

When computing `n` raises an error, the jump uses the value left behind.
<!-- src: language/control-flow "ON GOTO and ON GOSUB" -->

## Errors

- A negative `n` raises run-time error 5, "Illegal function call".
  <!-- src: language/control-flow "ON GOTO and ON GOSUB" -->
- A label that does not exist in the body is a compile error: "label `nowhere` is not defined in the main module".
  <!-- src: language/control-flow "ON GOTO and ON GOSUB"; language/control-flow "Labels per body" -->

## Differences

From QuickBASIC 4.5: an `n` above 255 raises error 5 there. QB64Rust goes on with the next statement, as for any `n`
beyond the list, as QB64pe does (`DIVERGENCES-QB45.md` Q-001).
<!-- src: Q-001; verification\v20_i_on_goto -->

## Availability

<!-- begin generated: availability -->
Supported. All 18 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:on_1 -->
```qb64
FOR choice = 1 TO 4
    ON choice GOSUB first, second, third
NEXT choice
PRINT "4 is beyond the list, so nothing was called for it."
END

first: PRINT "first": RETURN
second: PRINT "second": RETURN
third: PRINT "third": RETURN
```

Output:

```text
first
second
third
4 is beyond the list, so nothing was called for it.

Press any key to continue
```
<!-- end generated -->

## See also

`GOTO`, `GOSUB`, `SELECT CASE`, `ON ERROR`.
