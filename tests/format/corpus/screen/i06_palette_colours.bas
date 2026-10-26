' A palette in SCREEN 13 (m3-images-fonts-mouse, task 2.4): _PALETTECOLOR set and read, of the screen and of an
' image; _COPYPALETTE both ways, with each argument left out; the colour functions in a palette mode: _RGB and _RGBA
' give the nearest index, _RED, _GREEN, _BLUE and _ALPHA the parts of an index, with and without a handle.
Screen 13
_PaletteColor 1, _RGB32(250, 100, 50)
_PaletteColor 2, &HFF00FF00
p1~& = _PaletteColor(1)
a& = _NewImage(32, 16, 256)
_CopyPalette , a&
pb~& = _PaletteColor(1, a&)
_PaletteColor 3, _RGB32(1, 2, 3), a&
pa~& = _PaletteColor(3, a&)
_PaletteColor 4, _RGB32(200, 200, 0), a&
_CopyPalette a&
p4~& = _PaletteColor(4)
_PaletteColor 5, _RGB32(0, 0, 252), a&
_Dest a&
_CopyPalette a&, 0
_Dest 0
p5~& = _PaletteColor(5)
For c = 1 To 5
    Line (c * 20, 0)-(c * 20 + 15, 15), c, BF
Next
_Dest a&
Cls , 3
Line (0, 0)-(15, 15), 4, BF
_Dest 0
_PutImage (130, 0), a&
Locate 4, 1
Print Hex$(p1~&); " "; Hex$(pb~&); " "; Hex$(pa~&)
Print Hex$(p4~&); " "; Hex$(p5~&)
r~& = _RGB(250, 100, 50)
r2~& = _RGB(0, 0, 250, a&)
ra~& = _RGBA(200, 200, 0, 128)
Print r~&; r2~&; ra~&; _RGBA(0, 255, 0, 255, a&)
Print _Red(1); _Green(1); _Blue(1); _Alpha(1)
Print _Red(3, a&); _Green(4, a&); _Blue(5, a&); _Alpha(3, a&)
System
