' _SAVEIMAGE in each format libqb writes (BMP, PNG, JPEG, TGA, QOI, HDR), of an image and of the destination, with
' a format given and with none, and _LOADIMAGE of each file again; each put on the screen and some pixels printed;
' the files deleted (m3-images-fonts-mouse, task 2.4; verification\v31_c_save_load).
Dim f$(5)
f$(0) = "i03.bmp": f$(1) = "i03.png": f$(2) = "i03.jpg": f$(3) = "i03.tga": f$(4) = "i03.qoi": f$(5) = "i03.hdr"
Screen _NewImage(240, 320, 32)
Cls , _RGB32(0, 0, 80)
s& = _NewImage(24, 16, 32)
_Dest s&
Cls , _RGB32(10, 20, 30)
Line (0, 0)-(11, 7), _RGB32(255, 0, 0), BF
Line (12, 8)-(23, 15), _RGB32(0, 255, 0), BF
PSet (20, 2), _RGB32(255, 255, 255)
_Dest 0
For i = 0 To 5
    _SaveImage f$(i), s&
    h& = _LoadImage(f$(i), 32)
    _PutImage (i * 30, 0), h&
    _Source h&
    Locate 5 + i, 1: Print Right$(f$(i), 3); h& < -1; Hex$(Point(1, 1)); " "; Hex$(Point(20, 2))
    _Source 0
    _FreeImage h&
    Kill f$(i)
Next
' The destination saved with no handle; a format given after a gap; a name without a format.
_Dest s&
_SaveImage "i03d.png"
_Dest 0
_SaveImage "i03x.dat", s&, "bmp"
_SaveImage "i03y"
h& = _LoadImage("i03d.png", 32)
_PutImage (0, 20), h&
x& = _LoadImage("i03x.dat.bmp", 32)
_PutImage (30, 20), x&
Locate 12, 1: Print _FileExists("i03x.dat"); _FileExists("i03x.dat.bmp"); _FileExists("i03y.png"); _Width(x&)
Kill "i03d.png": Kill "i03x.dat.bmp": Kill "i03y.png"
' A palette image saved and loaded in 256 colours.
p& = _NewImage(16, 16, 256)
_Dest p&
Cls , 1
Line (0, 0)-(7, 7), 4, BF
_Dest 0
_SaveImage "i03p.png", p&
q& = _LoadImage("i03p.png", 256)
_Source q&
Print _PixelSize(q&); Point(1, 1); Point(10, 10)
_Source 0
_PutImage (60, 20), q&
Kill "i03p.png"
System
