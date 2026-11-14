# RANDOMIZE

`RANDOMIZE` seeds the random-number generator, so that `RND` gives another sequence. `RANDOMIZE TIMER` gives a
different one on each run; `RANDOMIZE USING n` gives the same one every time.

## Syntax

```text
RANDOMIZE [seed]
RANDOMIZE USING seed
```

## Arguments

- `seed`: a number of any numeric type.

## Behavior

`RANDOMIZE USING n` restarts the sequence from the seed `n`. The same seed always gives the same values, wherever it
stands in the program.
<!-- src: language/builtin-statements "RANDOMIZE"; verification\v22_d_random; tests\reference\examples\randomize_1 -->

`RANDOMIZE n` seeds the generator with `n` too, but the values also depend on those drawn before it: `RANDOMIZE 5` a
second time gives another sequence than the first.
<!-- src: language/builtin-statements "RANDOMIZE"; verification\v22_d_random -->

`RANDOMIZE TIMER` takes the seed from the clock, so each run gets other values.
<!-- src: language/builtin-statements "RANDOMIZE" -->

`RANDOMIZE` with no seed prints `Random-number seed (-32768 to 32767)? ` and reads the seed from the keyboard.
<!-- src: language/builtin-statements "RANDOMIZE" -->

The seed is taken in the same way whatever its type: `RANDOMIZE USING` of a `_BYTE`, a LONG, a SINGLE or a `_FLOAT`
holding 5 gives the same sequence.
<!-- src: verification\v22_d_random -->

## Errors

- A string as the seed is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:randomize_1 -->
```qb64
RANDOMIZE USING 42
a = RND: b = RND
RANDOMIZE USING 42
PRINT "The same seed gives the same values:"; a = RND; b = RND
RANDOMIZE TIMER
x = RND
PRINT "A value from 0 up to 1:"; x >= 0 AND x < 1
END
```

Output:

```text
The same seed gives the same values:-1 -1 
A value from 0 up to 1:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`RND`](rnd.md), [`TIMER`](timer.md).
