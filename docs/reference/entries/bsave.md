# BSAVE

`BSAVE file$, offset, length` writes `length` bytes from an offset in the `DEF SEG` segment to a file, after a header
of 7 bytes.

## Syntax

<!-- begin generated: syntax -->
```text
BSAVE saveFile$, VARPTR(array(index)), fileSize&
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`BSAVE` of 20 bytes makes a file of 27 bytes.
<!-- src: language/conventional-memory "BLOAD and BSAVE"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\bsave_1 -->

`BLOAD` reads such a file back.
<!-- src: language/conventional-memory "BLOAD and BSAVE"; tests\corpus\slice\s72_bload_peek_poke -->

## Errors

- A block that runs past the end of libqb's conventional memory raises error 5, and nothing is written.
  <!-- src: language/conventional-memory "BLOAD and BSAVE past the block"; D-049 -->

## Differences

QB64pe wrote or read past the end of its block of conventional memory there.
<!-- src: language/conventional-memory "BLOAD and BSAVE past the block"; D-049 -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:bsave_1 -->
```qb64
DIM a(1 TO 10) AS INTEGER
FOR i = 1 TO 10: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "bsave_1.tmp", VARPTR(a(1)), 20
DEF SEG
OPEN "bsave_1.tmp" FOR BINARY AS #1
PRINT "20 bytes and a header of 7:"; LOF(1)
CLOSE #1
KILL "bsave_1.tmp"
END
```

Output:

```text
20 bytes and a header of 7: 27 

Press any key to continue
```
<!-- end generated -->

## See also

`BLOAD`, `DEF SEG`, `VARPTR`.
