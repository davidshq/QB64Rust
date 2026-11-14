# _CLIPBOARDIMAGE

`_CLIPBOARDIMAGE` gives a new image of the picture on the clipboard, and `_CLIPBOARDIMAGE = handle` puts an image
there.

## Syntax

```text
handle& = _CLIPBOARDIMAGE
_CLIPBOARDIMAGE = handle&
```

## Behavior

`_CLIPBOARDIMAGE = img&` puts a copy of the image on the clipboard, and the function `_CLIPBOARDIMAGE` gives a new
image of what the clipboard holds, a handle below -1. A 4 by 4 32-bit image put there comes back with its size
and the red, green and blue of every pixel unchanged.
<!-- src: language/images "Image built-ins that no test runs"; tests\sandbox\clipboardimage_roundtrip -->

It uses the real clipboard, so it runs only in the sandbox family, in the test VM, not on the host.
<!-- src: language/images "Image built-ins that no test runs" -->

## Errors

- The statement without `=`, and the function with an argument, are compile errors.
  <!-- src: language/images "Image built-ins that no test runs"; verification\v31_x79_clipboardimage_no_equals; verification\v31_x80_clipboardimage_fn_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN h& = _CLIPBOARDIMAGE: _CLIPBOARDIMAGE = h& -->

## See also

[`_CLIPBOARD$`](_clipboard-str.md), [`_SCREENIMAGE`](_screenimage.md).
