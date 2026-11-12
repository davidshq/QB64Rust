# _FONTWIDTH

`_FONTWIDTH` gives the width of a character of the current font, and `_FONTHEIGHT` its height, in pixels.

## Syntax

```text
width = _FONTWIDTH[(font)]
height = _FONTHEIGHT[(font)]
```

## Behavior

Font 16 is 8 by 16 pixels and font 8 is 8 by 8.
<!-- src: language/fonts "Built-in fonts"; tests\reference\examples\_fontwidth_1 -->

A font that is not of fixed width gives a `_FONTWIDTH` of 0.
<!-- src: language/fonts "Forms of the font built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_fontwidth_1 -->
```qb64
img& = _NEWIMAGE(100, 40, 32)
_DEST img&
_FONT 16: w16 = _FONTWIDTH: h16 = _FONTHEIGHT
_FONT 8: w8 = _FONTWIDTH: h8 = _FONTHEIGHT
_DEST _CONSOLE
PRINT "Font 16:"; w16; "by"; h16; " font 8:"; w8; "by"; h8
END
```

Output:

```text
Font 16: 8 by 16  font 8: 8 by 8 

Press any key to continue
```
<!-- end generated -->

## See also

[`_FONT`](_font.md), [`_PRINTWIDTH`](_printstring.md).
