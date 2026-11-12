# POKE

`POKE offset, value` stores one byte at an offset in the segment set by `DEF SEG`.

## Syntax

<!-- begin generated: syntax -->
```text
POKE segment_offset, offset_value
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Two `POKE`s of 65 and 1 into an INTEGER make it 321. Only the low byte of the value is stored: `POKE` of 300 stores
44, and of -1 stores 255.
<!-- src: language/conventional-memory "DEF SEG, PEEK and POKE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\poke_1 -->

A value with a fraction is rounded first: `POKE s, 7.6` stores 8.
<!-- src: tests\corpus\slice\s72_bload_peek_poke -->

A variable or array that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program lives in libqb's
emulated conventional memory, in QB64pe's order and sizes, so that `PEEK` and `POKE` reach its bytes.
<!-- src: language/conventional-memory "Variables in conventional memory"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s158_varptr_strings; tests\corpus\slice\s73_conventional_arrays -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `POKE 1`: "`POKE` takes 2 arguments".
  <!-- src: language/conventional-memory "DEF SEG, PEEK and POKE"; verification\v28_x43_poke_one_argument -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:poke_1 -->
```qb64
DIM a(1 TO 4) AS INTEGER
DEF SEG = VARSEG(a(1))
POKE VARPTR(a(1)), 65
POKE VARPTR(a(1)) + 1, 1
POKE VARPTR(a(2)), 300
DEF SEG
PRINT "a(1) is 1 * 256 + 65:"; a(1)
PRINT "POKE keeps the low byte of 300:"; a(2)
END
```

Output:

```text
a(1) is 1 * 256 + 65: 321 
POKE keeps the low byte of 300: 44 

Press any key to continue
```
<!-- end generated -->

## See also

`PEEK`, `DEF SEG`, `VARPTR`, `_MEMPUT`.
