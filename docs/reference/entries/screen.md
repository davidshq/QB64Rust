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

The modes, as each leaves the screen:

| Mode | Size | Colors | Character height |
|---|---|---|---|
| 0 | 80 by 25 characters (`WIDTH 80, 50` gives 80 by 50) | 16 | 16 pixels (8 at 50 rows) |
| 1 | 320 by 200 pixels | 4 | 8 |
| 2 | 640 by 200 | 2 | 8 |
| 7 | 320 by 200 | 16 | 8 |
| 8 | 640 by 200 | 16 | 8 |
| 9 | 640 by 350 | 16 | 14 |
| 10 | 640 by 350 | 4 | 14 |
| 11 | 640 by 480 | 2 | 16 |
| 12 | 640 by 480 | 16 | 16 |
| 13 | 320 by 200 | 256 | 8 |
| An image handle | The image's own, as `SCREEN _NEWIMAGE(100, 60, 32)` | 256 for depth 256, 32-bit color for 32 | 16 |
<!-- src: language/screen-output "SCREEN"; tests\corpus\screen\c01_text_cells; tests\corpus\screen\m00_width_80_50; tests\corpus\screen\m01_screen_1; tests\corpus\screen\m02_screen_2; tests\corpus\screen\m07_screen_7; tests\corpus\screen\m08_screen_8; tests\corpus\screen\m09_screen_9; tests\corpus\screen\m10_screen_10; tests\corpus\screen\m11_screen_11; tests\corpus\screen\m12_screen_12; tests\corpus\screen\m13_screen_13; tests\corpus\screen\m256_newimage_256; tests\corpus\screen\m32_newimage_32 -->

`SCREEN(row, column)` gives the character code at that cell of the source: after `PRINT "Hi"` at row 2, column 3,
`SCREEN(2, 3)` is 72. With a color flag of 1 it gives the cell's colors.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\screen_1 -->

## Errors

- A mode that does not exist, as `SCREEN 3`, raises error 5.
  <!-- src: language/screen-output "SCREEN"; tests\corpus\screen\d14_errors -->

## Differences

From QuickBASIC 4.5: in `SCREEN 9` and `SCREEN 10` the text uses the video BIOS's 8-by-14 font there. QB64Rust,
as QB64pe, draws rows 1 to 14 of its 8-by-16 font, so a cell of text differs by 3 to 7 pixels
(`DIVERGENCES-QB45.md` Q-023).
<!-- src: Q-023 -->

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

[`_NEWIMAGE`](_newimage.md), [`PCOPY`](pcopy.md), [`WIDTH`](width.md), [`_SOURCE`](_source.md).
