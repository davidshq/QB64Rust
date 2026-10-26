' _MAPTRIANGLE in its 2-D form (m3-images-fonts-mouse, task 2.4): a square drawn as two triangles, with and
' without _SEAMLESS, _CLOCKWISE and _ANTICLOCKWISE on triangles of each turn, _SMOOTH, _SMOOTHSHRUNK and
' _SMOOTHSTRETCHED, the source left out (the _SOURCE), into an image, and between images of 256 colours.
Screen _NewImage(200, 130, 32)
Cls , _RGB32(0, 0, 40)
t& = _NewImage(16, 16, 32)
_Dest t&
Cls , _RGB32(255, 0, 0)
Line (8, 0)-(15, 7), _RGB32(0, 255, 0), BF
Line (0, 8)-(7, 15), _RGB32(0, 0, 255), BF
Line (8, 8)-(15, 15), _RGB32(255, 255, 0), BF
_Dest 0
_MapTriangle (0, 0)-(15, 0)-(0, 15), t& To(0, 0)-(40, 0)-(0, 40)
_MapTriangle (15, 0)-(15, 15)-(0, 15), t& To(40, 0)-(40, 40)-(0, 40)
_MapTriangle _Seamless(0, 0)-(15, 0)-(0, 15), t& To(50, 0)-(90, 0)-(50, 40)
_MapTriangle _Seamless(15, 0)-(15, 15)-(0, 15), t& To(90, 0)-(90, 40)-(50, 40)
_MapTriangle _Clockwise (0, 0)-(15, 0)-(0, 15), t& To(100, 0)-(140, 0)-(100, 40)
_MapTriangle _Clockwise (0, 0)-(15, 0)-(0, 15), t& To(100, 0)-(100, 40)-(140, 0)
_MapTriangle _AntiClockwise (0, 0)-(15, 0)-(0, 15), t& To(150, 0)-(190, 0)-(150, 40)
_MapTriangle _AntiClockwise _Seamless(0, 0)-(15, 0)-(0, 15), t& To(190, 40)-(150, 40)-(190, 0)
_MapTriangle (0, 0)-(15, 0)-(0, 15), t& To(0, 50)-(60, 50)-(0, 110), 0, _Smooth
_MapTriangle (0, 0)-(15, 0)-(0, 15), t& To(70, 50)-(80, 50)-(70, 60), , _SmoothShrunk
_MapTriangle (0, 0)-(15, 0)-(0, 15), t& To(90, 50)-(150, 50)-(90, 110), , _SmoothStretched
_Source t&
_MapTriangle (0, 0)-(15, 0)-(0, 15)To(160, 50)-(190, 50)-(160, 80)
_Source 0
u& = _NewImage(20, 20, 32)
_MapTriangle (15, 15)-(0, 15)-(15, 0), t& To(0, 0)-(19, 0)-(0, 19), u&
_PutImage (170, 90), u&
p& = _NewImage(16, 16, 256)
_Dest p&
Cls , 2
Line (0, 0)-(7, 7), 14, BF
q& = _NewImage(24, 24, 256)
_Dest q&
Cls , 1
_MapTriangle (0, 0)-(15, 0)-(0, 15), p& To(0, 0)-(23, 0)-(0, 23)
_Dest 0
_PutImage (130, 100), q&
System
' Compiled and run by no test (design D3; task 5.1), their calls compared by tests\callsite: the 3-D form of
' _MAPTRIANGLE (a z on each destination point, drawn into a hardware image), an image of the desktop and of the
' clipboard, a printed page, the depth buffer of hardware images.
_MapTriangle (0, 0)-(15, 0)-(0, 15), t& To(10, 10, -1)-(50, 10, -1)-(10, 50, -1)
_MapTriangle _Clockwise (0, 0)-(15, 0)-(0, 15), t& To(10, 10, -1)-(50, 10, -2)-(10, 50, -3), , _Smooth
h& = _ScreenImage
h& = _ScreenImage(0, 0, 9, 9)
h& = _ClipboardImage
_ClipboardImage = h&
_PrintImage h&
_DepthBuffer On , h&
_DepthBuffer _Clear
