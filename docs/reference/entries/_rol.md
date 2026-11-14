# _ROL

`_ROL(value, count)` turns the bits of a number to the left, the bits that leave on one side coming back on the other;
`_ROR` turns them to the right. The width is that of the value's type.

## Syntax

<!-- begin generated: syntax -->
```text
_ROL(numericalVariable, numericalValue)
_ROR(numericalVariable, numericalValue)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `value`: an integer.
- `count`: how many bits to turn.

## Result

An unsigned integer of the value's width.

## Behavior

The bits turn within the width of the first argument's type: with `b~%% = 129`, an `_UNSIGNED _BYTE`, `_ROL(b~%%, 1)`
is 3 and `_ROR(b~%%, 1)` is 192. `_ROR` of the INTEGER 1 is 32768.
<!-- src: language/builtin-functions "Shifts and rotations"; tests\reference\examples\_rol_1 -->

A literal counts by the size of its type, and a turn by the whole width gives the value back: `_ROR(70000, 32)` is
70000.
<!-- src: language/builtin-functions "Shifts and rotations" -->

## Errors

- A float or a string as an argument, or a `_BIT` type, is a compile error.
  <!-- src: language/builtin-functions "Shifts and rotations" -->
- `_ROL` and `_ROR` in a `CONST` are compile errors.
  <!-- src: language/builtin-functions "Not in a CONST" -->

## Availability

<!-- begin generated: availability -->
Supported. All 20 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_rol_1 -->
```qb64
b~%% = 129
PRINT _ROL(b~%%, 1); _ROR(b~%%, 1)
i% = 1
PRINT _ROR(i%, 1)
END
```

Output:

```text
 3  192 
 32768 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SHL`](_shl.md), [`_SETBIT`](_setbit.md).
