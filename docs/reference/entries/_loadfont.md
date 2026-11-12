# _LOADFONT

`_LOADFONT(name$, size)` loads a font file and gives a handle for `_FONT`. `_FREEFONT` frees it.

## Syntax

```text
handle = _LOADFONT(name$, size[, options$][, index])
_FREEFONT handle
```

## Behavior

On Windows, a bare name is looked for in the font folders libqb searches, the user's and the system's. The options and
the index are taken as libqb takes them, and text drawn in the font is what QB64pe's program draws.
<!-- src: language/fonts "Fonts loaded from Windows" -->

`_FREEFONT` frees the font.
<!-- src: language/fonts "Fonts loaded from Windows" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/fonts "Forms of the font built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: f& = _LOADFONT("arial.ttf", 20) : IF f& > 0 THEN _FREEFONT f& -->

## See also

`_FONT`, `_UPRINTSTRING`.
