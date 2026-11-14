# _SCREENPRINT

`_SCREENPRINT text$` types text into the window that has the focus, as if from the keyboard.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENPRINT text$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_SCREENPRINT text$` types the text as keys into the window in front. A program that has clicked its own window
and runs `_SCREENPRINT "abc" + CHR$(13)` reads the four codes 97, 98, 99 and 13 back with `INKEY$`.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\sandbox\screenprint_inkey -->

It types with the real keyboard, so it runs only in the sandbox family, in the test VM, not on the host.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

## Errors

- A form QB64pe rejects is a compile error: no text or a number. For example:
  - `x = _SCREENPRINT("a")`: "`_SCREENPRINT(...)`: `_SCREENPRINT` is a built-in statement".
  - `_SCREENPRINT`: "`_SCREENPRINT` takes 1 argument".
  - `_SCREENPRINT 1`: "`_SCREENPRINT` needs a string".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x56_screenprint_bare; verification\v58_x57_screenprint_number; verification\v58_x103_screenprint_fn -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _SCREENPRINT "abc" -->

## See also

[`_SCREENCLICK`](_screenclick.md).
