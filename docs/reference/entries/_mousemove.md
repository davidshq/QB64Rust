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

`_MOUSEMOVE x, y` moves the mouse cursor to that place of the program's screen: a pixel in a graphics screen, a
column and a row in a text screen. After `_MOUSEMOVE 50, 60` in a graphics screen, and once `_MOUSEINPUT` has read
the move, `_MOUSEX` and `_MOUSEY` are 50 and 60. In `SCREEN 0`, `_MOUSEMOVE 10, 5` gives 10 and 5.
<!-- src: language/mouse "Mouse built-ins that no test runs"; tests\sandbox\mousemove_readback -->

It moves the machine's real cursor, so it runs only in the sandbox family, in the test VM, not on the host.
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

[`_MOUSEX`](_mousex.md), [`_MOUSEY`](_mousex.md).
