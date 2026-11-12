# _MOUSEWHEEL

`_MOUSEWHEEL` gives how far the wheel turned at the event `_MOUSEINPUT` last took.
`_MOUSEWHEEL(1)` gives the sideways wheel.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEWHEEL [axis]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A DOUBLE.

## Behavior

With no argument, or 0, it gives the vertical wheel; with any other number, the horizontal one.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:377-392; tests\corpus\screen\u01_mouse_queue -->

In a `$CONSOLE:ONLY` program it reads the console's wheel, which a program with no console input leaves at 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:378-384; tests\reference\examples\_mousewheel_1 -->

## Errors

- `_MOUSEWHEEL(1, 2)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x16_mousewheel_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousewheel_1 -->
```qb64
WHILE _MOUSEINPUT: WEND
PRINT "Wheel:"; _MOUSEWHEEL; _MOUSEWHEEL(1)
END
```

Output:

```text
Wheel: 0  0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEINPUT`, `_WHEEL`.
