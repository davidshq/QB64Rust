# _MOUSESHOW

`_MOUSESHOW` shows the mouse cursor over the program's window, with an optional shape. `_MOUSEHIDE` hides it, and
`_MOUSEHIDE _DISABLE` also holds it in the window.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEHIDE [_DISABLE]
_MOUSESHOW [cursorShape$]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_MOUSESHOW` takes a shape's name, in any case: `"DEFAULT"` or `"ARROW"`, `"LINK"`, `"TEXT"`, `"CROSSHAIR"`,
`"VERTICAL"`, `"HORIZONTAL"`, `"TOPLEFT_BOTTOMRIGHT"`, `"TOPRIGHT_BOTTOMLEFT"`, `"WAIT"` or `"CYCLE"`, and a few other
names libqb takes.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:90-135 -->

The cursor is outside the screen's state, so the tests run these statements but do not compare their effect. A program
goes on after them as QB64pe's does.
<!-- src: language/mouse "Mouse built-ins that no test runs"; tests\corpus\screen\w01_window_builtins -->

In a `$CONSOLE:ONLY` program they do nothing.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:77-135 -->

## Errors

- Another shape's name raises error 5 in a program with a window.
  <!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:128-130 -->
- `_MOUSESHOW "a", "b"` and `_MOUSEHIDE _SHOW` are compile errors.
  <!-- src: verification\v26_x81_mouseshow_two; verification\v26_x83_mousehide_word -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: _MOUSESHOW : _MOUSESHOW "CROSSHAIR" : _MOUSEHIDE : _MOUSEHIDE _DISABLE -->

## See also

[`_MOUSEHIDDEN`](_mousehidden.md), [`_MOUSEDISABLED`](_mousedisabled.md), [`_MOUSECURSOR`](_mousecursor.md).
