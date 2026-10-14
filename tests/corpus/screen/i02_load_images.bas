' _LOADIMAGE of the files in i02_load_images.files (written by tools\screen\make_images.py): an 8-bit and a 24-bit
' BMP and a PNG with an alpha channel, in 32 bits, in 256 colours and in the screen's mode, and a file that is not
' there; each put on the screen, its size and some pixels printed (m3-images-fonts-mouse, task 2.4).
SCREEN _NEWIMAGE(240, 260, 32)
CLS , _RGB32(40, 40, 40)
a& = _LOADIMAGE("pattern8.bmp", 32)
b& = _LOADIMAGE("pattern24.bmp", 32)
c& = _LOADIMAGE("alpha.png", 32)
d& = _LOADIMAGE("pattern8.bmp", 256)
e& = _LOADIMAGE("pattern24.bmp")
f$ = "alpha.png"
g& = _LOADIMAGE(f$, , "")
m& = _LOADIMAGE("missing.png", 32)
_PUTIMAGE (0, 0)-(31, 23), a&
_PUTIMAGE (40, 0)-(71, 23), b&
_PUTIMAGE (80, 0)-(111, 23), c&
_PUTIMAGE (120, 0)-(151, 23), d&
_PUTIMAGE (160, 0)-(191, 23), e&
_PUTIMAGE (0, 28), g&
LOCATE 5, 1
PRINT a& < -1; b& < -1; c& < -1; d& < -1; e& < -1; g& < -1; m&
PRINT _WIDTH(a&); _HEIGHT(a&); _PIXELSIZE(d&); _PIXELSIZE(e&); _WIDTH(g&)
_SOURCE a&
PRINT HEX$(POINT(0, 0)); " "; HEX$(POINT(5, 3))
_SOURCE b&
PRINT HEX$(POINT(0, 0)); " "; HEX$(POINT(15, 11))
_SOURCE c&
PRINT HEX$(POINT(0, 0)); " "; HEX$(POINT(15, 11)); _ALPHA32(POINT(6, 0))
_SOURCE d&
PRINT POINT(3, 1); HEX$(_PALETTECOLOR(5, d&))
_SOURCE 0
SYSTEM
