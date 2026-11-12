# SCREEN

`SCREEN mode` chooses the screen mode: 0 for text, 12 for 640 by 480 in 16 colors, 13 for 320 by 200 in 256, or an
image handle. The function `SCREEN(row, column)` reads a character of a text screen.

## Syntax

```text
SCREEN mode[, [colorswitch][, [activepage][, visualpage]]]
code = SCREEN(row, column[, colorflag])
```

## Behavior

`SCREEN` selects the mode and the pages as QB64pe's program does, for the modes 0, 1, 2 and 7 to 13 and for an image
handle. A mode the runtime rejects raises its error.
<!-- src: language/screen-output "SCREEN" -->

`SCREEN(row, column)` gives the character code at that cell of the source: after `PRINT "Hi"` at row 2, column 3,
`SCREEN(2, 3)` is 72. With a color flag of 1 it gives the cell's colors.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\screen_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 35 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:screen_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
LOCATE 2, 3: PRINT "Hi";
_DEST _CONSOLE
PRINT SCREEN(2, 3); SCREEN(2, 4); CHR$(SCREEN(2, 3)); CHR$(SCREEN(2, 4))
END
```

Output:

```text
 72  105 Hi

Press any key to continue
```
<!-- end generated -->

## See also

`_NEWIMAGE`, `PCOPY`, `WIDTH`, `_SOURCE`.
