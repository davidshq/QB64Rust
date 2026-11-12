# _MOUSEBUTTON

`_MOUSEBUTTON(n)` tells whether mouse button `n` was down at the event `_MOUSEINPUT` last took: 1 is the left button,
2 the right and 3 the middle.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEBUTTON(buttonNumber)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

It gives the state at the event taken, so a click is two events: down, then up.
<!-- src: language/mouse "The mouse queue"; tests\corpus\screen\u01_mouse_queue -->

In a `$CONSOLE:ONLY` program, or while `_SOURCE` is the console, it reads the console's mouse instead, which a program
with no console input leaves at 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:360-375; tests\reference\examples\_mousebutton_1 -->

## Errors

- A button outside 1 to 8 raises error 5: `_MOUSEBUTTON(0)` and `_MOUSEBUTTON(9)` do.
  <!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:360-364; tests\reference\examples\_mousebutton_1 -->
- `_MOUSEBUTTON` with no number, or with two, is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x13_mousebutton_bare; verification\v31_x14_mousebutton_two -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousebutton_1 -->
```qb64
ON ERROR GOTO trap
PRINT "Left, right, middle:"; _MOUSEBUTTON(1); _MOUSEBUTTON(2); _MOUSEBUTTON(3)
x = _MOUSEBUTTON(0)
x = _MOUSEBUTTON(9)
END
trap:
PRINT "error"; ERR
RESUME NEXT
```

Output:

```text
Left, right, middle: 0  0  0 
error 5 
error 5 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEINPUT`, `_MOUSEX`, `_BUTTON`.
