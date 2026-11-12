# _BYTE

`_BYTE` is the type of an 8-bit whole number, from -128 to 127. `_UNSIGNED _BYTE` holds 0 to 255. The suffixes
are `%%` and `~%%`.

## Syntax

```text
DIM name AS _BYTE
DIM name AS _UNSIGNED _BYTE
name%%
name~%%
```

## Behavior

A `_BYTE` takes 1 byte. A value outside the range is cut to its low 8 bits, with no error: 128 stored in a `_BYTE`
reads as -128, and 256 stored in an `_UNSIGNED _BYTE` reads as 0.
<!-- src: language/numeric-semantics "Storing into an integer variable"; verification\v21_a_decls; tests\reference\examples\_byte_1 -->

A floating-point value stored into a `_BYTE` is first narrowed to SINGLE, then rounded half to even.
<!-- src: language/numeric-semantics "Storing into an integer variable" -->

In arithmetic, a `_BYTE` is widened to a 32-bit integer: with `b~%% = 255`, `b~%% + 1` is 256.
<!-- src: language/numeric-semantics "Arithmetic with unsigned, narrow and bit operands" -->

## Errors

- `BYTE` without its underscore is no type: "no type is named `BYTE`".
  <!-- src: verification\v21_x05_byte_no_underscore -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_byte_1 -->
```qb64
DIM b AS _BYTE
b = 127: PRINT "_BYTE 127:"; b
b = 128: PRINT "_BYTE 128:"; b
DIM ub AS _UNSIGNED _BYTE
ub = 255: PRINT "_UNSIGNED _BYTE 255:"; ub; "in"; LEN(ub); "byte"
ub = 256: PRINT "_UNSIGNED _BYTE 256:"; ub
END
```

Output:

```text
_BYTE 127: 127 
_BYTE 128:-128 
_UNSIGNED _BYTE 255: 255 in 1 byte
_UNSIGNED _BYTE 256: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`INTEGER`](integer.md), [`_UNSIGNED`](_unsigned.md), [`_BIT`](_bit.md), [`ASC`](asc.md), [`CHR$`](chr-str.md).
