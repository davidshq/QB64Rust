# _AUTODISPLAY

`_AUTODISPLAY` turns the automatic display back on after `_DISPLAY`, so that drawing shows at once again. The function
gives whether it is on.

## Syntax

```text
_AUTODISPLAY
value = _AUTODISPLAY
```

## Behavior

`_AUTODISPLAY` undoes `_DISPLAY`'s manual display as libqb does, and the function gives whether the automatic display
is on.
<!-- src: language/images "_AUTODISPLAY and PMAP" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: _AUTODISPLAY : a = _AUTODISPLAY -->

## See also

`_DISPLAY`.
