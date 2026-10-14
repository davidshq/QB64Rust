' _MAPTRIANGLE in its 2-D form (m3-images-fonts-mouse, task 2.4): a square drawn as two triangles, with and
' without _SEAMLESS, _CLOCKWISE and _ANTICLOCKWISE on triangles of each turn, _SMOOTH, _SMOOTHSHRUNK and
' _SMOOTHSTRETCHED, the source left out (the _SOURCE), into an image, and between images of 256 colours.
SCREEN _NEWIMAGE(200, 130, 32)
CLS , _RGB32(0, 0, 40)
t& = _NEWIMAGE(16, 16, 32)
_DEST t&
CLS , _RGB32(255, 0, 0)
LINE (8, 0)-(15, 7), _RGB32(0, 255, 0), BF
LINE (0, 8)-(7, 15), _RGB32(0, 0, 255), BF
LINE (8, 8)-(15, 15), _RGB32(255, 255, 0), BF
_DEST 0
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15), t& TO (0, 0)-(40, 0)-(0, 40)
_MAPTRIANGLE (15, 0)-(15, 15)-(0, 15), t& TO (40, 0)-(40, 40)-(0, 40)
_MAPTRIANGLE _SEAMLESS(0, 0)-(15, 0)-(0, 15), t& TO (50, 0)-(90, 0)-(50, 40)
_MAPTRIANGLE _SEAMLESS(15, 0)-(15, 15)-(0, 15), t& TO (90, 0)-(90, 40)-(50, 40)
_MAPTRIANGLE _CLOCKWISE (0, 0)-(15, 0)-(0, 15), t& TO (100, 0)-(140, 0)-(100, 40)
_MAPTRIANGLE _CLOCKWISE (0, 0)-(15, 0)-(0, 15), t& TO (100, 0)-(100, 40)-(140, 0)
_MAPTRIANGLE _ANTICLOCKWISE (0, 0)-(15, 0)-(0, 15), t& TO (150, 0)-(190, 0)-(150, 40)
_MAPTRIANGLE _ANTICLOCKWISE _SEAMLESS (0, 0)-(15, 0)-(0, 15), t& TO (190, 40)-(150, 40)-(190, 0)
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15), t& TO (0, 50)-(60, 50)-(0, 110), 0, _SMOOTH
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15), t& TO (70, 50)-(80, 50)-(70, 60), , _SMOOTHSHRUNK
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15), t& TO (90, 50)-(150, 50)-(90, 110), , _SMOOTHSTRETCHED
_SOURCE t&
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15) TO (160, 50)-(190, 50)-(160, 80)
_SOURCE 0
u& = _NEWIMAGE(20, 20, 32)
_MAPTRIANGLE (15, 15)-(0, 15)-(15, 0), t& TO (0, 0)-(19, 0)-(0, 19), u&
_PUTIMAGE (170, 90), u&
p& = _NEWIMAGE(16, 16, 256)
_DEST p&
CLS , 2
LINE (0, 0)-(7, 7), 14, BF
q& = _NEWIMAGE(24, 24, 256)
_DEST q&
CLS , 1
_MAPTRIANGLE (0, 0)-(15, 0)-(0, 15), p& TO (0, 0)-(23, 0)-(0, 23)
_DEST 0
_PUTIMAGE (130, 100), q&
SYSTEM
