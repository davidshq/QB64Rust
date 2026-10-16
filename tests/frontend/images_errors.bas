' TEST: check-fail
$CONSOLE:ONLY
' Forms of the colour and image built-ins the old compiler rejects, one a line (m3-images-fonts-mouse, design D1;
' verification\v31_x02-x12, x27-x39, x49-x51, x78, x85-x100, x104-x106, x108, x109): each is an error here too,
' none a "not supported yet"
DIM l AS LONG, img AS LONG
l = _COPYIMAGE
l = _LOADIMAGE()
l = _LOADIMAGE
l = _LOADIMAGE(5)
l = _LOADIMAGE("a", "x")
l = _LOADIMAGE("a", 32, "", 1)
_PUTIMAGE (1, 2)-
_PUTIMAGE , , , , , 1
_PUTIMAGE (0, 0)-(9, 9), img _SMOOTH
_PUTIMAGE ("a", 2), img
_PUTIMAGE (1, 2, 3), img
PRINT _RGB(1, 2)
PRINT _RGB(1, 2, 3, 4, 5)
PRINT _RGBA(1, 2, 3)
PRINT _RED()
PRINT _RED
PRINT _RED(1, 2, 3)
PRINT _RED32(1, 2)
PRINT _HSB32(1, 2)
PRINT _HUE32("a")
PRINT _RGB("a", 2, 3)
PRINT _WIDTH("a")
PRINT _WIDTH(1, 2)
PRINT _PIXELSIZE()
_SETALPHA
_SETALPHA 128, TO 9
_SETALPHA 128 TO 9
PRINT _CLEARCOLOR(1, 2)
_SAVEIMAGE
_SAVEIMAGE 5
_SAVEIMAGE "a", 1, "b", 2
_PALETTECOLOR 3
PRINT _PALETTECOLOR
PRINT _PALETTECOLOR(1, 2, 3)
_COPYPALETTE 1, 2, 3
l = _COPYIMAGE(1, 2, 3)
_BLEND 1, 2
PRINT _BLEND(1, 2)
_DONTBLEND 1, 2
_AUTODISPLAY 1
PRINT _AUTODISPLAY(1)
PRINT PMAP(1)
PRINT PMAP
PRINT PMAP(1, 2, 3)
l = _NEWIMAGE(1)
_FREEIMAGE 1, 2
_SETALPHA "a"
_WIDTH 5
_RGB 1, 2, 3
