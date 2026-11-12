# _TITLE

`_TITLE text$` sets the title of the program's window, and `_TITLE$` gives it.

## Syntax

```text
_TITLE text$
title$ = _TITLE$
```

## Behavior

In a `$CONSOLE:ONLY` program, `_TITLE$` gives "".
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\reference\examples\_title_1 -->

The title is any string expression, a fixed-length string included; `CALL _TITLE(s$)` is accepted.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\slice\s66_window_builtins -->

A window's state is outside what the tests compare, so no test checks what it does; it compiles to QB64pe's call.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\corpus\screen\w01_window_builtins -->

## Errors

- `_TITLE` with no text, a number or two texts, and `_TITLE$()` or `_TITLE$` with an argument, are compile errors.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x92_title_none; verification\v26_x93_title_number; verification\v26_x94_title_two; verification\v26_x101_title_func_empty -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_title_1 -->
```qb64
PRINT "Title: ["; _TITLE$; "]"
END
```

Output:

```text
Title: []

Press any key to continue
```
<!-- end generated -->

## See also

`_CONSOLETITLE`, `_ICON`.
