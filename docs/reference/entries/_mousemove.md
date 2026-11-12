# _MOUSEMOVE

`_MOUSEMOVE x, y` moves the mouse cursor to a place on the screen: a column and a row in a text mode, a pixel in a
graphics mode.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEMOVE x, y
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

It moves the machine's cursor, so no test runs it; it compiles to QB64pe's call.
<!-- src: language/mouse "Mouse built-ins that no test runs"; tests\corpus\screen\u01_mouse_queue -->

The place is checked against the screen: a column and row from 1 to the screen's width and height, or a pixel inside
the screen.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:188-232 -->

## Errors

- A place outside the screen raises error 5.
  <!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:188-266 -->
- `_MOUSEMOVE 10`, with one number, is a compile error.
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x17_mousemove_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _MOUSEMOVE 10, 20 -->

## See also

`_MOUSEX`, `_MOUSEY`.
