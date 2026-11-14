# _SHL

`_SHL(value, count)` shifts the bits of a number left, and `_SHR` right: `_SHL(1, 4)` is 16 and `_SHR(256, 4)` is 16.
A shift left by one doubles a number.

## Syntax

<!-- begin generated: syntax -->
```text
_SHL(numericalVariable, numericalValue)
_SHR(numericalVariable, numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `value`: an integer.
- `count`: how many bits to shift.

## Result

An `_UNSIGNED _INTEGER64`.

## Behavior

The value is shifted as an `_UNSIGNED _INTEGER64`, whatever its type, so a negative value shifts as its 64 bits:
`_SHR(-1, 1)` is 9223372036854775807.
<!-- src: language/builtin-functions "Shifts and rotations"; tests\reference\examples\_shl_1 -->

The count is taken modulo 64: `_SHL(1, 65)` is 2, and `_SHL(1, 64)` is 1.
<!-- src: language/builtin-functions "Shifts and rotations"; language/builtin-functions "Bit functions" -->

## Errors

- A float or a string as an argument, or a `_BIT` type, is a compile error.
  <!-- src: language/builtin-functions "Shifts and rotations" -->
- `_SHL` and `_SHR` in a `CONST` are compile errors.
  <!-- src: language/builtin-functions "Not in a CONST" -->

## Differences

From QB64pe: a constant shift count of 64 or more gives a value there that depends on its C++ compiler. QB64Rust takes
it modulo 64 (`DIVERGENCES-QB64PE.md` D-053).
<!-- src: D-053 -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_shl_1 -->
```qb64
PRINT _SHL(1, 4); _SHL(3, 2)
PRINT _SHR(256, 4); _SHR(7, 1)
END
```

Output:

```text
 16  12 
 16  3 

Press any key to continue
```
<!-- end generated -->

## See also

[`_ROL`](_rol.md), [`_SETBIT`](_setbit.md), [`AND`](and.md).
