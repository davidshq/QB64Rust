# SADD

`SADD(s$)` gives the offset of a string's characters in libqb's emulated conventional memory.

## Syntax

<!-- begin generated: syntax -->
```text
SADD(stringVariable$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An offset, as `VARPTR`'s.

## Behavior

After `s = "hello"`, `PEEK(SADD(s))` with `DEF SEG = VARSEG(s)` is 104, the code of `h`.
<!-- src: language/conventional-memory "SADD"; tests\corpus\slice\s158_varptr_strings; tests\reference\examples\sadd_1 -->

`SADD(s$())` gives the first element's characters.
<!-- src: language/conventional-memory "SADD" -->

A variable or array that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program lives in libqb's
emulated conventional memory, in QB64pe's order and sizes, so that `PEEK` and `POKE` reach its bytes.
<!-- src: language/conventional-memory "Variables in conventional memory"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s158_varptr_strings; tests\corpus\slice\s73_conventional_arrays -->

## Errors

- `SADD` of anything but a variable-length string variable or element is a compile error.
  <!-- src: language/conventional-memory "SADD" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:sadd_1 -->
```qb64
DIM s AS STRING
s = "hello"
DEF SEG = VARSEG(s)
PRINT "The first and last characters:"; PEEK(SADD(s)); PEEK(SADD(s) + 4)
DEF SEG
END
```

Output:

```text
The first and last characters: 104  111 

Press any key to continue
```
<!-- end generated -->

## See also

`VARPTR`, `VARSEG`, `PEEK`.
