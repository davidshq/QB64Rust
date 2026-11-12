# _RED32

`_RED32`, `_GREEN32`, `_BLUE32` and `_ALPHA32` give the parts of a 32-bit color, each from 0 to 255.

## Syntax

<!-- begin generated: syntax -->
```text
_ALPHA32(rgbaColor&)
_BLUE32(rgbaColor&)
_GREEN32(rgbaColor&)
_RED32(rgbaColor&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An INTEGER from 0 to 255.

## Behavior

Of `_RGBA32(10, 20, 30, 40)` they give 10, 20, 30 and 40.
<!-- src: language/images "The colour functions"; tests\reference\examples\_red32_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_red32_1 -->
```qb64
c~& = _RGBA32(10, 20, 30, 40)
PRINT _RED32(c~&); _GREEN32(c~&); _BLUE32(c~&); _ALPHA32(c~&)
END
```

Output:

```text
 10  20  30  40 

Press any key to continue
```
<!-- end generated -->

## See also

[`_RGB32`](_rgb32.md), [`_RED`](_red.md), [`POINT`](point.md).
