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

`_TITLE text$` sets the title of the program's window: after `_TITLE "QB64Rust sandbox title"`, Windows reads that
text from the window.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_title_handle; tests\corpus\screen\w01_window_builtins -->

In a program with a window, `_TITLE$` cannot be trusted: after `_TITLE "Probe title"` it gave a string of the
title's length, 11, whose bytes were not the title and changed from run to run. QB64pe's build does the same, as
both use libqb's call.
<!-- src: tests\sandbox\README.md -->

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

[`_CONSOLETITLE`](_consoletitle.md), [`_ICON`](_icon.md).
