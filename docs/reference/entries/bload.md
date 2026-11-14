# BLOAD

`BLOAD file$, offset` reads a file written by `BSAVE` to an offset in the `DEF SEG` segment. Without the offset it
reads to the one the file was saved from.

## Syntax

<!-- begin generated: syntax -->
```text
BLOAD fileName$, VARPTR(imageArray%(index))
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A file `BSAVE` wrote from one array, read back to another, gives the second the same values.
<!-- src: language/conventional-memory "BLOAD and BSAVE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\bload_1 -->

With no offset it uses the one in the file's header, in the current segment.
<!-- src: language/conventional-memory "BLOAD and BSAVE"; tests\corpus\slice\s72_bload_peek_poke -->

## Errors

- A file that does not exist raises error 53.
  <!-- src: language/conventional-memory "BLOAD and BSAVE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\bload_1 -->
- A block that runs past the end of libqb's conventional memory raises error 5, and nothing is read.
  <!-- src: language/conventional-memory "BLOAD and BSAVE past the block"; D-049 -->

## Differences

QB64pe wrote or read past the end of its block of conventional memory there.
<!-- src: language/conventional-memory "BLOAD and BSAVE past the block"; D-049 -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:bload_1 -->
```qb64
ON ERROR GOTO trap
DIM a(1 TO 5) AS INTEGER, b(1 TO 5) AS INTEGER
FOR i = 1 TO 5: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "bload_1.tmp", VARPTR(a(1)), 10
DEF SEG = VARSEG(b(1))
BLOAD "bload_1.tmp", VARPTR(b(1))
DEF SEG
FOR i = 1 TO 5: PRINT b(i);: NEXT: PRINT
KILL "bload_1.tmp"
BLOAD "no such file.tmp", VARPTR(b(1))
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
 11  22  33  44  55 
error 53 

Press any key to continue
```
<!-- end generated -->

## See also

[`BSAVE`](bsave.md), [`DEF SEG`](def-seg.md).
