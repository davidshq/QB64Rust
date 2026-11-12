# VARPTR

`VARPTR(variable)` gives the offset of a variable or array element in libqb's emulated conventional memory, for
`PEEK`, `POKE`, `BLOAD`, `BSAVE` and `CALL ABSOLUTE`.

## Syntax

<!-- begin generated: syntax -->
```text
VARPTR(variable_name[(reference_index%)])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An `_INTEGER64`.

## Behavior

For an array element, it is the element's index from the first one times the size of an element: `VARPTR(a(2))` of an
INTEGER array from 1 is 2, `VARPTR(c(3))` of a LONG array from 0 is 12.
<!-- src: language/conventional-memory "VARPTR and VARSEG of an array of the main module"; tests\corpus\slice\s72_bload_peek_poke; tests\reference\examples\varptr_1 -->

The index is computed and checked as in a read of the element: a FUNCTION in it runs once, and an index out of range
raises error 9.
<!-- src: language/conventional-memory "VARPTR and VARSEG of an array of the main module"; tests\corpus\slice\s72_bload_peek_poke -->

For a variable it is the variable's offset, in QB64pe's order and sizes: the first INTEGER of `s158_varptr_strings` is
at 65518.
<!-- src: language/conventional-memory "Variables in conventional memory"; tests\corpus\slice\s158_varptr_strings -->

A variable or array that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program lives in libqb's
emulated conventional memory, in QB64pe's order and sizes, so that `PEEK` and `POKE` reach its bytes.
<!-- src: language/conventional-memory "Variables in conventional memory"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s158_varptr_strings; tests\corpus\slice\s73_conventional_arrays -->

A parameter's `VARPTR` is that of where its argument lies, so every call's argument goes to conventional memory too.
<!-- src: language/conventional-memory "Parameters whose address is taken" -->

`HEX$(VARPTR(i))` and the like give QB64pe's string.
<!-- src: language/conventional-memory "VARPTR and VARSEG in HEX$, OCT$ and _BIN$"; tests\corpus\slice\s158_varptr_strings -->

## Errors

- A literal, an expression or a function call is a compile error, as is `VARPTR` of a `_BIT` array, of a whole `TYPE`
  variable or of the `STRING` member of an element.
  <!-- src: language/conventional-memory "What VARPTR and VARSEG accept" -->
- `VARPTR` of a whole array `x()` that is not of numbers is reported "not supported yet".
  <!-- src: language/conventional-memory "What VARPTR and VARSEG accept" -->
- A plain variable passed to a parameter whose address is taken is a compile error.
  <!-- src: language/conventional-memory "Variables that cannot be passed to such a parameter" -->

## Differences

From QB64pe: a `TYPE` variable of the main module that holds a `STRING` member, and that `VARPTR`, `VARSEG`, `SADD`
or `VARPTR$` names, lives in conventional memory there with its strings never made, so its first use crashes the
program. QB64Rust makes its strings, empty, and keeps QB64pe's place and member offsets
(`DIVERGENCES-QB64PE.md` D-037).
<!-- src: D-037 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:varptr_1 -->
```qb64
DIM a(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG
PRINT "Offsets of a(1), a(2), a(10):"; VARPTR(a(1)); VARPTR(a(2)); VARPTR(a(10))
PRINT "Offset of c(3):"; VARPTR(c(3))
END
```

Output:

```text
Offsets of a(1), a(2), a(10): 0  2  18 
Offset of c(3): 12 

Press any key to continue
```
<!-- end generated -->

## See also

[`VARSEG`](varseg.md), [`VARPTR$`](varptr-str.md), [`SADD`](sadd.md), [`PEEK`](peek.md), [`_OFFSET`](_offset.md).
