# WHILE

`WHILE … WEND` repeats statements while a condition is true, testing it before each pass. `DO WHILE … LOOP` does
the same.

## Syntax

```text
WHILE condition
    statements
WEND
```

## Arguments

- `condition`: a number. 0 is false, any other value is true.

## Behavior

The condition is tested before each pass, and the body runs while it is true. When it is false at the start, the
body does not run.
<!-- src: language/control-flow "DO and WHILE loops"; tests\reference\examples\while_1 -->

`EXIT WHILE` leaves the loop, and `_CONTINUE` goes on with the next test.
<!-- src: language/control-flow "EXIT from loops"; language/control-flow "_CONTINUE" -->

## Errors

These are compile errors:

- A string condition: "the condition of `WHILE` must be a number, not a string".
  <!-- src: language/control-flow "DO and WHILE loops" -->
- A `WHILE` with no `WEND`: "`WHILE` without `WEND`". A `WEND` with no `WHILE`: "`WEND` without `WHILE`".
  <!-- src: compiler/pipeline "Block structure" -->

## Differences

From BASIC PDS 7.1: in a `WHILE` condition whose sides are comparisons, it may skip the right side of `AND` or
`OR`. QB64Rust evaluates both sides, as QB 4.5 and QB64pe do (`DIVERGENCES-QB45.md` Q-026).
<!-- src: Q-026 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:while_1 -->
```qb64
count = 3
WHILE count > 0
    PRINT "count ="; count
    count = count - 1
WEND
PRINT "Done."
END
```

Output:

```text
count = 3 
count = 2 
count = 1 
Done.

Press any key to continue
```
<!-- end generated -->

## See also

`DO`, `FOR`, `EXIT`.
