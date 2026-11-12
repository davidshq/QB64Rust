# _MOUSEINPUT

`_MOUSEINPUT` takes the next mouse event from the queue. It gives -1 while it takes one and 0 when the queue is empty;
`_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON` and `_MOUSEWHEEL` then give that event.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEINPUT
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

Events come in the order they happened. `DO WHILE _MOUSEINPUT` reads them all; `WHILE _MOUSEINPUT: WEND` skips to the
latest.
<!-- src: language/mouse "The mouse queue"; tests\corpus\screen\u01_mouse_queue -->

A `$CONSOLE:ONLY` program has no window, so its queue stays empty.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:350-358; tests\reference\examples\_mouseinput_1 -->

## Errors

- `_MOUSEINPUT(1)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x101_mouseinput_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mouseinput_1 -->
```qb64
n = 0
DO WHILE _MOUSEINPUT
    n = n + 1
LOOP
PRINT "Mouse events waiting:"; n
END
```

Output:

```text
Mouse events waiting: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEX`, `_MOUSEBUTTON`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX`.
