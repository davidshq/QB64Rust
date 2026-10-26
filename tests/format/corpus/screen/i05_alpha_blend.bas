' A clear colour, alpha and blending in 32 bits (m3-images-fonts-mouse, task 2.4): _CLEARCOLOR of an image and of
' the destination, set and taken off with _NONE; _SETALPHA of every pixel, of one colour and of a range, with and
' without a handle; _DONTBLEND and _BLEND of the screen and of an image; a clear colour in 256 colours; the
' values of _CLEARCOLOR, _BLEND and _ALPHA32 printed.
Screen _NewImage(200, 140, 32)
Cls , _RGB32(0, 0, 255)
a& = _NewImage(20, 20, 32)
_Dest a&
Cls , _RGB32(255, 0, 0)
Line (5, 5)-(14, 14), _RGB32(0, 255, 0), BF
_Dest 0
_ClearColor _RGB32(0, 255, 0), a&
_PutImage (0, 0), a&
cc~& = _ClearColor(a&)
_ClearColor _None, a&
cn& = _ClearColor(a&)
_PutImage (25, 0), a&
b& = _CopyImage(a&)
_SetAlpha 128, , b&
_PutImage (50, 0), b&
c& = _CopyImage(a&)
_SetAlpha 0, _RGB32(0, 255, 0), c&
_PutImage (75, 0), c&
d& = _CopyImage(a&)
_SetAlpha 64, _RGB32(200, 0, 0) To _RGB32(255, 0, 0), d&
_PutImage (100, 0), d&
_DontBlend
bl1& = _Blend
_PutImage (125, 0), b&
_Blend
bl2& = _Blend
_DontBlend b&
blb& = _Blend(b&)
_PutImage (150, 0), b&
_Blend b&
_Dest b&
_SetAlpha 255
_Dest 0
_PutImage (175, 0), b&
p& = _NewImage(20, 20, 256)
_Dest p&
Cls , 4
Line (5, 5)-(14, 14), 2, BF
_ClearColor 2
cp& = _ClearColor
_Dest 0
_PutImage (0, 25), p&
_ClearColor _RGB32(1, 2, 3)
cs~& = _ClearColor
_ClearColor _None
_Source d&
al1& = _Alpha32(Point(0, 0))
al2& = _Alpha32(Point(7, 7))
_Source c&
al3& = _Alpha32(Point(7, 7))
_Source 0
Locate 5, 1
Print Hex$(cc~&); cn&; cp&; " "; Hex$(cs~&)
Print bl1&; bl2&; blb&; _Blend; _Blend(a&)
Print al1&; al2&; al3&; _Alpha(_RGBA(1, 2, 3, 4)); _Alpha(4, p&)
System
