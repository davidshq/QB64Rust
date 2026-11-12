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

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\screen\w01_window_builtins -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/console-and-window "Window built-ins are compiled" -->

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

`_WINDOWHANDLE`, `_SCREENICON`.
