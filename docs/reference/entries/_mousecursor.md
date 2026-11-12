# _MOUSECURSOR

`_MOUSECURSOR image, (x, y)` makes an image the mouse cursor over the program's window, with the point `(x, y)` of the
image as its hot spot.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSECURSOR imageHandle& [, (hotspotX&, hotspotY&)]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

Without the hot spot, the image's top left corner is the hot spot.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:137-156 -->

The cursor is outside the screen's state, so the tests run it but do not compare its effect.
<!-- src: language/mouse "Mouse built-ins that no test runs"; tests\corpus\screen\w01_window_builtins -->

In a `$CONSOLE:ONLY` program it does nothing.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:137-156 -->

## Errors

- `_MOUSECURSOR` with no image, or with `(2)` for the hot spot, is a compile error.
  <!-- src: verification\v26_x84_mousecursor_none; verification\v26_x86_mousecursor_one_in_pair -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: img& = _NEWIMAGE(16, 16, 32) : _MOUSECURSOR img&, (8, 8) -->

## See also

`_MOUSESHOW`, `_NEWIMAGE`.
