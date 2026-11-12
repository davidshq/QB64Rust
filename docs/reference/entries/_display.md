# _DISPLAY

`_DISPLAY` shows what has been drawn and turns off the automatic display, so that a program draws a whole frame before
it is seen. The function `_DISPLAY` gives the handle of the image on display.

## Syntax

```text
_DISPLAY
handle = _DISPLAY
```

## Behavior

The statement compiles to QB64pe's call, and a program that uses it leaves the screen QB64pe's leaves. `_AUTODISPLAY`
undoes it.
<!-- src: language/screen-output "Pacing statements"; language/images "_AUTODISPLAY and PMAP" -->

The function gives the handle of the image on display.
<!-- src: language/screen-output "_BLINK and the function _DISPLAY" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: _DISPLAY : h& = _DISPLAY -->

## See also

`_AUTODISPLAY`, `_LIMIT`, `PCOPY`.
