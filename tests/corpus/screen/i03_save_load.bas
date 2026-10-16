' _SAVEIMAGE in each format libqb writes (BMP, PNG, JPEG, TGA, QOI, HDR), of an image and of the destination, with
' a format given and with none, and _LOADIMAGE of each file again; each put on the screen and some pixels printed;
' the files deleted (m3-images-fonts-mouse, task 2.4; verification\v31_c_save_load).
DIM f$(5)
f$(0) = "i03.bmp": f$(1) = "i03.png": f$(2) = "i03.jpg": f$(3) = "i03.tga": f$(4) = "i03.qoi": f$(5) = "i03.hdr"
SCREEN _NEWIMAGE(240, 320, 32)
CLS , _RGB32(0, 0, 80)
s& = _NEWIMAGE(24, 16, 32)
_DEST s&
CLS , _RGB32(10, 20, 30)
LINE (0, 0)-(11, 7), _RGB32(255, 0, 0), BF
LINE (12, 8)-(23, 15), _RGB32(0, 255, 0), BF
PSET (20, 2), _RGB32(255, 255, 255)
_DEST 0
FOR i = 0 TO 5
    _SAVEIMAGE f$(i), s&
    h& = _LOADIMAGE(f$(i), 32)
    _PUTIMAGE (i * 30, 0), h&
    _SOURCE h&
    LOCATE 5 + i, 1: PRINT RIGHT$(f$(i), 3); h& < -1; HEX$(POINT(1, 1)); " "; HEX$(POINT(20, 2))
    _SOURCE 0
    _FREEIMAGE h&
    KILL f$(i)
NEXT
' The destination saved with no handle; a format given after a gap; a name without a format.
_DEST s&
_SAVEIMAGE "i03d.png"
_DEST 0
_SAVEIMAGE "i03x.dat", s&, "bmp"
_SAVEIMAGE "i03y"
h& = _LOADIMAGE("i03d.png", 32)
_PUTIMAGE (0, 20), h&
x& = _LOADIMAGE("i03x.dat.bmp", 32)
_PUTIMAGE (30, 20), x&
LOCATE 12, 1: PRINT _FILEEXISTS("i03x.dat"); _FILEEXISTS("i03x.dat.bmp"); _FILEEXISTS("i03y.png"); _WIDTH(x&)
KILL "i03d.png": KILL "i03x.dat.bmp": KILL "i03y.png"
' A palette image saved and loaded in 256 colours.
p& = _NEWIMAGE(16, 16, 256)
_DEST p&
CLS , 1
LINE (0, 0)-(7, 7), 4, BF
_DEST 0
_SAVEIMAGE "i03p.png", p&
q& = _LOADIMAGE("i03p.png", 256)
_SOURCE q&
PRINT _PIXELSIZE(q&); POINT(1, 1); POINT(10, 10)
_SOURCE 0
_PUTIMAGE (60, 20), q&
KILL "i03p.png"
SYSTEM
