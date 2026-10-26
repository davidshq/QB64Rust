' Images made with _NEWIMAGE and _COPYIMAGE, put with _PUTIMAGE (plainly, scaled, mirrored each way, a part of one,
' after STEP, a palette image and its copy onto 32 bits, smoothed, into another image), their sizes, and _FREEIMAGE
' (m3-images-fonts-mouse, task 2.4).
Screen _NewImage(240, 200, 32)
Cls , _RGB32(0, 0, 64)
a& = _NewImage(16, 12, 32)
_Dest a&
Cls , _RGB32(200, 0, 0)
Line (0, 0)-(7, 5), _RGB32(0, 200, 0), BF
PSet (15, 11), _RGB32(255, 255, 0)
_Dest 0
b& = _CopyImage(a&)
p& = _NewImage(8, 8, 256)
_Dest p&
Cls , 4
Line (0, 0)-(3, 3), 14, BF
_Dest 0
c& = _CopyImage(p&)
_PutImage (0, 0), a&
_PutImage (20, 0)-(51, 23), b&
_PutImage (87, 0)-(56, 23), a&
_PutImage (90, 23)-(121, 0), a&
_PutImage (130, 0), a&, 0, (0, 0)-(7, 5)
_PutImage Step(4, 0), c&
_PutImage (160, 0)-Step(15, 15), p&
_PutImage (180, 0)-(211, 23), a&, , , _Smooth
_PutImage (0, 30)-(239, 59), a&, 0, (0, 0)-(3, 3)
_PutImage (2, 2), c&, b&
_PutImage (0, 64), b&
_PutImage (20, 64)-Step(31, 11), a&, 0, Step(0, 6)-Step(15, 5)
d& = _NewImage(32, 24, 32)
_PutImage , a&, d&
_PutImage (60, 64), d&
_PutImage (100, 64), a&, , (8, 0)-(15, 11)
Locate 6, 1
Print _Width; _Height; _PixelSize; _Width(a&); _Height(a&)
Print _PixelSize(p&); _Width(c&); _PixelSize(c&); _Height(d&)
Print b& < -1; c& < -1; b& <> c&; _CopyImage(0) < -1
_FreeImage b&
_FreeImage c&
e& = _NewImage(40, 20, 32)
Print e& < -1; _Width(e&); _Height(_NewImage(7, 3, 256))
_Dest e&
Print _Width; _PixelSize;
_Dest 0
_FreeImage e&
System
