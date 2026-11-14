# _DEPTHBUFFER

`_DEPTHBUFFER` turns the depth buffer of the 3-D form of `_MAPTRIANGLE` on or off for an image, locks it, or clears
it.

## Syntax

<!-- begin generated: syntax -->
```text
_DEPTHBUFFER {ON|OFF|LOCK|_CLEAR}[,handle&]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

A screen program runs it; what it draws is not compared.
<!-- src: language/images "Image built-ins that no test runs"; tests\corpus\screen\w02_depthbuffer -->

It compiles to QB64pe's call, and links the part of libqb it needs.
<!-- src: language/images "Image built-ins that no test runs" -->

## Errors

- `_DEPTHBUFFER` with no word, or with a number, is a compile error.
  <!-- src: language/images "Image built-ins that no test runs"; verification\v31_x43_depthbuffer_bare; verification\v31_x44_depthbuffer_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _DEPTHBUFFER ON -->

## See also

[`_MAPTRIANGLE`](_maptriangle.md), [`SUB _GL`](_gl.md).
