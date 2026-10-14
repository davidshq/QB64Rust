$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): the colour functions, in a palette mode and in 32 bits, in a
' program with a hidden window. Each value is taken while the screen is the destination and printed to the console.
DIM r AS _UNSIGNED LONG, r4 AS LONG, g4 AS LONG, b4 AS LONG, a4 AS LONG, ra AS _UNSIGNED LONG
SCREEN 13
r = _RGB(255, 0, 0): r4 = _RED(4): g4 = _GREEN(4): b4 = _BLUE(4): a4 = _ALPHA(4): ra = _RGBA(0, 0, 255, 128)
_DEST _CONSOLE
PRINT "SCREEN 13:"; r; r4; g4; b4; a4; ra
_DEST 0
h& = _NEWIMAGE(8, 8, 32)
_DEST _CONSOLE
PRINT "32 bits:"; HEX$(_RGB(255, 0, 0, h&)); " "; HEX$(_RGBA(1, 2, 3, 4, h&)); _RED(&HFF102030, h&); _ALPHA(&H80102030, h&)
PRINT "32-bit parts:"; _RED32(&HFF102030); _GREEN32(&HFF102030); _BLUE32(&HFF102030); _ALPHA32(&H80102030)
PRINT "RGB32:"; HEX$(_RGB32(1, 2, 3)); " "; HEX$(_RGBA32(1, 2, 3, 4)); " "; HEX$(_RGB32(5)); " "; HEX$(_RGB32(5, 6))
PRINT "HSB32:"; HEX$(_HSB32(120, 50, 50)); " "; HEX$(_HSBA32(120, 50, 50, 50))
PRINT "hue, saturation, brightness:"; _HUE32(_RGB32(10, 200, 30)); _SATURATION32(_RGB32(10, 200, 30)); _BRIGHTNESS32(_RGB32(10, 200, 30))
PRINT "types:"; _RGB(1, 2, 3, h&) + 1; _RED32(-1) + 1; _HUE32(-1) / 3; -_RGB32(1)
' Out of range: clamped, and a fraction rounded.
PRINT "ranges:"; HEX$(_RGB32(300, -5, 2.5)); " "; HEX$(_HSB32(400, 150, -5)); _RED32(1.5)
SYSTEM
