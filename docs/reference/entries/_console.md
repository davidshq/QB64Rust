# _CONSOLE

`_CONSOLE` is the handle of the console, for `_DEST _CONSOLE`. As a statement, `_CONSOLE OFF` hides the console's
window and `_CONSOLE ON` shows it again.

## Syntax

```text
_CONSOLE
_CONSOLE OFF
_CONSOLE ON
```

## Behavior

The function gives the console's handle, -10. `_DEST _CONSOLE` sends output to the console.
<!-- src: language/console-and-window "_DEST and _SOURCE"; tests\reference\examples\_console_1 -->

In a program with a console of its own, `_CONSOLE OFF` hides the console's window and `_CONSOLE ON` shows it. Output
still goes to the console while it is hidden.
<!-- src: language/console-and-window "The console shown and hidden"; tests\reference\examples\_console_1 -->

In a program without a console both statements do nothing.
<!-- src: language/console-and-window "The console shown and hidden" -->

## Errors

- `_CONSOLE` alone as a statement, `_CONSOLE 1` or `_CONSOLE ON, OFF` is a compile error.
  <!-- src: language/console-and-window "The console shown and hidden" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_console_1 -->
```qb64
PRINT "The console's handle is"; _CONSOLE
_CONSOLE OFF
PRINT "Output still goes to the console while it is hidden."
_CONSOLE ON
END
```

Output:

```text
The console's handle is-10 
Output still goes to the console while it is hidden.

Press any key to continue
```
<!-- end generated -->

## See also

`$CONSOLE`, `_DEST`, `_ECHO`.
