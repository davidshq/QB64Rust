# PEEK

`PEEK(offset)` gives the byte at an offset in the segment set by `DEF SEG`, from 0 to 255.

## Syntax

<!-- begin generated: syntax -->
```text
PEEK(segment_offset)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG.

## Behavior

An INTEGER of `&H1234` holds the bytes 52 and 18, the low byte first.
<!-- src: language/conventional-memory "DEF SEG, PEEK and POKE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\peek_1 -->

A variable or array that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program lives in libqb's
emulated conventional memory, in QB64pe's order and sizes, so that `PEEK` and `POKE` reach its bytes.
<!-- src: language/conventional-memory "Variables in conventional memory"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s158_varptr_strings; tests\corpus\slice\s73_conventional_arrays -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `l = PEEK("a")`: "`PEEK` needs a number".
  - `l = PEEK(1, 2)`: "`PEEK` takes 1 argument".
  <!-- src: language/conventional-memory "DEF SEG, PEEK and POKE"; verification\v28_x42_peek_string; verification\v28_x44_peek_two_arguments -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:peek_1 -->
```qb64
DIM a(1 TO 4) AS INTEGER
a(2) = &H1234
DEF SEG = VARSEG(a(1))
PRINT "Low byte, high byte:"; PEEK(VARPTR(a(2))); PEEK(VARPTR(a(2)) + 1)
DEF SEG
END
```

Output:

```text
Low byte, high byte: 52  18 

Press any key to continue
```
<!-- end generated -->

## See also

[`POKE`](poke.md), [`DEF SEG`](def-seg.md), [`VARPTR`](varptr.md), [`_MEMGET`](_memget.md).
