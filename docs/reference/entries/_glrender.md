# _GLRENDER

`_GLRENDER` sets where the drawing of `SUB _GL` goes: `_BEHIND` the program's screen, `_ONTOP` of it, or `_ONLY`, with
no screen.

## Syntax

<!-- begin generated: syntax -->
```text
_GLRENDER {_BEHIND|_ONTOP|_ONLY}
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The layer decides what shows where `SUB _GL` and the screen both draw.
<!-- src: language/opengl "SUB _GL"; verification\v59_h1_behind; verification\v59_h2_only; verification\v59_h3_ontop -->

A window's state is outside what the tests compare, so no test checks what it shows; it compiles to QB64pe's call.
<!-- src: language/console-and-window "The window's display settings"; verification\v58_s05_display_window -->

## Errors

- `_GLRENDER` with no word, or with a number, is a compile error.
  <!-- src: language/console-and-window "The window's display settings"; verification\v58_x38_glrender_bare; verification\v58_x39_glrender_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _GLRENDER _BEHIND -->

## See also

[`SUB _GL`](_gl.md), [`_DISPLAYORDER`](_displayorder.md).
