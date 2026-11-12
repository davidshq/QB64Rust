# _FONT

`_FONT handle` chooses the font text is drawn in: 8, 14 or 16 for a built-in font, or a handle from `_LOADFONT`.
`_FONT` as a function gives the current one.

## Syntax

```text
_FONT handle[, image]
handle = _FONT[(image)]
```

## Behavior

A new image uses font 16. After `_FONT 8`, `_FONT` is 8.
<!-- src: language/fonts "Built-in fonts"; tests\reference\examples\_font_1 -->

The built-in fonts 8, 14 and 16 need no font file.
<!-- src: language/fonts "Built-in fonts" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/fonts "Forms of the font built-ins" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_font_1 -->
```qb64
img& = _NEWIMAGE(100, 40, 32)
_DEST img&
f0 = _FONT
_FONT 8
f8 = _FONT
_DEST _CONSOLE
PRINT "The default font is"; f0; "and after _FONT 8 it is"; f8
END
```

Output:

```text
The default font is 16 and after _FONT 8 it is 8 

Press any key to continue
```
<!-- end generated -->

## See also

`_LOADFONT`, `_FONTWIDTH`, `_PRINTSTRING`.
