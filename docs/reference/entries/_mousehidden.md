# _MOUSEHIDDEN

`_MOUSEHIDDEN` tells whether `_MOUSEHIDE` hid the cursor over the program's window.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEHIDDEN
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

In a window whose cursor nothing hid, it is 0.
<!-- src: language/mouse "Forms of the mouse built-ins"; tests\corpus\screen\u02_mouse_text -->

A `$CONSOLE:ONLY` program has no window, so it is always 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:158-165; tests\reference\examples\_mousehidden_1 -->

## Errors

- `_MOUSEHIDDEN(1)` is a compile error, "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins"; verification\v31_x102_mousehidden_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousehidden_1 -->
```qb64
PRINT "Hidden:"; _MOUSEHIDDEN
END
```

Output:

```text
Hidden: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEHIDE`, `_MOUSESHOW`, `_MOUSEDISABLED`.
