# FOR

`FOR` repeats statements while a counter steps from a start value to an end value: `FOR i = 1 TO 10 … NEXT i`.

## Syntax

```text
FOR variable = start TO end [STEP step]
    statements
NEXT [variable [, variable]...]
```

## Arguments

- `variable`: a numeric variable, or a numeric member of a `TYPE` variable such as `r.v`.
- `start`, `end`, `step`: numbers. `step` is 1 when it is left out.

## Behavior

`start`, `end` and `step` are evaluated once, in that order, before the first pass. Changing a variable they name
inside the loop does not change the loop.
<!-- src: language/control-flow "FOR loops" -->

The variable is set to `start`, and the body runs while the variable has not passed `end`: is not above it for a
step of 0 or more, not below it for a negative step. `NEXT` adds `step` to the variable's current value, so a change
made in the body counts.
<!-- src: language/control-flow "FOR loops"; tests\reference\examples\for_1 -->

After the loop the variable holds the first value past `end`: `FOR i = 1 TO 3` leaves `i` at 4. A loop whose start
is already past its end does not run, and leaves the variable at `start`.
<!-- src: language/control-flow "FOR loops"; tests\reference\examples\for_1 -->

The test uses a type wider than the variable's, so a loop to the variable's largest value ends: `FOR i% = 32760 TO
32767 STEP 4` ends with `i%` at -32768, its store having wrapped. A floating-point `end` is rounded to that type
first: `FOR i% = 1 TO 2.6` runs for 1, 2 and 3.
<!-- src: language/control-flow "FOR loops" -->

`NEXT` with no variable closes the innermost `FOR`. `NEXT j, i` closes two loops, the inner one first. A variable is
a name plus a type: `NEXT i` does not close `FOR i%`.
<!-- src: language/control-flow "NEXT variables" -->

A `FOR` may be opened in one file and closed by a `NEXT` in a file it includes.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_block_across -->

`EXIT FOR` leaves the loop, and `_CONTINUE` goes on with the next pass.
<!-- src: language/control-flow "EXIT from loops"; language/control-flow "_CONTINUE" -->

## Errors

These are compile errors:

- A string variable, a string `start`, `end` or `step`, a `_BIT` variable, a constant or an array element as the
  variable: "the `FOR` variable `s$` must be a number, not a string" for a string.
  <!-- src: language/control-flow "FOR loops" -->
- A `FOR` with no `NEXT`: "`FOR` without `NEXT`". A `NEXT` with no `FOR`: "`NEXT` without `FOR`".
  <!-- src: compiler/pipeline "Block structure" -->
- A `NEXT` naming another loop's variable, as `NEXT i, j` for loops of `i` and `j` in that order: "`NEXT i` does not
  close `FOR j`".
  <!-- src: language/control-flow "NEXT variables" -->

## Availability

<!-- begin generated: availability -->
Supported. All 131 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:for_1 -->
```qb64
FOR i = 1 TO 3
    PRINT "i ="; i
NEXT i
PRINT "After the loop, i ="; i
FOR n = 10 TO 0 STEP -5
    PRINT n;
NEXT
PRINT
FOR k = 5 TO 1
    PRINT "never"
NEXT k
PRINT "A loop from 5 to 1 does not run; k ="; k
END
```

Output:

```text
i = 1 
i = 2 
i = 3 
After the loop, i = 4 
 10  5  0 
A loop from 5 to 1 does not run; k = 5 

Press any key to continue
```
<!-- end generated -->

## See also

`DO`, `WHILE`, `EXIT`, `_CONTINUE`.
