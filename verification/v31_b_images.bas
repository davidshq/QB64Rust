$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): image handles, sizes, a freed handle, a file that is not there,
' the clear colour, a palette entry, blending, PMAP and _AUTODISPLAY, in a program with a hidden window. Each value
' is taken while the screen or an image is the destination and printed to the console.
DIM w(5) AS LONG, e AS LONG
SCREEN 12
a& = _NEWIMAGE(64, 48, 256)
b& = _COPYIMAGE(a&)
c& = _NEWIMAGE(64, 48, 32)
w(0) = _WIDTH: w(1) = _HEIGHT: w(2) = _PIXELSIZE
_DEST _CONSOLE
PRINT "handles:"; a& < -1; b& < -1; c& < -1; a& <> b&
PRINT "screen:"; w(0); w(1); w(2); "image:"; _WIDTH(a&); _HEIGHT(a&); _PIXELSIZE(a&)
PRINT "copy:"; _WIDTH(b&); _HEIGHT(b&); _PIXELSIZE(b&); "in 32 bits:"; _PIXELSIZE(c&)
PRINT "a new image:"; _WIDTH(_NEWIMAGE(64, 48, 32)); _RED32(&HFF102030)
t& = _NEWIMAGE(10, 10, 0)
PRINT "text image:"; _WIDTH(t&); _HEIGHT(t&); _PIXELSIZE(t&)
l = _LOADIMAGE("./foobar.bmp", 32): PRINT "not there:"; l
PRINT "HEX$:"; HEX$(_LOADIMAGE("none.bmp", 32)); " "; HEX$(_RGB(255, 0, 0, c&)); " "; HEX$(_RED32(&HFF102030))
_FREEIMAGE b&
ON ERROR GOTO handler
e = 0: x = _WIDTH(b&): PRINT "freed:"; e; x
e = 0: _FREEIMAGE b&: PRINT "freed twice:"; e
e = 0: x = _WIDTH(12345): PRINT "never made:"; e; x
e = 0: _FREEIMAGE 0: PRINT "free the screen:"; e
e = 0: x = _COPYIMAGE(a&, 32): PRINT "copied to 32 bits:"; e; x
e = 0: x = _COPYIMAGE(a&, 256): PRINT "copied to 256:"; e; x < -1
ON ERROR GOTO 0
_DEST 0
cc0& = _CLEARCOLOR
_CLEARCOLOR 4, a&
cc1& = _CLEARCOLOR(a&)
_CLEARCOLOR _NONE, a&
cc2& = _CLEARCOLOR(a&)
_CLEARCOLOR _RGB32(1, 2, 3), c&
cc3~& = _CLEARCOLOR(c&)
_DEST _CONSOLE
PRINT "clear colour:"; cc0&; cc1&; cc2&; HEX$(cc3~&)
_DEST 0
_PALETTECOLOR 3, _RGB32(10, 20, 30), a&
p3~& = _PALETTECOLOR(3, a&)
_PALETTECOLOR 3, _RGB32(10, 20, 30)
p4~& = _PALETTECOLOR(3)
_DEST _CONSOLE
PRINT "palette:"; HEX$(p3~&); " "; HEX$(p4~&); _PALETTECOLOR(3, a&) + 1
PRINT "blend:"; _BLEND(c&); _BLEND(a&);
_DONTBLEND c&
PRINT _BLEND(c&);
_BLEND c&
PRINT _BLEND(c&)
_DEST 0
WINDOW (0, 0)-(100, 100)
p0 = PMAP(50, 0): p1 = PMAP(50, 1): p2 = PMAP(320, 2): p5 = PMAP(240, 3)
d0 = _AUTODISPLAY
_DISPLAY
d1 = _AUTODISPLAY
_AUTODISPLAY
d2 = _AUTODISPLAY
_DEST _CONSOLE
PRINT "PMAP:"; p0; p1; p2; p5
PRINT "autodisplay:"; d0; d1; d2
SYSTEM
handler:
e = ERR
RESUME NEXT
