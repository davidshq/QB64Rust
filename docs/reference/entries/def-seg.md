# DEF SEG

`DEF SEG = segment` sets the segment that `PEEK`, `POKE`, `BLOAD`, `BSAVE` and `CALL ABSOLUTE` use. `DEF SEG` alone
sets it back to the default.

## Syntax

```text
DEF SEG [= segment]
```

## Behavior

The segment of a variable or array is `VARSEG` of it. After `DEF SEG = VARSEG(a(1))`, `PEEK(VARPTR(a(3)))` reads the
first byte of `a(3)`.
<!-- src: language/conventional-memory "DEF SEG, PEEK and POKE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\def-seg_1 -->

The segment is converted as a LONG argument is.
<!-- src: language/conventional-memory "DEF SEG, PEEK and POKE" -->

## Errors

- `DEF SEG 5` (a value and no `=`), `DEF SEG =` (no value) and a string are compile errors.
  <!-- src: language/conventional-memory "DEF SEG, PEEK and POKE" -->

## Availability

<!-- begin generated: availability -->
Supported. All 24 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:def-seg_1 -->
```qb64
DIM a(1 TO 4) AS INTEGER
a(3) = 513
DEF SEG = VARSEG(a(1))
PRINT "The bytes of a(3):"; PEEK(VARPTR(a(3))); PEEK(VARPTR(a(3)) + 1)
DEF SEG
END
```

Output:

```text
The bytes of a(3): 1  2 

Press any key to continue
```
<!-- end generated -->

## See also

[`PEEK`](peek.md), [`POKE`](poke.md), [`VARSEG`](varseg.md), [`BLOAD`](bload.md).
