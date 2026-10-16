' Images made with _NEWIMAGE and _COPYIMAGE, put with _PUTIMAGE (plainly, scaled, mirrored each way, a part of one,
' after STEP, a palette image and its copy onto 32 bits, smoothed, into another image), their sizes, and _FREEIMAGE
' (m3-images-fonts-mouse, task 2.4).
SCREEN _NEWIMAGE(240, 200, 32)
CLS , _RGB32(0, 0, 64)
a& = _NEWIMAGE(16, 12, 32)
_DEST a&
CLS , _RGB32(200, 0, 0)
LINE (0, 0)-(7, 5), _RGB32(0, 200, 0), BF
PSET (15, 11), _RGB32(255, 255, 0)
_DEST 0
b& = _COPYIMAGE(a&)
p& = _NEWIMAGE(8, 8, 256)
_DEST p&
CLS , 4
LINE (0, 0)-(3, 3), 14, BF
_DEST 0
c& = _COPYIMAGE(p&)
_PUTIMAGE (0, 0), a&
_PUTIMAGE (20, 0)-(51, 23), b&
_PUTIMAGE (87, 0)-(56, 23), a&
_PUTIMAGE (90, 23)-(121, 0), a&
_PUTIMAGE (130, 0), a&, 0, (0, 0)-(7, 5)
_PUTIMAGE STEP(4, 0), c&
_PUTIMAGE (160, 0)-STEP(15, 15), p&
_PUTIMAGE (180, 0)-(211, 23), a&, , , _SMOOTH
_PUTIMAGE (0, 30)-(239, 59), a&, 0, (0, 0)-(3, 3)
_PUTIMAGE (2, 2), c&, b&
_PUTIMAGE (0, 64), b&
_PUTIMAGE (20, 64)-STEP(31, 11), a&, 0, STEP(0, 6)-STEP(15, 5)
d& = _NEWIMAGE(32, 24, 32)
_PUTIMAGE , a&, d&
_PUTIMAGE (60, 64), d&
_PUTIMAGE (100, 64), a&, , (8, 0)-(15, 11)
LOCATE 6, 1
PRINT _WIDTH; _HEIGHT; _PIXELSIZE; _WIDTH(a&); _HEIGHT(a&)
PRINT _PIXELSIZE(p&); _WIDTH(c&); _PIXELSIZE(c&); _HEIGHT(d&)
PRINT b& < -1; c& < -1; b& <> c&; _COPYIMAGE(0) < -1
_FREEIMAGE b&
_FREEIMAGE c&
e& = _NEWIMAGE(40, 20, 32)
PRINT e& < -1; _WIDTH(e&); _HEIGHT(_NEWIMAGE(7, 3, 256))
_DEST e&
PRINT _WIDTH; _PIXELSIZE;
_DEST 0
_FREEIMAGE e&
SYSTEM
