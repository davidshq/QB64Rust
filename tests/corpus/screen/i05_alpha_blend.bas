' A clear colour, alpha and blending in 32 bits (m3-images-fonts-mouse, task 2.4): _CLEARCOLOR of an image and of
' the destination, set and taken off with _NONE; _SETALPHA of every pixel, of one colour and of a range, with and
' without a handle; _DONTBLEND and _BLEND of the screen and of an image; a clear colour in 256 colours; the
' values of _CLEARCOLOR, _BLEND and _ALPHA32 printed.
SCREEN _NEWIMAGE(200, 140, 32)
CLS , _RGB32(0, 0, 255)
a& = _NEWIMAGE(20, 20, 32)
_DEST a&
CLS , _RGB32(255, 0, 0)
LINE (5, 5)-(14, 14), _RGB32(0, 255, 0), BF
_DEST 0
_CLEARCOLOR _RGB32(0, 255, 0), a&
_PUTIMAGE (0, 0), a&
cc~& = _CLEARCOLOR(a&)
_CLEARCOLOR _NONE, a&
cn& = _CLEARCOLOR(a&)
_PUTIMAGE (25, 0), a&
b& = _COPYIMAGE(a&)
_SETALPHA 128, , b&
_PUTIMAGE (50, 0), b&
c& = _COPYIMAGE(a&)
_SETALPHA 0, _RGB32(0, 255, 0), c&
_PUTIMAGE (75, 0), c&
d& = _COPYIMAGE(a&)
_SETALPHA 64, _RGB32(200, 0, 0) TO _RGB32(255, 0, 0), d&
_PUTIMAGE (100, 0), d&
_DONTBLEND
bl1& = _BLEND
_PUTIMAGE (125, 0), b&
_BLEND
bl2& = _BLEND
_DONTBLEND b&
blb& = _BLEND(b&)
_PUTIMAGE (150, 0), b&
_BLEND b&
_DEST b&
_SETALPHA 255
_DEST 0
_PUTIMAGE (175, 0), b&
p& = _NEWIMAGE(20, 20, 256)
_DEST p&
CLS , 4
LINE (5, 5)-(14, 14), 2, BF
_CLEARCOLOR 2
cp& = _CLEARCOLOR
_DEST 0
_PUTIMAGE (0, 25), p&
_CLEARCOLOR _RGB32(1, 2, 3)
cs~& = _CLEARCOLOR
_CLEARCOLOR _NONE
_SOURCE d&
al1& = _ALPHA32(POINT(0, 0))
al2& = _ALPHA32(POINT(7, 7))
_SOURCE c&
al3& = _ALPHA32(POINT(7, 7))
_SOURCE 0
LOCATE 5, 1
PRINT HEX$(cc~&); cn&; cp&; " "; HEX$(cs~&)
PRINT bl1&; bl2&; blb&; _BLEND; _BLEND(a&)
PRINT al1&; al2&; al3&; _ALPHA(_RGBA(1, 2, 3, 4)); _ALPHA(4, p&)
SYSTEM
