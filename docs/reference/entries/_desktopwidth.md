# _DESKTOPWIDTH

`_DESKTOPWIDTH` and `_DESKTOPHEIGHT` give the size of the desktop in pixels.

## Syntax

```text
w = _DESKTOPWIDTH
h = _DESKTOPHEIGHT
```

## Result

A LONG.

## Behavior

In a `$CONSOLE:ONLY` program both are 0.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\reference\examples\_desktopwidth_1 -->

In a program with a window they are the desktop's, so a test does not compare them.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\screen\w01_window_builtins -->

They may be written without parentheses.
<!-- src: language/builtin-functions "Built-ins without parentheses" -->

## Errors

- An argument is a compile error.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x113_desktopwidth_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_desktopwidth_1 -->
```qb64
PRINT "Desktop:"; _DESKTOPWIDTH; _DESKTOPHEIGHT
END
```

Output:

```text
Desktop: 0  0 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SCREENX`](_screenx.md), [`_NEWIMAGE`](_newimage.md), [`_DESKTOPREFRESHRATE`](_desktoprefreshrate.md).
