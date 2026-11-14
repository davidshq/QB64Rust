# _UCHARPOS

`_UCHARPOS(text$, positions())` fills an array with the pixel position of each character of a text, and gives the
number of characters.

## Syntax

```text
count = _UCHARPOS(text$, positions&()[, utfencoding][, font])
```

## Behavior

For `"abc"` in the default font the positions are 0, 8 and 16, and the result is 3.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_ucharpos_1 -->

The array is a LONG or `_UNSIGNED LONG` array, whole or from an element.
<!-- src: language/fonts "_UCHARPOS takes a LONG array" -->

## Errors

- Any other array is a compile error: "Expected LONG array-name".
  <!-- src: language/fonts "_UCHARPOS takes a LONG array" -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_ucharpos_1 -->
```qb64
img& = _NEWIMAGE(80, 30, 32)
_DEST img&
DIM p(1 TO 3) AS LONG
n = _UCHARPOS("abc", p())
_DEST _CONSOLE
PRINT n; "characters at"; p(1); p(2); p(3)
END
```

Output:

```text
 3 characters at 0  8  16 

Press any key to continue
```
<!-- end generated -->

## See also

[`_UPRINTSTRING`](_uprintstring.md), [`_UPRINTWIDTH`](_uprintstring.md).
