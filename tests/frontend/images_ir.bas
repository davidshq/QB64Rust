' TEST: ir
' The image statements of m3-images-fonts-mouse in the IR (design D1, each call measured from the old compiler's
' C++ and compared by tests\callsite; verification\v31_x01_accepted): one built-in operation each, its slots and
' its passed mask. _PUTIMAGE in every form of its template (each pair and STEP, the two images, _SMOOTH, each part
' left out), _MAPTRIANGLE in its 2-D and 3-D forms and its words, _CLEARCOLOR with _NONE, _SETALPHA with TO, the
' others with and without their handle; after CALL the same operations. The Settings line names the image codec,
' which _LOADIMAGE and _SAVEIMAGE need (design D2)
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM img AS LONG, t AS STRING
img = _LOADIMAGE("a.bmp", 32)
_SAVEIMAGE "a.png"
_SAVEIMAGE "a.png", img
_SAVEIMAGE "a.png", , "bmp"
_SAVEIMAGE t, s, t
_PUTIMAGE
_PUTIMAGE (10, 10), img
_PUTIMAGE (0, 0)-(99, 99), img, 0
_PUTIMAGE STEP(5, 5), img
_PUTIMAGE , img, 0, (0, 0)-(9, 9)
_PUTIMAGE (0, 0)-(9, 9), img, , , _SMOOTH
_PUTIMAGE (0, 0)-STEP(9, 9), img
_PUTIMAGE STEP(1, 1)-STEP(9, 9), img
_PUTIMAGE , img
_PUTIMAGE , , 0
_PUTIMAGE , img, 0, (1, 1)
_PUTIMAGE , img, 0, STEP(1, 1)-STEP(3, 3)
_PUTIMAGE (1, 2), img, 0, (1, 1)-(3, 3), _SMOOTH
_PUTIMAGE , , , (1, 1)-(3, 3)
_PUTIMAGE , , , , _SMOOTH
_PUTIMAGE (i, l)-(q, s), d, f, (u, u)-(i, i)
_PUTIMAGE (1, 2)-(3, 4)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), 0, _SMOOTH
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), , _SMOOTHSHRUNK
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), , _SMOOTHSTRETCHED
_MAPTRIANGLE _SEAMLESS(0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE _CLOCKWISE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE _ANTICLOCKWISE _SEAMLESS (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9) TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (i, l)-(q, s)-(d, f), u TO (i, l)-(q, s)-(d, f), u
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10, -1)-(10, 50, -1), 0, _SMOOTH
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10)-(10, 50)
_CLEARCOLOR
_CLEARCOLOR 4
_CLEARCOLOR 4, img
_CLEARCOLOR _NONE
_CLEARCOLOR _NONE, img
_CLEARCOLOR _NONE 4
_CLEARCOLOR d, q
_SETALPHA 128
_SETALPHA 128, , img
_SETALPHA 128, 5
_SETALPHA 128, 5 TO 9, img
_SETALPHA s, d TO q, i
_BLEND
_BLEND img
_DONTBLEND
_DONTBLEND d
_PALETTECOLOR 3, _RGB32(10, 20, 30)
_PALETTECOLOR s, d, q
_COPYPALETTE
_COPYPALETTE img
_COPYPALETTE , img
_COPYPALETTE s, d
_AUTODISPLAY
_FREEIMAGE img
_FREEIMAGE
CALL _PUTIMAGE((10, 10), img)
CALL _SETALPHA(128, 5 TO 9)
CALL _CLEARCOLOR(_NONE)
CALL _BLEND
CALL _AUTODISPLAY
CALL _SAVEIMAGE("a.png")
CALL _COPYPALETTE
CALL _PALETTECOLOR(3, 4)
CALL _DONTBLEND(img)
CALL _MAPTRIANGLE((0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50))
