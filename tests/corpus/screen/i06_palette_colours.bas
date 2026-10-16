' A palette in SCREEN 13 (m3-images-fonts-mouse, task 2.4): _PALETTECOLOR set and read, of the screen and of an
' image; _COPYPALETTE both ways, with each argument left out; the colour functions in a palette mode: _RGB and _RGBA
' give the nearest index, _RED, _GREEN, _BLUE and _ALPHA the parts of an index, with and without a handle.
SCREEN 13
_PALETTECOLOR 1, _RGB32(250, 100, 50)
_PALETTECOLOR 2, &HFF00FF00
p1~& = _PALETTECOLOR(1)
a& = _NEWIMAGE(32, 16, 256)
_COPYPALETTE , a&
pb~& = _PALETTECOLOR(1, a&)
_PALETTECOLOR 3, _RGB32(1, 2, 3), a&
pa~& = _PALETTECOLOR(3, a&)
_PALETTECOLOR 4, _RGB32(200, 200, 0), a&
_COPYPALETTE a&
p4~& = _PALETTECOLOR(4)
_PALETTECOLOR 5, _RGB32(0, 0, 252), a&
_DEST a&
_COPYPALETTE a&, 0
_DEST 0
p5~& = _PALETTECOLOR(5)
FOR c = 1 TO 5
    LINE (c * 20, 0)-(c * 20 + 15, 15), c, BF
NEXT
_DEST a&
CLS , 3
LINE (0, 0)-(15, 15), 4, BF
_DEST 0
_PUTIMAGE (130, 0), a&
LOCATE 4, 1
PRINT HEX$(p1~&); " "; HEX$(pb~&); " "; HEX$(pa~&)
PRINT HEX$(p4~&); " "; HEX$(p5~&)
r~& = _RGB(250, 100, 50)
r2~& = _RGB(0, 0, 250, a&)
ra~& = _RGBA(200, 200, 0, 128)
PRINT r~&; r2~&; ra~&; _RGBA(0, 255, 0, 255, a&)
PRINT _RED(1); _GREEN(1); _BLUE(1); _ALPHA(1)
PRINT _RED(3, a&); _GREEN(4, a&); _BLUE(5, a&); _ALPHA(3, a&)
SYSTEM
