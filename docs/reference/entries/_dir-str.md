# _DIR$

`_DIR$("desktop")` gives the path of one of the user's folders: the desktop, documents and others the system knows.

## Syntax

<!-- begin generated: syntax -->
```text
_DIR$(folderspecification)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The folder's name, such as `"desktop"` or `"documents"`.

## Result

A STRING, the folder's path.

## Behavior

`_DIR$` gives the path QB64pe's runtime gives for the name, ending with the folder separator.
<!-- src: language/builtin-functions "The system's folders, commands and build"; verification\v53_c_system -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: d$ = _DIR$("desktop") -->

## See also

`_CWD$`, `_STARTDIR$`.
