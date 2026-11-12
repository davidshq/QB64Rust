# _PRINTSTRING

`_PRINTSTRING (x, y), text$` draws text at a pixel position, in the current font. `_PRINTWIDTH` gives the width the
text takes.

## Syntax

```text
_PRINTSTRING (x, y), text$[, image]
width = _PRINTWIDTH(text$[, image])
```

## Behavior

`_PRINTSTRING (2, 2), "Hi"` draws the two characters with their top left at (2, 2), and `_PRINTWIDTH("Hi")` is 16 in
font 16.
<!-- src: language/fonts "Built-in fonts"; tests\reference\examples\_printstring_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_PRINTSTRING "a"`: "this `_PRINTSTRING` statement matches none of its forms".
  - `_PRINTSTRING (1, 2), 5`: "argument 3 of `_PRINTSTRING` needs a string".
  - `_PRINTSTRING (1, 2)`: "this `_PRINTSTRING` statement matches none of its forms".
  <!-- src: language/fonts "Forms of the font built-ins"; verification\v31_x57_printstring_no_coords; verification\v31_x58_printstring_number_text; verification\v31_x59_printstring_no_text -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_printstring_1 -->
```qb64
img& = _NEWIMAGE(60, 20, 32)
_DEST img&
_PRINTSTRING (2, 2), "Hi"
w = _PRINTWIDTH("Hi")
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

`_UPRINTSTRING`, `_FONT`, `_PRINTMODE`.
