# Numbers

Each number has a type, from `_BIT` to `_FLOAT`, and each operator computes in a width that depends on its operands'
types. The rules are QB64pe's, which are not always QuickBASIC's.

The pages of this area: [INTEGER](../entries/integer.md), [LONG](../entries/long.md),
[_INTEGER64](../entries/_integer64.md), [SINGLE](../entries/single.md), [DOUBLE](../entries/double.md),
[_FLOAT](../entries/_float.md), [_BYTE](../entries/_byte.md), [_BIT](../entries/_bit.md),
[_OFFSET](../entries/_offset.md), [_UNSIGNED](../entries/_unsigned.md), [AND](../entries/and.md),
[OR](../entries/or.md), [XOR](../entries/xor.md), [NOT](../entries/not.md), [EQV](../entries/eqv.md),
[IMP](../entries/imp.md), [MOD](../entries/mod.md), [CINT](../entries/cint.md), [CLNG](../entries/clng.md),
[_NEGATE](../entries/_negate.md).

## Literals

An integer literal without a suffix has the smallest of INTEGER, LONG and `_INTEGER64` that holds it; `-2147483648` is
an `_INTEGER64`.
<!-- src: language/numeric-semantics "Integer literal typing" -->

## Arithmetic

`+`, `-` and `*` on integers compute in 32 bits, or 64 with an `_INTEGER64`, and wrap on overflow with no error.
<!-- src: language/numeric-semantics "Integer arithmetic width" -->

`/` always gives a float; between integers it computes in `_FLOAT` and prints with DOUBLE's 16 digits. `\` and `MOD`
round their operands half to even, then truncate toward zero.
<!-- src: language/numeric-semantics "Division"; language/numeric-semantics "Integer division and MOD"; tests\reference\examples\mod_1 -->

A divisor of 0 in `\` or `MOD` raises error 11, which `ON ERROR` traps; in QB64pe's build it ends the program.
<!-- src: language/numeric-semantics "Integer division and MOD"; D-052 -->

`^` is computed left to right at run time, and in a `CONST` right to left.
<!-- src: language/numeric-semantics "Power"; language/constants "Warning for a chained power in a CONST" -->

## Storing and printing

A float stored into an integer variable is rounded half to even, as `CINT` rounds: `CINT(2.5)` is 2 and `CINT(3.5)` is
4.
<!-- src: language/numeric-semantics "Storing into an integer variable"; tests\reference\examples\cint_1 -->

`PRINT` of a number writes `STR$` of the value in the type it was computed in, followed by one space.
<!-- src: language/numeric-semantics "Printing numbers" -->

`NOT`, `AND`, `OR`, `XOR`, `EQV` and `IMP` work bit by bit on integers, a float first rounded to an integer. A true
comparison is -1, all bits set, so `AND` of two true comparisons is -1.
<!-- src: language/numeric-semantics "Logical operators"; tests\reference\examples\and_1 -->
