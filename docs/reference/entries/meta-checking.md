# $CHECKING

`$CHECKING:OFF` turns off the run-time checks of the lines below it, so that they run faster: no array bounds, no
check for errors and events. `$CHECKING:ON` turns them back on.

## Syntax

```text
$CHECKING:OFF
$CHECKING:ON
```

## Behavior

Each holds for the lines after it in the order of the source text, through SUBs, FUNCTIONs and included files, until
the next one. A program starts with checking on.
<!-- src: language/run-time-checks "The CHECKING metacommand" -->

Below `$CHECKING:OFF`, array indexes are not checked against the bounds. Indexes within the bounds give the same
elements as with checking on.
<!-- src: language/run-time-checks "Indexes without checks"; tests\reference\examples\meta-checking_1 -->

An error raised by an unchecked statement waits until a checked statement runs, and is handled there. An event waits
the same way.
<!-- src: language/run-time-checks "Statements without checks" -->

With the fix `checking-on-service` on ([Fixes](../chapters/fixes.md)), such an error is handled before the first
checked statement that runs after the unchecked ones, so `RESUME NEXT` goes on at that statement and `RESUME` runs
it.
<!-- src: language/run-time-checks "Fix checking-on-service" -->

`_MEMGET`, `_MEMPUT` and `_MEMFILL` there do not check their block.
<!-- src: language/run-time-checks "Memory statements without checks" -->

It does not change the value of any expression, the tests of a block, `ON ERROR` or `RESUME`.
<!-- src: language/run-time-checks "What CHECKING leaves alone" -->

## Errors

- Any other form, as `$CHECKING:MAYBE`: "`$CHECKING` takes `:OFF` or `:ON`, alone on its line".
  <!-- src: language/run-time-checks "The CHECKING metacommand" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-checking_1 -->
```qb64
DIM a(1 TO 3) AS LONG
$CHECKING:OFF
FOR i = 1 TO 3
    a(i) = i * 10
NEXT i
$CHECKING:ON
PRINT a(1); a(2); a(3)
END
```

Output:

```text
 10  20  30 

Press any key to continue
```
<!-- end generated -->

## See also

[`$ASSERTS`](meta-asserts.md), [`_MEMGET`](_memget.md).
