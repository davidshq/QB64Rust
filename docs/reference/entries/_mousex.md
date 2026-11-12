# _MOUSEX

`_MOUSEX` and `_MOUSEY` give where the mouse was at the event `_MOUSEINPUT` last took: a column and a row in a text
mode, a pixel in a graphics mode.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEX
_MOUSEY
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

In a graphics mode they give the pixel, and in a text mode the column and row, both kept inside the screen.
<!-- src: language/mouse "Position in text and graphics modes"; tests\corpus\screen\u01_mouse_queue; tests\corpus\screen\u02_mouse_text -->

In a `$CONSOLE:ONLY` program, or while `_SOURCE` is the console, it reads the console's mouse instead, which a program
with no console input leaves at 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:268-348; tests\reference\examples\_mousex_1 -->

## Errors

- `_MOUSEX(1)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x15_mousex_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousex_1 -->
```qb64
WHILE _MOUSEINPUT: WEND
PRINT "Console mouse at"; _MOUSEX; _MOUSEY
END
```

Output:

```text
Console mouse at 0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEINPUT`, `_MOUSEMOVE`, `_MOUSEMOVEMENTX`.
