# _OFFSET

`_OFFSET` is the type of a memory address, 64 bits wide. `_OFFSET(variable)` is the function that gives a
variable's address. The suffix of the type is `%&`.

## Syntax

<!-- begin generated: syntax -->
```text
_OFFSET(variable)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
DIM name AS _OFFSET
DIM name AS _UNSIGNED _OFFSET
name%&
name~%&
```

## Arguments

- `variable`: a variable, an array element or an array.

## Result

The function gives an `_UNSIGNED _OFFSET`.

## Behavior

An `_OFFSET` takes 8 bytes and holds the values of a signed 64-bit integer. `_UNSIGNED _OFFSET`, suffix `~%&`,
holds those of an unsigned one.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_decls; verification\v24_a_define -->

`_OFFSET(variable)` gives the address of the variable, the element or the array. It is the same address as the
`OFFSET` of a `_MEM` block of the variable.
<!-- src: language/mem-blocks "The address of a reference"; tests\reference\examples\_offset_1 -->

With an `_OFFSET` operand, `+`, `-`, `\`, `MOD` and the bit-by-bit operators work on integers. `*` with a
floating-point operand and `/` are computed in `_FLOAT` and rounded half to even. The result is an `_OFFSET`,
unsigned unless an `_OFFSET` operand is signed.
<!-- src: language/numeric-semantics "_OFFSET operands" -->

## Errors

These are compile errors:

- `^` with an `_OFFSET` operand: "`^` cannot be used with an `_OFFSET`".
  <!-- src: language/numeric-semantics "_OFFSET operands"; verification\v21_x28_offset_power -->
- The suffix `%&` after a number, as in `5%&`: "an `_OFFSET` suffix cannot follow a number".
  <!-- src: language/numeric-semantics "Integer literal typing"; verification\v21_x10_offset_literal -->
- `_OFFSET` of a value that is no variable, as in `_OFFSET(5)`: "`_OFFSET` needs a variable, an element or an
  array". `_OFFSET` of a `_BIT` array is an error too.
  <!-- src: language/mem-blocks "The address of a reference" -->

## Availability

<!-- begin generated: availability -->
Supported. All 24 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_offset_1 -->
```qb64
DIM a AS LONG
DIM o AS _OFFSET
o = _OFFSET(a)
PRINT "An _OFFSET takes"; LEN(o); "bytes"
PRINT "The address of a is not 0:"; o <> 0
END
```

Output:

```text
An _OFFSET takes 8 bytes
The address of a is not 0:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_MEM`](_mem.md), [`_MEMGET`](_memget.md), [`_INTEGER64`](_integer64.md).
