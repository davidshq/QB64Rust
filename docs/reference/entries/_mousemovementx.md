# _MOUSEMOVEMENTX

`_MOUSEMOVEMENTX` and `_MOUSEMOVEMENTY` give how far the mouse moved at the event `_MOUSEINPUT` last took, in pixels.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEMOVEMENTX
_MOUSEMOVEMENTY
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

The movement of a program's first event counts from wherever the cursor was, so only later events give a movement
known in advance.
<!-- src: tests\corpus\screen\u01_mouse_queue -->

In a `$CONSOLE:ONLY` program, or while `_SOURCE` is the console, it reads the console's mouse instead, which a program
with no console input leaves at 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:176-186; tests\reference\examples\_mousemovementx_1 -->

## Errors

- `_MOUSEMOVEMENTX(1)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x103_mousemovementx_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousemovementx_1 -->
```qb64
WHILE _MOUSEINPUT: WEND
PRINT "Movement:"; _MOUSEMOVEMENTX; _MOUSEMOVEMENTY
END
```

Output:

```text
Movement: 0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEINPUT`, `_MOUSEX`, `_MOUSEHIDE`.
