# _UFONTHEIGHT

`_UFONTHEIGHT` gives the height of a font as `_UPRINTSTRING` draws it, and `_ULINESPACING` the distance between its
lines.

## Syntax

```text
height = _UFONTHEIGHT[(font)]
spacing = _ULINESPACING[(font)]
```

## Behavior

The default font is 16 pixels high, with lines 16 pixels apart.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_ufontheight_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_ufontheight_1 -->
```qb64
img& = _NEWIMAGE(80, 30, 32)
_DEST img&
h = _UFONTHEIGHT: s = _ULINESPACING
_DEST _CONSOLE
PRINT "The font is"; h; "pixels high, with"; s; "between lines."
END
```

Output:

```text
The font is 16 pixels high, with 16 between lines.

Press any key to continue
```
<!-- end generated -->

## See also

[`_UPRINTSTRING`](_uprintstring.md), [`_FONTHEIGHT`](_fontwidth.md).
