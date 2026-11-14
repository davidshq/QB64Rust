# DO

`DO … LOOP` repeats statements. The test may stand at the top, at the bottom, or nowhere, and may be a `WHILE`
(go on while true) or an `UNTIL` (go on until true).

## Syntax

```text
DO [WHILE condition | UNTIL condition]
    statements
LOOP [WHILE condition | UNTIL condition]
```

## Arguments

- `condition`: a number. 0 is false, any other value is true.

## Behavior

`DO WHILE c` tests before each pass and runs the body while `c` is true. `DO UNTIL c` runs it while `c` is false.
Either may not run the body at all.
<!-- src: language/control-flow "DO and WHILE loops"; tests\reference\examples\do_1 -->

`LOOP WHILE c` and `LOOP UNTIL c` test after each pass, so the body runs at least once.
<!-- src: language/control-flow "DO and WHILE loops"; tests\reference\examples\do_1 -->

A `DO … LOOP` with no condition repeats until `EXIT DO` or a jump leaves it.
<!-- src: language/control-flow "DO and WHILE loops"; language/control-flow "EXIT from loops"; tests\reference\examples\do_1 -->

`_CONTINUE` goes on with the next pass; a test at the bottom still runs.
<!-- src: language/control-flow "_CONTINUE" -->

## Errors

These are compile errors:

- A string condition, as in `LOOP UNTIL "a"`: "the condition of `LOOP` must be a number, not a string".
  <!-- src: language/control-flow "DO and WHILE loops" -->
- A `DO` with no `LOOP`: "`DO` without `LOOP`". A `LOOP` with no `DO`: "`LOOP` without `DO`".
  <!-- src: compiler/pipeline "Block structure" -->

## Differences

From BASIC PDS 7.1: in a `DO WHILE`, `DO UNTIL` or `LOOP WHILE` condition whose sides are comparisons, it may skip
the right side of `AND` or `OR`. QB64Rust evaluates both sides, as QB 4.5 and QB64pe do (`DIVERGENCES-QB45.md`
Q-026).
<!-- src: Q-026 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:do_1 -->
```qb64
n = 1
DO WHILE n < 100
    n = n * 3
LOOP
PRINT "First power of 3 from 100 up:"; n
tries = 0
DO
    tries = tries + 1
LOOP UNTIL tries >= 4
PRINT "tries ="; tries
DO
    m = m + 1
    IF m = 3 THEN EXIT DO
LOOP
PRINT "left the loop at m ="; m
END
```

Output:

```text
First power of 3 from 100 up: 243 
tries = 4 
left the loop at m = 3 

Press any key to continue
```
<!-- end generated -->

## See also

[`WHILE`](while.md), [`FOR`](for.md), [`EXIT`](exit.md), [`_CONTINUE`](_continue.md).
