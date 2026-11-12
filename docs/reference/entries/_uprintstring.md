# _UPRINTSTRING

`_UPRINTSTRING (x, y), text$` draws Unicode text at a pixel position. `_UPRINTWIDTH` gives the width it takes.

## Syntax

```text
_UPRINTSTRING (x, y), text$[, maxwidth][, utfencoding][, font][, image]
width = _UPRINTWIDTH(text$[, utfencoding][, font])
```

## Behavior

It draws and measures text as libqb does: `"Hi"` in the default font is 16 pixels wide.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_uprintstring_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_UPRINTSTRING (1, 2)`: "this `_UPRINTSTRING` statement matches none of its forms".
  - `_UPRINTSTRING STEP(1, 2), "a"`: "this `_UPRINTSTRING` statement matches none of its forms".
  - `_UPRINTSTRING (1, 2), "a", 1, 2, 3, 4, 5`: "this `_UPRINTSTRING` statement matches none of its forms".
  <!-- src: language/fonts "Forms of the font built-ins"; verification\v31_x63_uprintstring_no_text; verification\v31_x64_uprintstring_step; verification\v31_x65_uprintstring_eight -->

## Availability

<!-- begin generated: availability -->
Supported. All 15 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_uprintstring_1 -->
```qb64
img& = _NEWIMAGE(80, 30, 32)
_DEST img&
_UPRINTSTRING (2, 2), "Hi"
w = _UPRINTWIDTH("Hi")
_DEST _CONSOLE
PRINT "The text is"; w; "pixels wide and draws"; Lit&(img&); "white pixels."
END

FUNCTION Lit& (img&)
    _SOURCE img&
    n& = 0
    FOR y = 0 TO _HEIGHT(img&) - 1
        FOR x = 0 TO _WIDTH(img&) - 1
            IF POINT(x, y) = _RGB32(255, 255, 255) THEN n& = n& + 1
        NEXT x
    NEXT y
    Lit& = n&
END FUNCTION
```

Output:

```text
The text is 16 pixels wide and draws 64 white pixels.

Press any key to continue
```
<!-- end generated -->

## See also

`_PRINTSTRING`, `_UFONTHEIGHT`, `_UCHARPOS`.
