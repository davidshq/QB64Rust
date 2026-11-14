# The screen

A program with a window prints and draws on its screen, in one of QuickBASIC's screen modes or on an image of its own.
Text statements place the cursor and set colors; drawing statements draw by pixel.

The pages of this area: [SCREEN](../entries/screen.md), [CLS](../entries/cls.md), [COLOR](../entries/color.md),
[LOCATE](../entries/locate.md), [WIDTH](../entries/width.md), [VIEW PRINT](../entries/view-print.md),
[PRINT](../entries/print.md), [PSET](../entries/pset.md), [LINE](../entries/line.md), [CIRCLE](../entries/circle.md),
[PAINT](../entries/paint.md), [DRAW](../entries/draw.md), [VIEW](../entries/view.md), [WINDOW](../entries/window.md),
[PALETTE](../entries/palette.md), [PCOPY](../entries/pcopy.md), [POINT](../entries/point.md),
[PMAP](../entries/pmap.md), [CSRLIN](../entries/csrlin.md), [POS](../entries/pos.md), [TAB](../entries/tab.md),
[SPC](../entries/spc.md), [_CONTROLCHR](../entries/_controlchr.md), [_DISPLAY](../entries/_display.md),
[_AUTODISPLAY](../entries/_autodisplay.md), [_BLINK](../entries/_blink.md),
[_COPYPALETTE](../entries/_copypalette.md), [_DEFAULTCOLOR](../entries/_defaultcolor.md), [_FPS](../entries/_fps.md),
[_HSB32](../entries/_hsb32.md), [_HUE32](../entries/_hue32.md), [_PALETTECOLOR](../entries/_palettecolor.md),
[_PRINTIMAGE](../entries/_printimage.md), [_RED](../entries/_red.md).

## Modes and text

`SCREEN mode` selects one of the legacy modes 0, 1, 2 and 7 to 13, or an image made with `_NEWIMAGE`. `CLS`, `COLOR`,
`LOCATE`, `WIDTH` and `VIEW PRINT` change the text screen.
<!-- src: language/screen-output "SCREEN"; language/screen-output "Text-screen statements"; tests\reference\examples\locate_1 -->

`TAB(n)` moves to column `n`, on the next line when the cursor is past it, and `SPC(n)` prints `n` blanks.
<!-- src: language/screen-output "TAB and SPC in text output"; tests\reference\examples\tab_1 -->

## Drawing

`PSET`, `LINE`, `CIRCLE`, `PAINT` and `DRAW` draw on the destination; `VIEW` and `WINDOW` set the area and the
coordinates. `POINT` reads a pixel back.
<!-- src: language/screen-output "Drawing statements"; language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\line_1 -->

Graphics `GET` and `PUT` copy a rectangle of the screen into a numeric array and back.
<!-- src: language/screen-output "Graphics GET and PUT" -->

## How the tests see the screen

A console program has no screen, so the examples draw on an image made with `_NEWIMAGE` and print what `POINT` or
`SCREEN` read back from it.
<!-- src: tests\reference\examples\pset_1; tests\reference\examples\locate_1 -->
