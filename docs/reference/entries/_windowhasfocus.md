# _WINDOWHASFOCUS

`_WINDOWHASFOCUS` tells whether the program's window has the keyboard's focus.

## Syntax

<!-- begin generated: syntax -->
```text
_WINDOWHASFOCUS
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

In a `$CONSOLE:ONLY` program it is 0.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\reference\examples\_windowhasfocus_1 -->

In a program with a window, it is -1 while the window has the keyboard's focus: after the program clicks its own
window with `_SCREENCLICK`, it is -1.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_title_handle; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `x = _WINDOWHASFOCUS(1)`: "`_WINDOWHASFOCUS` takes no arguments".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v71_x07_windowhasfocus_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_windowhasfocus_1 -->
```qb64
PRINT "Focus:"; _WINDOWHASFOCUS
END
```

Output:

```text
Focus: 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_WINDOWHANDLE`](_windowhandle.md), [`_SCREENICON`](_screenicon.md).
