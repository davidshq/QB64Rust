# _CLIPBOARD$

`_CLIPBOARD$` gives the text on the system's clipboard, and `_CLIPBOARD$ = text$` puts text there.

## Syntax

<!-- begin generated: syntax -->
```text
_CLIPBOARD$ = text$
_CLIPBOARD$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
_CLIPBOARD$ = text$
```

## Behavior

`_CLIPBOARD$ = s$` puts `s$` on the clipboard up to its first NUL.
<!-- src: language/builtin-functions "The system's name and the clipboard"; verification\v49_d_clipboard -->

No test on the host uses it, since it races with Windows' clipboard history; it runs in
`tests\sandbox\clipboard_text`.
<!-- src: tests\corpus\slice\s150_builtins_more -->

## Errors

- An argument, a number assigned, and `CALL _CLIPBOARD$` are compile errors.
  <!-- src: language/builtin-functions "The system's name and the clipboard"; verification\v49_x45_clipboard_arg; verification\v49_x46_clipboard_number; verification\v49_x50_clipboard_call -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _CLIPBOARD$ = "text": t$ = _CLIPBOARD$ -->

## See also

[`_CLIPBOARDIMAGE`](_clipboardimage.md).
