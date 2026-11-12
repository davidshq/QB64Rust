# _CONSOLEINPUT

`_CONSOLEINPUT` reads the next event of the console's input: a key or a mouse event. `_CINP` then gives the key.

## Syntax

```text
value = _CONSOLEINPUT
```

## Behavior

`_CONSOLEINPUT` and `_CINP` compile to QB64pe's calls and give what its program gives. With no input waiting they give
what `verification\v58_f13_console_input` records.
<!-- src: language/console-and-window "The console's own statements and functions"; verification\v58_f13_console_input -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: x = _CONSOLEINPUT -->

## See also

`_CINP`, `INKEY$`, `$CONSOLE`.
