# _CLIPBOARDIMAGE

`_CLIPBOARDIMAGE` gives a new image of the picture on the clipboard, and `_CLIPBOARDIMAGE = handle` puts an image
there.

## Syntax

```text
handle& = _CLIPBOARDIMAGE
_CLIPBOARDIMAGE = handle&
```

## Behavior

No test runs it, since what it reads and writes is the clipboard's; it compiles to QB64pe's call.
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

`_CLIPBOARD$`, `_SCREENIMAGE`.
