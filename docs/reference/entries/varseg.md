# VARSEG

`VARSEG(variable)` gives the segment of a variable or array element in libqb's emulated conventional memory, for `DEF
SEG`.

## Syntax

<!-- begin generated: syntax -->
```text
VARSEG(variable_name[(start_index)])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An `_INTEGER64`.

## Behavior

Every element of an array has the same segment. The arrays follow one another in the order of their `DIM`, each
rounded up to 16 bytes: two INTEGER arrays of 10 elements are 2 paragraphs apart.
<!-- src: language/conventional-memory "VARPTR and VARSEG of an array of the main module"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\varseg_1 -->

A variable's segment is 80.
<!-- src: language/conventional-memory "Variables in conventional memory"; tests\corpus\slice\s158_varptr_strings; tests\reference\examples\varseg_1 -->

Dynamic arrays come from the other end of the memory, at each allocation.
<!-- src: language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s73_conventional_arrays -->

Unlike `VARPTR`, `VARSEG` does not compute or check the index of an element.
<!-- src: tests\corpus\slice\s72_bload_peek_poke -->

## Errors

- An argument that is not a variable or an element is a compile error.
  <!-- src: language/conventional-memory "What VARPTR and VARSEG accept" -->
- A static array larger than 65,536 bytes, or one for which the memory is used up, raises error 257.
  <!-- src: language/conventional-memory "Arrays in conventional memory" -->

## Differences

From QB64pe: a `TYPE` variable of the main module that holds a `STRING` member, and that `VARPTR`, `VARSEG`, `SADD`
or `VARPTR$` names, lives in conventional memory there with its strings never made, so its first use crashes the
program. QB64Rust makes its strings, empty, and keeps QB64pe's place and member offsets
(`DIVERGENCES-QB64PE.md` D-037).
<!-- src: D-037 -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:varseg_1 -->
```qb64
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER
PRINT "Paragraphs from a to b:"; VARSEG(b(1)) - VARSEG(a(1))
PRINT "One segment for the whole array:"; VARSEG(a(1)) = VARSEG(a(7))
PRINT "Segment of a variable:"; VARSEG(n%)
END
```

Output:

```text
Paragraphs from a to b: 2 
One segment for the whole array:-1 
Segment of a variable: 80 

Press any key to continue
```
<!-- end generated -->

## See also

[`VARPTR`](varptr.md), [`DEF SEG`](def-seg.md).
