# RND

`RND` gives a pseudo-random number from 0 up to, but not including, 1. `INT(RND * 6) + 1` is a die roll from 1 to 6.

## Syntax

<!-- begin generated: syntax -->
```text
RND[(behavior)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `n`, optional: 0 repeats the last value, a negative number restarts the sequence from a seed of its own, and
  anything else, or nothing, gives the next value.

## Result

A SINGLE from 0 up to 1.

## Behavior

`RND` gives the next value of the sequence. Without `RANDOMIZE`, a program gets the same sequence on every run.
<!-- src: verification\v22_d_random; tests\reference\examples\rnd_1 -->

`RND(0)` gives the last value again. `RND(1)`, or any positive argument, is the same as `RND`.
<!-- src: verification\v22_d_random -->

A negative argument starts the sequence again from a value set by the argument: `RND(-1)` twice gives the same value
both times.
<!-- src: verification\v22_d_random -->

The sequence is QB64pe's: after `RANDOMIZE USING 5` or `RANDOMIZE 5`, a program prints the same values as with QB64pe.
<!-- src: language/builtin-statements "RANDOMIZE"; verification\v22_d_random -->

The value is a SINGLE: stored in a DOUBLE it shows the SINGLE's digits, as `.3257257342338562`.
<!-- src: verification\v22_d_random -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:rnd_1 -->
```qb64
RANDOMIZE USING 5
PRINT RND; RND
PRINT "RND(0) repeats the last value:"; RND(0)
FOR i = 1 TO 5
    PRINT INT(RND * 6) + 1;
NEXT i
PRINT
END
```

Output:

```text
 .3257257  .7353894 
RND(0) repeats the last value: .7353894 
 5  2  5  3  6 

Press any key to continue
```
<!-- end generated -->

## See also

[`RANDOMIZE`](randomize.md), [`INT`](int.md), [`TIMER`](timer.md).
