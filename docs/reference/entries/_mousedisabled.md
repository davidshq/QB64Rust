# _MOUSEDISABLED

`_MOUSEDISABLED` tells whether `_MOUSEHIDE _DISABLE` took the cursor, hiding it and holding it in the window so that
only its movement counts.

## Syntax

<!-- begin generated: syntax -->
```text
_MOUSEDISABLED
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

In a window whose cursor nothing disabled, it is 0.
<!-- src: language/mouse "Forms of the mouse built-ins"; tests\corpus\screen\u02_mouse_text -->

A `$CONSOLE:ONLY` program has no window, so it is always 0.
<!-- src: read: libqb\internal\c\libqb\src\mouse.cpp:167-174; tests\reference\examples\_mousedisabled_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example "Incorrect number of arguments".
  <!-- src: language/mouse "Forms of the mouse built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mousedisabled_1 -->
```qb64
PRINT "Disabled:"; _MOUSEDISABLED
END
```

Output:

```text
Disabled: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_MOUSEHIDE`, `_MOUSEMOVEMENTX`, `_MOUSEHIDDEN`.
