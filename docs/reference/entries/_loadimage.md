# _LOADIMAGE

`_LOADIMAGE(name$, mode)` loads an image file and gives its handle, or -1 when it cannot.

## Syntax

<!-- begin generated: syntax -->
```text
_LOADIMAGE(fileName$[, [mode&][, requirements$]])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `name$`: the file.
- `mode`: 32, 256, or left out for the screen's mode.

## Result

A LONG handle, or -1.

## Behavior

`_LOADIMAGE` loads the file as libqb does, into the mode its second argument names.
<!-- src: language/images "Loading and saving images"; tests\reference\examples\_saveimage_1 -->

A file that cannot be loaded gives -1, with no error.
<!-- src: tests\reference\examples\_loadimage_1 -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/images "Forms of the image built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_loadimage_1 -->
```qb64
h& = _LOADIMAGE("no such picture.png", 32)
PRINT "A file that cannot be loaded gives"; h&
END
```

Output:

```text
A file that cannot be loaded gives-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_SAVEIMAGE`, `_PUTIMAGE`, `_FREEIMAGE`.
