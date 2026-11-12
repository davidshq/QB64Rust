# _SCREENHIDE

`_SCREENHIDE` hides the program's window and `_SCREENSHOW` shows it again. The function `_SCREENHIDE` tells whether
the window is hidden.

## Syntax

```text
_SCREENHIDE
_SCREENSHOW
hidden = _SCREENHIDE
```

## Behavior

The function gives -1 when the window is hidden and 0 when it is not, and 0 in a `$CONSOLE:ONLY` program.
<!-- src: language/console-and-window "The function _SCREENHIDE"; verification\v57_h01_screenhide_value; tests\reference\examples\_screenhide_1 -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\slice\s66_window_builtins -->

## Errors

- An argument to either statement is a compile error.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x95_screenhide_arg; verification\v26_x96_screenshow_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_screenhide_1 -->
```qb64
PRINT "_SCREENHIDE in a console program:"; _SCREENHIDE
END
```

Output:

```text
_SCREENHIDE in a console program: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`$SCREENHIDE`, `_SCREENICON`, `_CONSOLE`.
